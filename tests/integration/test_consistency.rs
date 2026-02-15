//! Integration tests for consistency checking

#[cfg(test)]
mod tests {
    use ai_novel_agent::services::ConsistencyChecker;

    /// Test consistency checker can be instantiated
    #[tokio::test]
    async fn test_consistency_checker() {
        let checker = ConsistencyChecker::new();
        let result = checker.check_consistency("test-project").await;
        assert!(result.is_ok());
    }

    /// Test consistency result structure
    #[test]
    fn test_consistency_result() {
        use ai_novel_agent::services::consistency::ConsistencyCheckResult;

        let result = ConsistencyCheckResult {
            passed: true,
            issues: vec![],
        };

        assert!(result.passed);
        assert!(result.issues.is_empty());
    }
}
