# Implementation Plan: AI Novel Generation Agent

**Branch**: `001-ai-novel-gen` | **Date**: 2026-02-14 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-ai-novel-gen/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

AI-powered novel generation system that supports million-character novels with context consistency. Features include: market analysis via Fanqie website scraping, automated outline and chapter planning with plot twists every 10 chapters, context-aware content generation, and Fanqie Writer platform integration for auto-publishing. The system now adds intelligent model selection based on writing task requirements and a scoring mechanism that evaluates novel potential against popular Fanqie works before generation.

## Technical Context

**Language/Version**: Rust 1.75+ (stable) | Per constitution: Rust implementation required
**Primary Dependencies**: reqwest, serde, tokio, tracing, qdrant (vector DB), playwright (browser automation)
**Storage**: File-based JSON/SQLite for novel data and context management
**Testing**: cargo test with 80% minimum coverage per constitution
**Target Platform**: Linux server, CLI interface
**Project Type**: Single CLI application
**Performance Goals**: <30s per chapter generation, <2GB memory, <500ms context retrieval
**Constraints**: Million-character context window support, Chinese language generation
**Scale/Scope**: Single novel project supporting 300-500 chapters (million+ characters)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Gates from Constitution

| Principle | Requirement | Status |
|-----------|-------------|--------|
| I. Context Consistency | Must maintain narrative consistency across all chapters | ✅ Covered by design |
| II. Outline-Driven | Must generate according to project outline | ✅ In scope |
| III. Memory Management | Must handle >1M character context | ✅ In scope |
| IV. Test-First Development | TDD mandatory | ✅ Will enforce |
| V. Observability | Structured logging required | ✅ In scope |
| VI. Modularity | Reusable library components | ✅ In scope |
| Rust Implementation | Must use Rust | ✅ Confirmed |
| Performance Standards | <30s/chapter, <2GB memory | ✅ In scope |
| Quality Gates | 80% test coverage | ✅ Will enforce |

**Gate Result**: PASS - All constitutional requirements covered by current scope

## Project Structure

### Documentation (this feature)

```text
specs/001-ai-novel-gen/
├── plan.md              # This file
├── spec.md              # Feature specification
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
└── tasks.md             # Phase 2 output
```

### Source Code (repository root)

```text
src/
├── models/              # Data structures
├── services/           # Business logic
├── cli/                # Command-line interface
└── lib/                # Core library

tests/
├── unit/
├── integration/
└── contract/
```

**Structure Decision**: Single Rust CLI project with library crate for core functionality

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| None yet | - | - |

---

## Phase 0: Research ✅ COMPLETED

**Research completed** - See [research.md](./research.md) for full findings:

1. **LLM Model Selection**: Qwen2.5 (primary) + Claude fallback - Best Chinese quality
2. **Fanqie Integration**: No official API - Using Playwright browser automation
3. **Scoring**: Three-dimensional framework - Market 40%, Content 35%, Feasibility 25%
4. **Context Management**: Semantic retrieval + summary compression

---

## Phase 1: Design ✅ COMPLETED

**Deliverables:**
- [x] Data model - [data-model.md](./data-model.md)
- [x] API contracts - [contracts/api-contracts.md](./contracts/api-contracts.md)
- [x] Quickstart - [quickstart.md](./quickstart.md)

