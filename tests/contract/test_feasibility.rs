//! Contract tests for feasibility analysis

#[cfg(test)]
mod tests {
    use ai_novel_agent::models::{NovelGenre, FeasibilityReport};

    /// Test that feasibility report can be created with genre
    #[test]
    fn test_create_feasibility_report() {
        let report = FeasibilityReport::new(
            uuid::Uuid::new_v4(),
            NovelGenre::Fantasy,
        );

        assert_eq!(report.genre, NovelGenre::Fantasy);
        assert_eq!(report.recommendation, ai_novel_agent::models::Recommendation::Revise);
    }

    /// Test recommendation calculation
    #[test]
    fn test_calculate_recommendation() {
        let mut report = FeasibilityReport::new(
            uuid::Uuid::new_v4(),
            NovelGenre::Urban,
        );

        // Test high score
        report.scores.market_viability = 80;
        report.calculate_recommendation();
        assert_eq!(report.recommendation, ai_novel_agent::models::Recommendation::Proceed);

        // Test medium score
        let mut report2 = FeasibilityReport::new(
            uuid::Uuid::new_v4(),
            NovelGenre::Urban,
        );
        report2.scores.market_viability = 60;
        report2.calculate_recommendation();
        assert_eq!(report2.recommendation, ai_novel_agent::models::Recommendation::Revise);

        // Test low score
        let mut report3 = FeasibilityReport::new(
            uuid::Uuid::new_v4(),
            NovelGenre::Urban,
        );
        report3.scores.market_viability = 40;
        report3.calculate_recommendation();
        assert_eq!(report3.recommendation, ai_novel_agent::models::Recommendation::Reject);
    }
}
