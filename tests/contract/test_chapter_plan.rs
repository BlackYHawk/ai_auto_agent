//! Contract tests for chapter planning

#[cfg(test)]
mod tests {
    use ai_novel_agent::models::{ChapterPlan, ChapterSummary};

    /// Test chapter plan creation
    #[test]
    fn test_create_chapter_plan() {
        let plan = ChapterPlan::new(uuid::Uuid::new_v4(), 100);

        assert_eq!(plan.total_chapters, 100);
        assert!(plan.chapters.is_empty());
    }

    /// Test plot twist positions (every 10 chapters)
    #[test]
    fn test_plot_twist_positions() {
        let plan = ChapterPlan::new(uuid::Uuid::new_v4(), 50);

        assert_eq!(plan.plot_twist_positions, vec![10, 20, 30, 40, 50]);
    }

    /// Test chapter summary
    #[test]
    fn test_chapter_summary() {
        let summary = ChapterSummary {
            number: 10,
            title: "第十章".to_string(),
            summary: "重要章节".to_string(),
            key_events: vec!["事件1".to_string()],
            protagonist_development: "主角成长".to_string(),
            word_count_estimate: 10000,
            is_plot_twist_chapter: true,
            plot_twist_description: Some("重大转折".to_string()),
        };

        assert!(summary.is_plot_twist_chapter);
        assert_eq!(summary.plot_twist_description, Some("重大转折".to_string()));
    }
}
