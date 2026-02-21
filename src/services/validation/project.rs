//! Project validation service

use crate::models::{ErrorCode, ValidationError, ValidationResult, ValidationWarning};

/// Validation rules for project fields
pub struct ProjectValidator;

impl ProjectValidator {
    /// Validate all project creation fields
    pub fn validate(name: &str, summary: &str, genre: &str, target: u64) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Validate name
        if name.trim().is_empty() {
            errors.push(ValidationError {
                field: "name".to_string(),
                message: "Name is required".to_string(),
                code: ErrorCode::MissingField,
            });
        } else if name.len() > 100 {
            errors.push(ValidationError {
                field: "name".to_string(),
                message: "Name must be 100 characters or less".to_string(),
                code: ErrorCode::TooLong,
            });
        }

        // Validate summary
        if summary.trim().is_empty() {
            errors.push(ValidationError {
                field: "summary".to_string(),
                message: "Summary is required".to_string(),
                code: ErrorCode::MissingField,
            });
        } else if summary.len() < 10 {
            errors.push(ValidationError {
                field: "summary".to_string(),
                message: "Summary must be at least 10 characters".to_string(),
                code: ErrorCode::TooShort,
            });
        } else if summary.len() > 2000 {
            errors.push(ValidationError {
                field: "summary".to_string(),
                message: "Summary must be 2000 characters or less".to_string(),
                code: ErrorCode::TooLong,
            });
        }

        // Validate genre
        let valid_genres = ["fantasy", "urban", "xianxia", "historical", "romance", "scifi", "game", "horror"];
        if genre.trim().is_empty() {
            errors.push(ValidationError {
                field: "genre".to_string(),
                message: "Genre is required".to_string(),
                code: ErrorCode::MissingField,
            });
        } else if !valid_genres.contains(&genre.to_lowercase().as_str()) {
            errors.push(ValidationError {
                field: "genre".to_string(),
                message: format!("Invalid genre. Supported: {}", valid_genres.join(", ")),
                code: ErrorCode::InvalidFormat,
            });
        }

        // Validate target word count
        if target == 0 {
            errors.push(ValidationError {
                field: "target_word_count".to_string(),
                message: "Target word count must be greater than 0".to_string(),
                code: ErrorCode::OutOfRange,
            });
        } else if target > 10_000_000 {
            errors.push(ValidationError {
                field: "target_word_count".to_string(),
                message: "Target word count must be 10,000,000 or less".to_string(),
                code: ErrorCode::OutOfRange,
            });
        }

        // Add warnings for unusual values
        if target < 10_000 {
            warnings.push(ValidationWarning {
                field: "target_word_count".to_string(),
                message: "Target word count is very low. Consider at least 30,000 words for a novel.".to_string(),
            });
        } else if target > 5_000_000 {
            warnings.push(ValidationWarning {
                field: "target_word_count".to_string(),
                message: "Very large target. This may take significant time to generate.".to_string(),
            });
        }

        let valid = errors.is_empty();
        ValidationResult {
            valid,
            errors,
            warnings,
        }
    }

    /// Validate just the target word count
    pub fn validate_target(target: u64) -> ValidationResult {
        let mut errors = Vec::new();

        if target == 0 {
            errors.push(ValidationError {
                field: "target_word_count".to_string(),
                message: "Target word count must be greater than 0".to_string(),
                code: ErrorCode::OutOfRange,
            });
        }

        if errors.is_empty() {
            ValidationResult::valid()
        } else {
            ValidationResult::invalid(errors)
        }
    }

    /// Check if a genre is valid
    pub fn is_valid_genre(genre: &str) -> bool {
        let valid_genres = ["fantasy", "urban", "xianxia", "historical", "romance", "scifi", "game", "horror"];
        valid_genres.contains(&genre.to_lowercase().as_str())
    }

    /// Get list of valid genres
    pub fn valid_genres() -> Vec<&'static str> {
        vec!["fantasy", "urban", "xianxia", "historical", "romance", "scifi", "game", "horror"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_project() {
        let result = ProjectValidator::validate(
            "My Novel",
            "This is a summary of my novel",
            "fantasy",
            100000,
        );
        assert!(result.valid);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_missing_name() {
        let result = ProjectValidator::validate(
            "",
            "This is a summary",
            "fantasy",
            100000,
        );
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| e.field == "name"));
    }

    #[test]
    fn test_missing_summary() {
        let result = ProjectValidator::validate(
            "My Novel",
            "",
            "fantasy",
            100000,
        );
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| e.field == "summary"));
    }

    #[test]
    fn test_invalid_genre() {
        let result = ProjectValidator::validate(
            "My Novel",
            "This is a summary",
            "invalid",
            100000,
        );
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| e.field == "genre"));
    }

    #[test]
    fn test_zero_target() {
        let result = ProjectValidator::validate(
            "My Novel",
            "This is a summary",
            "fantasy",
            0,
        );
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| e.field == "target_word_count"));
    }

    #[test]
    fn test_valid_genres() {
        for genre in ProjectValidator::valid_genres() {
            assert!(ProjectValidator::is_valid_genre(genre));
        }
        assert!(!ProjectValidator::is_valid_genre("invalid"));
    }
}
