//! Contract tests for chapter generation

#[cfg(test)]
mod tests {
    use ai_novel_agent::models::{GeneratedChapter, GenerationParams, ChapterStatus};

    /// Test chapter creation
    #[test]
    fn test_create_chapter() {
        let params = GenerationParams {
            model: "qwen2.5".to_string(),
            temperature: 0.8,
            max_tokens: 4096,
        };

        let chapter = GeneratedChapter::new(
            uuid::Uuid::new_v4(),
            1,
            "第一章 起始".to_string(),
            "这是第一章的内容".to_string(),
            params.clone(),
        );

        assert_eq!(chapter.chapter_number, 1);
        assert_eq!(chapter.status, ChapterStatus::Draft);
        assert_eq!(chapter.word_count, 8); // Chinese chars
    }

    /// Test chapter approval
    #[test]
    fn test_chapter_approval() {
        let params = GenerationParams {
            model: "test".to_string(),
            temperature: 0.8,
            max_tokens: 4096,
        };

        let mut chapter = GeneratedChapter::new(
            uuid::Uuid::new_v4(),
            1,
            "Test".to_string(),
            "Content".to_string(),
            params,
        );

        chapter.approve();
        assert_eq!(chapter.status, ChapterStatus::Approved);
    }
}
