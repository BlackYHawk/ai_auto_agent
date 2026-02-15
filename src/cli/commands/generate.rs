//! Generate Command

use anyhow::Result;
use uuid::Uuid;
use crate::services::llm::LlmClient;
use crate::services::generation::GenerationService;
use crate::services::StorageService;

pub async fn run(project_id: &str, chapters: &str) -> Result<()> {
    tracing::info!("Generating chapters {} for: {}", chapters, project_id);

    let project_uuid = Uuid::parse_str(project_id)?;

    // Parse chapter range
    let chapter_nums: Vec<u32> = if chapters.contains('-') {
        let parts: Vec<&str> = chapters.split('-').collect();
        let start: u32 = parts[0].parse()?;
        let end: u32 = parts[1].parse()?;
        (start..=end).collect()
    } else {
        vec![chapters.parse()?]
    };

    println!("Generating {} chapters...", chapter_nums.len());

    // Create mock LLM client
    let llm_client = LlmClient::new(Box::new(crate::services::llm::QwenProvider::new("mock".to_string(), None)));
    let service = GenerationService::new(llm_client);

    // Create storage for project
    let storage = StorageService::new_project(".", project_uuid)?;

    for chapter_num in &chapter_nums {
        let context = format!("Previous chapters context for chapter {}", chapter_num);
        let prompt = format!("Generate chapter {} content based on outline", chapter_num);

        let chapter = service.generate_chapter(
            project_uuid,
            *chapter_num,
            &context,
            &prompt,
        ).await?;

        // Save chapter to project directory
        storage.save(&chapter)?;

        println!("\n=== Chapter {} ===", chapter.chapter_number);
        println!("Title: {}", chapter.title);
        println!("Content preview: {}", &chapter.content[..chapter.content.len().min(200)]);
        println!("Word count: {}", chapter.word_count);
    }

    println!("\n✅ Generated {} chapters", chapter_nums.len());
    println!("Saved to: projects/{}/chapters/", project_id);

    Ok(())
}
