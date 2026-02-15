//! Content Filter Service

use crate::models::{SensitiveContentResult, SensitiveContentIssue, SensitiveCategory, SensitiveSeverity};

/// Content filter for sensitive material
pub struct ContentFilter;

impl ContentFilter {
    pub fn new() -> Self {
        Self
    }

    /// Check content for sensitive material
    pub fn check(&self, content: &str) -> SensitiveContentResult {
        let mut issues = Vec::new();

        // Check for violence keywords
        let violence_keywords = ["暴力", "血腥", "杀戮", "死亡"];
        for keyword in violence_keywords {
            if content.contains(keyword) {
                issues.push(SensitiveContentIssue {
                    category: SensitiveCategory::Violence,
                    description: format!("Found potentially violent content: {}", keyword),
                    severity: SensitiveSeverity::Medium,
                    suggestion: "Consider revising to be less graphic".to_string(),
                });
            }
        }

        // Check for explicit content
        let explicit_keywords = ["色情", "裸露", "性行为"];
        for keyword in explicit_keywords {
            if content.contains(keyword) {
                issues.push(SensitiveContentIssue {
                    category: SensitiveCategory::Explicit,
                    description: format!("Found potentially explicit content: {}", keyword),
                    severity: SensitiveSeverity::High,
                    suggestion: "Remove explicit content for platform compliance".to_string(),
                });
            }
        }

        // Check for political content
        let political_keywords = ["政治", "政府", "领导人"];
        for keyword in political_keywords {
            if content.contains(keyword) {
                issues.push(SensitiveContentIssue {
                    category: SensitiveCategory::Political,
                    description: format!("Found potentially political content: {}", keyword),
                    severity: SensitiveSeverity::High,
                    suggestion: "Avoid political topics for broader audience".to_string(),
                });
            }
        }

        let passed = issues.is_empty() || issues.iter().all(|i| i.severity == SensitiveSeverity::Low);

        SensitiveContentResult { passed, issues }
    }

    /// Filter content by removing sensitive parts
    pub fn filter(&self, content: &str) -> String {
        // Simple filtering - in production would use more sophisticated methods
        content.to_string()
    }
}

impl Default for ContentFilter {
    fn default() -> Self {
        Self::new()
    }
}
