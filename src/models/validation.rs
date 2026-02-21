//! Validation models for the novel generation system

use serde::{Deserialize, Serialize};

/// Validation result containing errors and warnings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
}

impl ValidationResult {
    pub fn valid() -> Self {
        Self {
            valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn invalid(errors: Vec<ValidationError>) -> Self {
        Self {
            valid: false,
            errors,
            warnings: Vec::new(),
        }
    }

    pub fn with_warning(mut self, warning: ValidationWarning) -> Self {
        self.warnings.push(warning);
        self
    }
}

/// Validation error with field and message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
    pub code: ErrorCode,
}

/// Error codes for validation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    MissingField,
    InvalidFormat,
    OutOfRange,
    Duplicate,
    TooShort,
    TooLong,
}

/// Validation warning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationWarning {
    pub field: String,
    pub message: String,
}

/// Risk level for copyright checks
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

impl RiskLevel {
    pub fn is_high(&self) -> bool {
        matches!(self, RiskLevel::High)
    }
}

/// Copyright check result for character names
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyrightCheckResult {
    pub character_name: String,
    pub is_potential_duplicate: bool,
    pub risk_level: RiskLevel,
    pub suggested_alternatives: Vec<String>,
    pub source_work: Option<String>,
}

impl CopyrightCheckResult {
    pub fn safe(name: &str) -> Self {
        Self {
            character_name: name.to_string(),
            is_potential_duplicate: false,
            risk_level: RiskLevel::Low,
            suggested_alternatives: Vec::new(),
            source_work: None,
        }
    }

    pub fn risky(
        name: &str,
        risk_level: RiskLevel,
        source: Option<String>,
        alternatives: Vec<String>,
    ) -> Self {
        Self {
            character_name: name.to_string(),
            is_potential_duplicate: true,
            risk_level,
            suggested_alternatives: alternatives,
            source_work: source,
        }
    }
}

/// Consistency check result for outline validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistencyCheckResult {
    pub is_consistent: bool,
    pub score: f32,
    pub matched_keywords: Vec<String>,
    pub mismatched_elements: Vec<String>,
    pub warnings: Vec<String>,
}

impl ConsistencyCheckResult {
    pub fn consistent(score: f32, matched: Vec<String>) -> Self {
        Self {
            is_consistent: score >= 0.7,
            score,
            matched_keywords: matched,
            mismatched_elements: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn inconsistent(score: f32, mismatched: Vec<String>, warnings: Vec<String>) -> Self {
        Self {
            is_consistent: score >= 0.7,
            score,
            matched_keywords: Vec::new(),
            mismatched_elements: mismatched,
            warnings,
        }
    }
}

/// Combined validation result for outline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlineValidation {
    pub consistency: ConsistencyCheckResult,
    pub protagonist_check: CopyrightCheckResult,
    pub character_checks: Vec<CopyrightCheckResult>,
}

impl OutlineValidation {
    /// Check if outline passes all validations
    pub fn is_valid(&self) -> bool {
        self.consistency.is_consistent
            && self.protagonist_check.risk_level != RiskLevel::High
            && !self.character_checks.iter().any(|c| c.risk_level == RiskLevel::High)
    }

    /// Get all warnings
    pub fn warnings(&self) -> Vec<String> {
        let mut warnings = Vec::new();

        // Add consistency warnings
        warnings.extend(self.consistency.warnings.clone());
        warnings.extend(self.consistency.mismatched_elements.clone());

        // Add protagonist copyright warnings
        if self.protagonist_check.is_potential_duplicate {
            if let Some(source) = &self.protagonist_check.source_work {
                warnings.push(format!(
                    "主角名字 '{}' 可能与 '{}' 重复",
                    self.protagonist_check.character_name, source
                ));
            }
            if !self.protagonist_check.suggested_alternatives.is_empty() {
                warnings.push(format!(
                    "建议替换名字: {}",
                    self.protagonist_check.suggested_alternatives.join(", ")
                ));
            }
        }

        // Add character copyright warnings
        for check in &self.character_checks {
            if check.is_potential_duplicate {
                if let Some(source) = &check.source_work {
                    warnings.push(format!(
                        "角色 '{}' 可能与 '{}' 重复",
                        check.character_name, source
                    ));
                }
            }
        }

        warnings
    }
}
