//! Services module

pub mod storage;
pub mod scraping;
pub mod feasibility;
pub mod scoring;
pub mod outline;
pub mod content_filter;
pub mod chapter_planning;
pub mod generation;
pub mod llm;
pub mod context;
pub mod fanqie;
pub mod vector_store;
pub mod consistency;
pub mod progress;
pub mod validation;

pub use storage::*;
pub use scraping::*;
pub use feasibility::*;
pub use scoring::*;
pub use outline::*;
pub use content_filter::*;
pub use chapter_planning::*;
pub use generation::*;
pub use llm::*;
pub use context::*;
pub use fanqie::*;
pub use vector_store::*;
pub use consistency::{ConsistencyCheckResult, ConsistencyChecker};
pub use progress::*;
pub use validation::{
    CopyrightChecker, ConsistencyChecker as ValidationConsistencyChecker,
    is_common_name, ProjectValidator,
};
