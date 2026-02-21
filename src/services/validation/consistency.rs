//! Consistency validation service for outline and genre

use crate::models::{ConsistencyCheckResult, NovelGenre};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Genre keywords for consistency checking
pub struct ConsistencyChecker {
    genre_keywords: HashMap<String, Vec<String>>,
    genre_antonyms: HashMap<String, Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
struct GenreKeywords {
    keywords: HashMap<String, Vec<String>>,
    #[serde(default)]
    antonyms: HashMap<String, Vec<String>>,
}

impl ConsistencyChecker {
    /// Create a new consistency checker with default keywords
    pub fn new() -> Self {
        let mut checker = Self {
            genre_keywords: HashMap::new(),
            genre_antonyms: HashMap::new(),
        };
        checker.load_default_keywords();
        checker
    }

    /// Load keywords from JSON file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Self {
        let mut checker = Self {
            genre_keywords: HashMap::new(),
            genre_antonyms: HashMap::new(),
        };

        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(data) = serde_json::from_str::<GenreKeywords>(&content) {
                checker.genre_keywords = data.keywords;
                checker.genre_antonyms = data.antonyms;
            }
        }

        checker
    }

    /// Load default genre keywords
    fn load_default_keywords(&mut self) {
        // Fantasy keywords
        self.genre_keywords.insert(
            "fantasy".to_string(),
            vec![
                "魔法".to_string(),
                "修炼".to_string(),
                "灵石".to_string(),
                "宗门".to_string(),
                "斗气".to_string(),
                "灵魂".to_string(),
                "神器".to_string(),
                "天赋".to_string(),
                "异火".to_string(),
                "血脉".to_string(),
                "法则".to_string(),
                "次元".to_string(),
                "召唤".to_string(),
                "炼药师".to_string(),
            ],
        );

        // Xianxia keywords
        self.genre_keywords.insert(
            "xianxia".to_string(),
            vec![
                "修仙".to_string(),
                "灵气".to_string(),
                "筑基".to_string(),
                "金丹".to_string(),
                "元婴".to_string(),
                "飞升".to_string(),
                "渡劫".to_string(),
                "灵根".to_string(),
                "功法".to_string(),
                "丹药".to_string(),
                "仙剑".to_string(),
                "洞府".to_string(),
                "仙宗".to_string(),
                "妖修".to_string(),
                "魔道".to_string(),
            ],
        );

        // Urban keywords
        self.genre_keywords.insert(
            "urban".to_string(),
            vec![
                "都市".to_string(),
                "总裁".to_string(),
                "豪门".to_string(),
                "创业".to_string(),
                "商业".to_string(),
                "公司".to_string(),
                "投资".to_string(),
                "股票".to_string(),
                "房地产".to_string(),
                "互联网".to_string(),
                "黑客".to_string(),
                "特工".to_string(),
                "医生".to_string(),
                "律师".to_string(),
                "明星".to_string(),
            ],
        );

        // Historical keywords
        self.genre_keywords.insert(
            "historical".to_string(),
            vec![
                "古代".to_string(),
                "朝廷".to_string(),
                "皇帝".to_string(),
                "太子".to_string(),
                "王爷".to_string(),
                "大臣".to_string(),
                "科举".to_string(),
                "江湖".to_string(),
                "武林".to_string(),
                "侠客".to_string(),
                "镖局".to_string(),
                "青楼".to_string(),
                "客栈".to_string(),
                "商铺".to_string(),
                "银两".to_string(),
            ],
        );

        // Romance keywords
        self.genre_keywords.insert(
            "romance".to_string(),
            vec![
                "爱情".to_string(),
                "甜蜜".to_string(),
                "宠溺".to_string(),
                "误会".to_string(),
                "表白".to_string(),
                "约会".to_string(),
                "婚礼".to_string(),
                "前任".to_string(),
                "出轨".to_string(),
                "豪门".to_string(),
                "契约".to_string(),
                "暗恋".to_string(),
                "相亲".to_string(),
                "闪婚".to_string(),
                "甜宠".to_string(),
            ],
        );

        // SciFi keywords
        self.genre_keywords.insert(
            "scifi".to_string(),
            vec![
                "太空".to_string(),
                "飞船".to_string(),
                "星球".to_string(),
                "外星人".to_string(),
                "人工智能".to_string(),
                "机器人".to_string(),
                "基因".to_string(),
                "克隆".to_string(),
                "未来".to_string(),
                "末世".to_string(),
                "机甲".to_string(),
                "星舰".to_string(),
                "维度".to_string(),
                "宇宙".to_string(),
                "时间旅行".to_string(),
            ],
        );

        // Game keywords
        self.genre_keywords.insert(
            "game".to_string(),
            vec![
                "游戏".to_string(),
                "电竞".to_string(),
                "玩家".to_string(),
                "副本".to_string(),
                "装备".to_string(),
                "技能".to_string(),
                "升级".to_string(),
                "公会".to_string(),
                "排位".to_string(),
                "主播".to_string(),
                "代练".to_string(),
                "全服".to_string(),
                "首杀".to_string(),
                "神装".to_string(),
                "满级".to_string(),
            ],
        );

        // Horror keywords
        self.genre_keywords.insert(
            "horror".to_string(),
            vec![
                "恐怖".to_string(),
                "鬼魂".to_string(),
                "僵尸".to_string(),
                "吸血鬼".to_string(),
                "狼人".to_string(),
                "灵异".to_string(),
                "惊悚".to_string(),
                "墓地".to_string(),
                "凶宅".to_string(),
                "诅咒".to_string(),
                "附身".to_string(),
                "阴阳眼".to_string(),
                "捉鬼".to_string(),
                "茅山".to_string(),
                "盗墓".to_string(),
            ],
        );

        // Set up antonyms (keywords that shouldn't appear in certain genres)
        self.genre_antonyms.insert(
            "xianxia".to_string(),
            vec!["电脑".to_string(), "手机".to_string(), "互联网".to_string(), "汽车".to_string()],
        );
        self.genre_antonyms.insert(
            "historical".to_string(),
            vec!["电脑".to_string(), "手机".to_string(), "互联网".to_string(), "WiFi".to_string()],
        );
    }

    /// Check consistency between outline content and expected genre
    pub fn check(&self, genre: &str, outline_content: &str, premise: &str) -> ConsistencyCheckResult {
        let genre_lower = genre.to_lowercase();
        let keywords = self.genre_keywords.get(&genre_lower);
        let antonyms = self.genre_antonyms.get(&genre_lower);

        let mut matched_keywords = Vec::new();
        let mut mismatched_elements = Vec::new();
        let mut warnings = Vec::new();

        // Combine outline and premise for checking
        let combined_text = format!("{} {}", outline_content, premise);

        // Check for matching keywords
        if let Some(kw) = keywords {
            for keyword in kw {
                if combined_text.contains(keyword) {
                    matched_keywords.push(keyword.clone());
                }
            }
        }

        // Check for antonyms (shouldn't appear in certain genres)
        if let Some(ant) = antonyms {
            for antonym in ant {
                if combined_text.contains(antonym) {
                    mismatched_elements.push(format!(
                        "Genre '{}' should not contain '{}'",
                        genre, antonym
                    ));
                }
            }
        }

        // Calculate consistency score
        let keyword_count = matched_keywords.len() as f32;
        let antonym_penalty = mismatched_elements.len() as f32 * 0.3;
        let score = (keyword_count / 10.0).min(1.0) - antonym_penalty;
        let score = score.max(0.0).min(1.0);

        // Generate warnings
        if matched_keywords.is_empty() {
            warnings.push(format!(
                "No genre-specific keywords found. The outline may not match the genre '{}'.",
                genre
            ));
        } else if matched_keywords.len() < 3 {
            warnings.push(format!(
                "Only {} genre-specific keywords found. Consider adding more genre elements.",
                matched_keywords.len()
            ));
        }

        let is_consistent = score >= 0.3 && mismatched_elements.len() < 2;

        ConsistencyCheckResult {
            is_consistent,
            score,
            matched_keywords,
            mismatched_elements,
            warnings,
        }
    }

    /// Get keywords for a specific genre
    pub fn get_keywords(&self, genre: &str) -> Vec<String> {
        self.genre_keywords
            .get(&genre.to_lowercase())
            .cloned()
            .unwrap_or_default()
    }
}

