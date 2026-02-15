# Implementation Plan: Project-Based File Organization

**Branch**: `002-project-storage` | **Date**: 2026-02-15 | **Spec**: specs/002-project-storage/spec.md
**Input**: Feature specification from `/specs/002-project-storage/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Implement project-based file organization for the AI Novel Agent. All generated content (feasibility analysis, outlines, chapter plans, and chapters) will be stored in dedicated project directories (`projects/<project-id>/`) with appropriate subdirectories: `analysis/`, `outline/`, `plans/`, and `chapters/`. This extends the existing StorageService to support project-scoped storage.

## Technical Context

**Language/Version**: Rust 1.75
**Primary Dependencies**: std::fs, serde_json (already in project)
**Storage**: File system (JSON files)
**Testing**: cargo test (existing)
**Target Platform**: CLI (Linux/macOS/Windows)
**Project Type**: Single CLI application
**Performance Goals**: N/A (file I/O bound, minimal latency requirements)
**Constraints**: None significant
**Scale/Scope**: Individual user projects (1-10 concurrent projects)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**TDD Compliance**: This feature follows TDD - tests exist for StorageService, new tests will be added for project-scoped storage.

**N/A**: No constitution violations for this simple file organization feature.

## Project Structure

### Documentation (this feature)

```text
specs/002-project-storage/
├── plan.md              # This file
├── spec.md              # Feature specification
├── research.md          # Not needed - straightforward implementation
├── data-model.md        # Not needed - uses existing models
├── quickstart.md        # Not needed - no new CLI commands
└── tasks.md             # Created by /speckit.tasks
```

### Source Code (repository root)

```text
src/
├── services/
│   └── storage.rs       # Modified to support project-scoped storage
├── models/
│   └── novel.rs         # NovelProject already has id field
└── cli/
    └── commands/
        └── [existing commands updated to use project-scoped storage]

tests/
├── contract/
├── integration/
└── unit/
```

**Structure Decision**: Single Rust project - modify existing StorageService to accept project_id parameter and create project directory structure automatically.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

Not applicable - no violations.

## Implementation Approach

### Phase 1: Directory Structure Creation

1. Modify `StorageService` to accept optional `project_id` parameter
2. Add method to create project directory structure: `projects/<project-id>/{analysis,outline,plans,chapters}/`
3. Update `new` command to call directory creation

### Phase 2: Update Storage Paths

1. Update feasibility command to save to `projects/<project-id>/analysis/`
2. Update outline command to save to `projects/<project-id>/outline/`
3. Update plan command to save to `projects/<project-id>/plans/`
4. Update generate command to save to `projects/<project-id>/chapters/`

### Phase 3: Backups and Error Handling

1. Add timestamp-based backup for overwrites
2. Add error handling for disk full / permission errors
