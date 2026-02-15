//! Chapter Planning Service

use anyhow::Result;
use uuid::Uuid;
use crate::models::{ChapterPlan, ChapterSummary, NovelOutline};

/// Chapter planning service
pub struct ChapterPlanningService;

impl ChapterPlanningService {
    pub fn new() -> Self {
        Self
    }

    /// Generate chapter plan from outline
    pub async fn generate_plan(
        &self,
        project_id: Uuid,
        outline: &NovelOutline,
    ) -> Result<ChapterPlan> {
        tracing::info!("Generating chapter plan for project: {}", project_id);

        // Calculate total chapters based on word count target
        let target_words = outline.target_word_count;
        let words_per_chapter = 10_000;
        let total_chapters = ((target_words as f64 / words_per_chapter as f64).ceil() as u32).max(10);

        let mut plan = ChapterPlan::new(project_id, total_chapters);

        // Generate chapter summaries based on plot arcs
        let arcs = &outline.arcs;
        let protagonist = &outline.protagonist;

        for i in 1..=total_chapters {
            let is_plot_twist = i % 10 == 0;

            // Find which arc this chapter belongs to
            let current_arc = arcs.iter().find(|arc| i >= arc.start_chapter && i <= arc.end_chapter);

            let (title, summary, key_event) = if let Some(arc) = current_arc {
                if i == arc.start_chapter {
                    (format!("第{}章 {}", i, arc.name), arc.summary.clone(), arc.key_events.first().cloned().unwrap_or_default())
                } else if i == arc.end_chapter {
                    (format!("第{}章 转折", i), format!("{}的高潮", arc.name), arc.climax.clone())
                } else {
                    (format!("第{}章", i), format!("继续{}", arc.name), format!("第{}章的事件", i))
                }
            } else {
                (format!("第{}章", i), "过渡章节".to_string(), format!("章节{}事件", i))
            };

            let summary = ChapterSummary {
                number: i,
                title,
                summary,
                key_events: vec![key_event],
                protagonist_development: format!("{}的成长", protagonist.name),
                word_count_estimate: words_per_chapter as u32,
                is_plot_twist_chapter: is_plot_twist,
                plot_twist_description: if is_plot_twist {
                    Some(self.generate_plot_twist(i, current_arc))
                } else {
                    None
                },
            };

            plan.chapters.push(summary);
        }

        Ok(plan)
    }

    /// Generate a plot twist description for a chapter
    #[allow(unused_variables)]
    fn generate_plot_twist(&self, chapter: u32, arc: Option<&crate::models::PlotArc>) -> String {
        let twists = vec![
            "主角遭遇重大危机",
            "隐藏实力被发现",
            "神秘强者出现",
            "身份意外曝光",
            "生死攸关的抉择",
            "强大敌人来袭",
            "意外获得珍贵宝物",
            "亲人/朋友遇险",
            "势力格局巨变",
            "真相浮出水面",
        ];

        let idx = (chapter / 10 - 1) as usize % twists.len();
        twists[idx].to_string()
    }

    /// Update a specific chapter in the plan
    pub fn update_chapter(&mut self, plan: &mut ChapterPlan, chapter_num: u32, summary: ChapterSummary) -> Result<()> {
        if let Some(chapter) = plan.chapters.iter_mut().find(|c| c.number == chapter_num) {
            *chapter = summary;
        }
        Ok(())
    }
}

impl Default for ChapterPlanningService {
    fn default() -> Self {
        Self::new()
    }
}