impl Default for ConsistencyChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert string genre to NovelGenre enum
pub fn parse_genre(genre: &str) -> Option<NovelGenre> {
    match genre.to_lowercase().as_str() {
        "fantasy" => Some(NovelGenre::Fantasy),
        "urban" => Some(NovelGenre::Urban),
        "xianxia" => Some(NovelGenre::Xianxia),
        "historical" => Some(NovelGenre::Historical),
        "romance" => Some(NovelGenre::Romance),
        "scifi" => Some(NovelGenre::Scifi),
        "game" => Some(NovelGenre::Game),
        "horror" => Some(NovelGenre::Horror),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fantasy_consistency() {
        let checker = ConsistencyChecker::new();
        let result = checker.check(
            "fantasy",
            "主角拥有变异斗气，开始修炼之路，学习灵气功法成为炼药师",
            "穿越到异世界成为修炼者",
        );
        assert!(result.is_consistent);
        assert!(result.score > 0.0);
    }

    #[test]
    fn test_xianxia_with_modern_terms() {
        let checker = ConsistencyChecker::new();
        let result = checker.check(
            "xianxia",
            "主角用手机修仙",
            "现代都市修仙",
        );
        // Should have mismatched elements (phone in xianxia)
        assert!(!result.mismatched_elements.is_empty() || result.score < 0.5);
    }

    #[test]
    fn test_urban_consistency() {
        let checker = ConsistencyChecker::new();
        let result = checker.check(
            "urban",
            "创业成为公司总裁",
            "都市创业故事",
        );
        assert!(result.is_consistent);
    }
}
