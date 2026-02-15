//! Outline Generation Service

use anyhow::Result;
use uuid::Uuid;
use crate::models::{
    NovelOutline, NovelGenre, OutlineStatus, PlotArc, CharacterArc,
    CharacterRole, CharacterMoment, WorldSettings, WorldType, Location,
    LocationImportance,
};

/// Outline generation service
#[allow(dead_code)]
pub struct OutlineService {
    llm_client: Option<crate::services::llm::LlmClient>,
}

impl OutlineService {
    /// Create a new outline service
    pub fn new() -> Self {
        Self { llm_client: None }
    }

    /// Create with LLM client
    pub fn with_llm(client: crate::services::llm::LlmClient) -> Self {
        Self { llm_client: Some(client) }
    }

    /// Generate an outline
    pub async fn generate(
        &self,
        project_id: Uuid,
        genre: NovelGenre,
        premise: String,
        theme: String,
        target_word_count: u64,
    ) -> Result<NovelOutline> {
        tracing::info!("Generating outline for project: {}", project_id);

        let mut outline = NovelOutline::new(project_id, premise, theme, target_word_count);

        // Generate plot arcs based on genre
        outline.arcs = self.generate_plot_arcs(genre, target_word_count);

        // Generate protagonist
        outline.protagonist = self.generate_protagonist(genre);

        // Generate supporting characters
        outline.supporting_characters = self.generate_supporting_characters(genre);

        // Generate world settings
        outline.world_settings = self.generate_world_settings(genre);

        outline.status = OutlineStatus::Draft;

        Ok(outline)
    }

    /// Generate plot arcs
    #[allow(unused_variables)]
    fn generate_plot_arcs(&self, genre: NovelGenre, word_count: u64) -> Vec<PlotArc> {
        let total_chapters = (word_count / 10_000) as u32;

        vec![
            PlotArc {
                id: Uuid::new_v4(),
                name: "序章与起源".to_string(),
                start_chapter: 1,
                end_chapter: total_chapters / 3,
                summary: "主角经历重大事件,开启冒险之旅".to_string(),
                key_events: vec!["主角身世揭晓".to_string(), "获得能力或机遇".to_string()],
                climax: "首次重大胜利".to_string(),
            },
            PlotArc {
                id: Uuid::new_v4(),
                name: "成长与挑战".to_string(),
                start_chapter: total_chapters / 3 + 1,
                end_chapter: 2 * total_chapters / 3,
                summary: "主角面对更强的敌人,不断成长".to_string(),
                key_events: vec!["遇见导师".to_string(), "获得珍贵资源".to_string(), "经历重大挫折".to_string()],
                climax: "击败强劲对手".to_string(),
            },
            PlotArc {
                id: Uuid::new_v4(),
                name: "终极对决".to_string(),
                start_chapter: 2 * total_chapters / 3 + 1,
                end_chapter: total_chapters,
                summary: "最终决战,解决最大威胁".to_string(),
                key_events: vec!["真相大白".to_string(), "最终准备".to_string()],
                climax: "终极对决与胜利".to_string(),
            },
        ]
    }

    /// Generate protagonist
    fn generate_protagonist(&self, genre: NovelGenre) -> CharacterArc {
        let (name, traits) = match genre {
            NovelGenre::Fantasy | NovelGenre::Xianxia => ("叶凡".to_string(), vec!["坚韧".to_string(), "机智".to_string(), "重情义".to_string()]),
            NovelGenre::Urban => ("林逸".to_string(), vec!["低调".to_string(), "腹黑".to_string(), "护短".to_string()]),
            NovelGenre::Romance => ("顾宁".to_string(), vec!["温柔".to_string(), "坚强".to_string(), "善良".to_string()]),
            _ => ("主角".to_string(), vec!["勇敢".to_string(), "智慧".to_string()]),
        };

        CharacterArc {
            id: Uuid::new_v4(),
            name,
            role: CharacterRole::Protagonist,
            description: "故事的主角,经历重大成长".to_string(),
            personality_traits: traits,
            arc_description: "从弱小到强大,从迷茫到坚定".to_string(),
            key_moments: vec![
                CharacterMoment {
                    chapter: 1,
                    description: "故事开始".to_string(),
                    development: "展现潜力".to_string(),
                },
            ],
        }
    }

    /// Generate supporting characters
    #[allow(unused_variables)]
    fn generate_supporting_characters(&self, genre: NovelGenre) -> Vec<CharacterArc> {
        vec![
            CharacterArc {
                id: Uuid::new_v4(),
                name: "导师/贵人".to_string(),
                role: CharacterRole::Supporting,
                description: "帮助主角成长的人物".to_string(),
                personality_traits: vec!["智慧".to_string(), "神秘".to_string()],
                arc_description: "传授技能,提供帮助".to_string(),
                key_moments: vec![],
            },
            CharacterArc {
                id: Uuid::new_v4(),
                name: "对手/敌人".to_string(),
                role: CharacterRole::Antagonist,
                description: "主角的主要对手".to_string(),
                personality_traits: vec!["强大".to_string(), "阴险".to_string()],
                arc_description: "给主角制造麻烦,最终被打败".to_string(),
                key_moments: vec![],
            },
        ]
    }

    /// Generate world settings
    fn generate_world_settings(&self, genre: NovelGenre) -> WorldSettings {
        let (world_type, name, rules) = match genre {
            NovelGenre::Fantasy => (WorldType::Fantasy, "玄幻世界".to_string(), vec!["修炼体系".to_string(), "灵石为基础".to_string()]),
            NovelGenre::Xianxia => (WorldType::Xianxia, "修仙界".to_string(), vec!["灵气修炼".to_string(), "境界划分".to_string(), "天劫考验".to_string()]),
            NovelGenre::Urban => (WorldType::Modern, "现代都市".to_string(), vec!["金钱至上".to_string(), "弱肉强食".to_string()]),
            NovelGenre::Scifi => (WorldType::Scifi, "未来世界".to_string(), vec!["科技为主".to_string(), "星际旅行".to_string()]),
            _ => (WorldType::Modern, "普通世界".to_string(), vec![]),
        };

        WorldSettings {
            name,
            world_type,
            description: format!("{:?}类型的小说世界", genre),
            rules,
            locations: vec![
                Location {
                    name: "主要场景".to_string(),
                    description: "故事主要发生的地点".to_string(),
                    importance: LocationImportance::Major,
                },
            ],
        }
    }
}

impl Default for OutlineService {
    fn default() -> Self {
        Self::new()
    }
}
