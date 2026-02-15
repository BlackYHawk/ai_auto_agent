//! Generation Service

use anyhow::Result;
use uuid::Uuid;
use crate::models::{GeneratedChapter, GenerationParams};

/// Chapter generation service
pub struct GenerationService {
    llm_client: crate::services::llm::LlmClient,
}

impl GenerationService {
    pub fn new(llm_client: crate::services::llm::LlmClient) -> Self {
        Self { llm_client }
    }

    /// Generate a chapter
    pub async fn generate_chapter(
        &self,
        project_id: Uuid,
        chapter_number: u32,
        context: &str,
        prompt: &str,
    ) -> Result<GeneratedChapter> {
        tracing::info!("Generating chapter {} for project {}", chapter_number, project_id);

        let params = GenerationParams {
            model: "qwen2.5".to_string(),
            temperature: 0.8,
            max_tokens: 4096,
        };

        // Generate content using LLM
        let content = self.llm_client.generate(context, prompt).await?;

        let chapter = GeneratedChapter::new(
            project_id,
            chapter_number,
            format!("第{}章", chapter_number),
            content,
            params,
        );

        Ok(chapter)
    }

    /// Generate multiple chapters in batch
    pub async fn generate_batch(
        &self,
        project_id: Uuid,
        chapter_numbers: &[u32],
        context: &str,
    ) -> Result<Vec<GeneratedChapter>> {
        let mut chapters = Vec::new();

        for &num in chapter_numbers {
            let prompt = format!("Generate chapter {}", num);
            let chapter = self.generate_chapter(project_id, num, context, &prompt).await?;
            chapters.push(chapter);
        }

        Ok(chapters)
    }
}
