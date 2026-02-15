//! Vector Store Service

use anyhow::Result;

/// Vector storage interface
pub trait VectorStore: Send + Sync {
    fn add(&self, id: &str, vector: &[f32], payload: &str) -> Result<()>;
    fn search(&self, query: &[f32], top_k: usize) -> Result<Vec<SearchResult>>;
}

pub struct SearchResult {
    pub id: String,
    pub score: f32,
    pub payload: String,
}

/// In-memory vector store (simple implementation)
#[allow(dead_code)]
pub struct SimpleVectorStore {
    vectors: Vec<(String, Vec<f32>, String)>,
}

impl SimpleVectorStore {
    pub fn new() -> Self {
        Self { vectors: Vec::new() }
    }
}

impl VectorStore for SimpleVectorStore {
    #[allow(unused_variables)]
    fn add(&self, id: &str, vector: &[f32], payload: &str) -> Result<()> {
        // Simple implementation - store as-is
        Ok(())
    }

    fn search(&self, _query: &[f32], top_k: usize) -> Result<Vec<SearchResult>> {
        // Simple implementation - return empty
        Ok(Vec::with_capacity(top_k))
    }
}

impl Default for SimpleVectorStore {
    fn default() -> Self {
        Self::new()
    }
}
