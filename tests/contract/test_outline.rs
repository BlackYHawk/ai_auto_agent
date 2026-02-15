//! Contract tests for outline generation

#[cfg(test)]
mod tests {
    use ai_novel_agent::models::{NovelOutline, OutlineStatus, WorldType};

    /// Test outline creation
    #[test]
    fn test_create_outline() {
        let outline = NovelOutline::new(
            uuid::Uuid::new_v4(),
            "一个少年踏上修仙之路".to_string(),
            "坚持与成长".to_string(),
            1_000_000,
        );

        assert_eq!(outline.premise, "一个少年踏上修仙之路");
        assert_eq!(outline.theme, "坚持与成长");
        assert_eq!(outline.status, OutlineStatus::Draft);
    }

    /// Test outline approval
    #[test]
    fn test_outline_approval() {
        let mut outline = NovelOutline::new(
            uuid::Uuid::new_v4(),
            "test premise".to_string(),
            "test theme".to_string(),
            500_000,
        );

        outline.approve();
        assert_eq!(outline.status, OutlineStatus::Approved);

        outline.lock();
        assert_eq!(outline.status, OutlineStatus::Locked);
    }

    /// Test world settings
    #[test]
    fn test_world_settings() {
        let outline = NovelOutline::new(
            uuid::Uuid::new_v4(),
            "test".to_string(),
            "test".to_string(),
            1_000_000,
        );

        assert_eq!(outline.world_settings.world_type, WorldType::Modern);
    }
}
