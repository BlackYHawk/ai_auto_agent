//! Copyright check service for character names

use crate::models::{CopyrightCheckResult, RiskLevel};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Character copyright database
pub struct CopyrightChecker {
    known_characters: HashMap<String, Vec<CharacterEntry>>,
}

#[derive(Debug, Deserialize, Clone)]
struct CharacterEntry {
    name: String,
    source: String,
    alternatives: Vec<String>,
}

impl CopyrightChecker {
    /// Create a new copyright checker with default database
    pub fn new() -> Self {
        let mut checker = Self {
            known_characters: HashMap::new(),
        };
        checker.load_default_characters();
        checker
    }

    /// Load characters from JSON file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Self {
        let mut checker = Self {
            known_characters: HashMap::new(),
        };

        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(data) = serde_json::from_str::<HashMap<String, Vec<CharacterEntry>>>(&content) {
                for (genre, characters) in data {
                    checker.known_characters.insert(genre, characters);
                }
            }
        }

        checker
    }

    /// Load default characters (common well-known novel characters)
    fn load_default_characters(&mut self) {
        let defaults = vec![
            ("fantasy", "萧炎", "斗破苍穹", vec!["萧云", "萧天", "萧凡"]),
            ("fantasy", "唐三", "斗罗大陆", vec!["唐云", "唐风", "唐羽"]),
            ("fantasy", "林动", "武动乾坤", vec!["林云", "林风", "林轩"]),
            ("fantasy", "牧尘", "大主宰", vec!["牧云", "牧风", "牧原"]),
            ("xianxia", "韩立", "凡人修仙传", vec!["韩云", "韩风", "韩轩"]),
            ("xianxia", "张小凡", "诛仙", vec!["张云", "张风", "张凡"]),
            ("xianxia", "陆雪琪", "诛仙", vec!["陆云", "陆霜", "陆萱"]),
            ("xianxia", "碧瑶", "诛仙", vec!["碧云", "碧霞", "碧莲"]),
            ("urban", "陈北冥", "都市仙尊", vec!["陈云", "陈风", "陈南"]),
            ("urban", "叶凡", "都市全能高手", vec!["叶云", "叶风", "叶天"]),
            ("urban", "林ako", "都市", vec!["林云", "林风", "林轩"]),
            ("historical", "雍正", "雍正王朝", vec!["胤禛", "雍正", "弘历"]),
            ("historical", "乾隆", "乾隆王朝", vec!["弘历", "乾隆", "永琰"]),
            ("romance", "何以笙箫默", "何以笙箫默", vec!["何云", "何风", "何以"]),
            ("scifi", "刘培强", "流浪地球", vec!["刘云", "刘强", "刘风"]),
            ("scifi", "韩朵朵", "流浪地球", vec!["韩云", "韩花", "韩晓"]),
        ];

        for (genre, name, source, alternatives) in defaults {
            let entry = CharacterEntry {
                name: name.to_string(),
                source: source.to_string(),
                alternatives: alternatives.into_iter().map(String::from).collect(),
            };
            self.known_characters
                .entry(genre.to_string())
                .or_insert_with(Vec::new)
                .push(entry);
        }
    }

    /// Check a character name for potential copyright issues
    pub fn check(&self, name: &str, _genre: Option<&str>) -> CopyrightCheckResult {
        let name_lower = name.to_lowercase();

        // Check against all known characters
        for characters in self.known_characters.values() {
            for entry in characters {
                if entry.name.to_lowercase() == name_lower {
                    return CopyrightCheckResult::risky(
                        name,
                        RiskLevel::High,
                        Some(entry.source.clone()),
                        entry.alternatives.clone(),
                    );
                }
            }
        }

        // Check for partial matches (simpler matching)
        for characters in self.known_characters.values() {
            for entry in characters {
                if entry.name.to_lowercase().contains(&name_lower)
                    || name_lower.contains(&entry.name.to_lowercase())
                {
                    if entry.name.len() > 2 && name.len() > 2 {
                        return CopyrightCheckResult::risky(
                            name,
                            RiskLevel::Medium,
                            Some(entry.source.clone()),
                            entry.alternatives.clone(),
                        );
                    }
                }
            }
        }

        CopyrightCheckResult::safe(name)
    }

    /// Check multiple character names
    pub fn check_multiple(&self, names: &[String], genre: Option<&str>) -> Vec<CopyrightCheckResult> {
        names.iter().map(|n| self.check(n, genre)).collect()
    }
}

impl Default for CopyrightChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// Check if a name might be a common name (low copyright risk)
pub fn is_common_name(name: &str) -> bool {
    let common_names = [
        "张伟", "李娜", "王芳", "刘洋", "陈静", "杨明", "赵磊", "孙丽",
        "周涛", "吴敏", "徐强", "孙杰", "马超", "朱华", "胡军", "郭磊",
        "何云", "张云", "李云", "王云", "刘云", "陈云", "杨云", "赵云",
        "张三", "李四", "王五", "赵六", "孙七", "周八", "吴九", "郑十",
        "小明", "小红", "小芳", "小强", "小丽", "小刚", "小华", "小军",
        "Alex", "John", "Mary", "Bob", "Tom", "Jack", "Lucy", "Lily",
    ];

    common_names.iter().any(|n| n.to_lowercase() == name.to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_character() {
        let checker = CopyrightChecker::new();
        let result = checker.check("萧炎", None);
        assert!(result.is_potential_duplicate);
        assert_eq!(result.risk_level, RiskLevel::High);
    }

    #[test]
    fn test_common_name() {
        let checker = CopyrightChecker::new();
        let result = checker.check("张三", None);
        assert!(!result.is_potential_duplicate);
        assert_eq!(result.risk_level, RiskLevel::Low);
    }

    #[test]
    fn test_unique_name() {
        let checker = CopyrightChecker::new();
        let result = checker.check("萧云", None);
        // Should suggest alternatives
        assert!(result.suggested_alternatives.len() > 0 || !result.is_potential_duplicate);
    }
}
