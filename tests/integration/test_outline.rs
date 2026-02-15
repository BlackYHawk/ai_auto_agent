//! Integration tests for outline service

#[cfg(test)]
mod tests {
    use ai_novel_agent::models::NovelGenre;

    /// Test outline service can generate an outline
    #[tokio::test]
    async fn test_outline_service_generate() {
        let service = ai_novel_agent::services::OutlineService::new();
        let project_id = uuid::Uuid::new_v4();

        let outline = service.generate(
            project_id,
            NovelGenre::Fantasy,
            "一个少年发现自己是天选之子".to_string(),
            "命运与抗争".to_string(),
            1_000_000,
        ).await;

        assert!(outline.is_ok());
    }
}
