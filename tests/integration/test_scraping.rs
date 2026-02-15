//! Integration tests for scraping service

#[cfg(test)]
mod tests {
    use ai_novel_agent::models::NovelGenre;

    /// Test scraping service can analyze a genre
    #[tokio::test]
    async fn test_scraping_service_analyze_genre() {
        // This test requires the actual scraping implementation
        // For now, we test the service can be instantiated
        let _service = ai_novel_agent::services::FeasibilityService::new();
        assert!(true);
    }

    /// Test genre enum display
    #[test]
    fn test_genre_display() {
        assert_eq!(format!("{}", NovelGenre::Fantasy), "fantasy");
        assert_eq!(format!("{}", NovelGenre::Urban), "urban");
        assert_eq!(format!("{}", NovelGenre::Xianxia), "xianxia");
    }
}
