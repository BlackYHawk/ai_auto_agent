//! AI Novel Agent - Core Library
//!
//! A Rust-based AI agent for automated novel generation with million-word scale support.

pub mod models;
pub mod services;
pub mod cli;
pub mod config;

/// Re-export commonly used types
pub use anyhow::Result;
pub use anyhow::anyhow;

/// Initialize logging with tracing
pub fn init_logging() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ai_novel_agent=info,warn".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
