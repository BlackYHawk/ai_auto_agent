//! Unit tests for models

#[cfg(test)]
mod tests {
    use ai_novel_agent::models::*;

    #[test]
    fn test_novel_genre_display() {
        assert_eq!(format!("{}", NovelGenre::Fantasy), "fantasy");
        assert_eq!(format!("{}", NovelGenre::Urban), "urban");
    }

    #[test]
    fn test_project_status_transitions() {
        let mut project = NovelProject::new(
            "Test".to_string(),
            NovelGenre::Fantasy,
            1_000_000,
        );

        assert_eq!(project.status, ProjectStatus::Draft);

        project.set_status(ProjectStatus::Feasibility);
        assert_eq!(project.status, ProjectStatus::Feasibility);

        project.set_status(ProjectStatus::Outline);
        assert_eq!(project.status, ProjectStatus::Outline);
    }

    #[test]
    fn test_chapter_plan_creation() {
        let plan = ChapterPlan::new(uuid::Uuid::new_v4(), 100);
        assert_eq!(plan.total_chapters, 100);
        assert_eq!(plan.plot_twist_positions.len(), 10);
    }

    #[test]
    fn test_outline_approval() {
        let mut outline = NovelOutline::new(
            uuid::Uuid::new_v4(),
            "test".to_string(),
            "test".to_string(),
            500_000,
        );

        outline.approve();
        assert_eq!(outline.status, OutlineStatus::Approved);

        outline.lock();
        assert_eq!(outline.status, OutlineStatus::Locked);
    }

    #[test]
    fn test_feasibility_recommendation() {
        let mut report = FeasibilityReport::new(
            uuid::Uuid::new_v4(),
            NovelGenre::Fantasy,
        );

        report.scores.market_viability = 80;
        report.calculate_recommendation();
        assert_eq!(report.recommendation, Recommendation::Proceed);

        report.scores.market_viability = 60;
        report.calculate_recommendation();
        assert_eq!(report.recommendation, Recommendation::Revise);

        report.scores.market_viability = 40;
        report.calculate_recommendation();
        assert_eq!(report.recommendation, Recommendation::Reject);
    }

    #[test]
    fn test_novel_score_calculation() {
        let score = NovelScore::calculate(80, 70, 60);
        assert_eq!(score.total_score, 71); // 80*0.4 + 70*0.35 + 60*0.25 = 71.5 -> 71
        assert_eq!(score.recommendation, Recommendation::Proceed);
    }
}
