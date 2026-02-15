# Quickstart Guide: AI Novel Generation Agent

**Feature**: 001-ai-novel-gen
**Date**: 2026-02-14

---

## Prerequisites

- Rust 1.75+ installed
- LLM API key (Qwen or OpenAI)
- Fanqie account (for publishing - optional)

---

## Installation

```bash
# Clone the project
git clone <repository-url>
cd ai_auto_agent

# Build the project
cargo build --release

# Run setup
cargo run -- init
```

---

## Configuration

Create `config.toml`:

```toml
[llm]
provider = "qwen"  # or "openai", "claude"
api_key = "your-api-key"

[fanqie]
# Optional - for auto-publishing
cookie = "your-session-cookie"
enabled = false

[storage]
path = "./data"

[generation]
default_word_count = 1000000
chapters_per_batch = 10
```

---

## Basic Usage

### 1. Create a New Novel Project

```bash
# Interactive mode
cargo run -- new

# Or with parameters
cargo run -- new my-fantasy-novel --genre fantasy --target 1000000
```

### 2. Run Feasibility Analysis

```bash
# Analyze market for the genre
cargo run -- feasibility <project-id>
```

This will:
- Scrape Fanqie for genre data
- Analyze top 100 works
- Generate feasibility report
- Provide go/revise/reject recommendation

### 3. Generate Outline

```bash
# Generate novel outline
cargo run -- outline <project-id> --premise "A young cultivator discovers..."
```

This will:
- Generate complete story structure
- Create character arcs
- Design world settings
- Check for sensitive content

### 4. Plan Chapters

```bash
# Generate chapter breakdown
cargo run -- plan <project-id>
```

This will:
- Create 300-500 chapter plan
- Place plot twists every 10 chapters
- Generate chapter summaries

### 5. Generate Chapters

```bash
# Generate single chapter
cargo run -- generate <project-id> --chapter 1

# Generate batch
cargo run -- generate <project-id> --batch 1-50
```

### 6. Publish to Fanqie (Optional)

```bash
# Setup credentials (first time)
cargo run -- auth save --cookie "your-cookie"

# Create novel on Fanqie
cargo run -- publish create <project-id>

# Upload chapters
cargo run -- publish upload <project-id> --chapters 1-50

# Auto-submit for review
cargo run -- publish submit <project-id> --auto
```

---

## Workflow Diagram

```
┌─────────────┐
│   Create    │
│   Project   │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ Feasibility │ ──→ Reject? → Select different genre
│   Analysis  │
└──────┬──────┘
       │ Pass
       ▼
┌─────────────┐
│   Generate  │
│   Outline   │
└──────┬──────┘
       │ Approve
       ▼
┌─────────────┐
│Chapter Plan │
└──────┬──────┘
       │ Approve
       ▼
┌─────────────┐
│  Generate   │◄──────┐
│  Chapters   │       │
└──────┬──────┘       │
       │               │ Review loop
       ▼               │
┌─────────────┐       │
│   Review    │───────┘
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  Publish to │
│   Fanqie    │
└─────────────┘
```

---

## Scoring System

The system scores your novel concept based on:

| Dimension | Weight | Factors |
|-----------|--------|---------|
| Market Viability | 40% | Genre demand, competition, trends |
| Content Potential | 35% | Originality, emotional resonance, character appeal |
| Technical Feasibility | 25% | Plot scalability, conflict sustainability |

**Score Thresholds**:
- 70+: Proceed with generation
- 50-69: Revise concept
- <50: Try different genre

---

## Common Issues

### Fanqie Login Failed
- Ensure cookie is valid
- Try regenerating session cookie in browser

### Context Exceeded Error
- System will auto-compress older chapters
- Check consistency with `cargo run -- check <project-id>`

### Generation Quality Issues
- Adjust temperature in config
- Regenerate specific chapters

---

## Next Steps

- See [API Contracts](./contracts/api-contracts.md) for programmatic access
- See [Data Model](./data-model.md) for entity definitions
- See [Research](./research.md) for technical decisions
