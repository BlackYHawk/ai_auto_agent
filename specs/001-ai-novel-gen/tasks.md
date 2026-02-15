# Tasks: AI Novel Generation Agent

**Input**: Design documents from `specs/001-ai-novel-gen/`
**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md, contracts/

**Tests**: The constitution mandates TDD for all features. Tests are MANDATORY - write tests first, ensure they fail, then implement.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Single project**: `src/`, `tests/` at repository root
- Paths shown below assume single project

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [X] T001 Create project structure per implementation plan in src/
- [X] T002 Initialize Rust project with cargo and dependencies (Cargo.toml)
- [X] T003 [P] Configure logging with tracing in src/lib.rs
- [X] T004 [P] Setup error handling with anyhow in src/lib.rs

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T005 Setup data models (NovelProject, Chapter, etc.) in src/models/mod.rs
- [X] T006 [P] Implement file storage service in src/services/storage.rs
- [X] T007 [P] Create CLI argument parser in src/cli/mod.rs
- [X] T008 Setup config management in src/config/mod.rs

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Market Analysis & Feasibility Study (Priority: P1) 🎯 MVP

**Goal**: Generate feasibility report for specified genre by analyzing Fanqie Novel website

**Independent Test**: Can be tested by generating a feasibility report and verifying market analysis data

### Tests for User Story 1 (MANDATORY - Constitution IV)

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T009 [P] [US1] Contract test for feasibility analysis endpoint in tests/contract/test_feasibility.rs
- [X] T010 [P] [US1] Integration test for scraping service in tests/integration/test_scraping.rs

### Implementation for User Story 1

- [X] T011 [US1] Create scraping service in src/services/scraping.rs
- [X] T012 [US1] Implement FeasibilityReport model in src/models/feasibility.rs
- [X] T013 [US1] Build feasibility analysis service in src/services/feasibility.rs
- [X] T014 [US1] Add scoring algorithm in src/services/scoring.rs
- [X] T015 [US1] Implement CLI command for feasibility in src/cli/commands/feasibility.rs

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - Novel Outline Generation (Priority: P1)

**Goal**: Generate compelling novel outline with story arcs, character development, and sensitive content filtering

**Independent Test**: Can be tested by generating an outline and verifying complete story structure

### Tests for User Story 2 (MANDATORY)

- [X] T016 [P] [US2] Contract test for outline generation in tests/contract/test_outline.rs
- [X] T017 [P] [US2] Integration test for outline service in tests/integration/test_outline.rs

### Implementation for User Story 2

- [X] T018 [P] [US2] Create NovelOutline model in src/models/outline.rs
- [X] T019 [P] [US2] Create PlotArc and CharacterArc models in src/models/outline.rs
- [X] T020 [US2] Implement outline generation service in src/services/outline.rs
- [X] T021 [US2] Add sensitive content filter in src/services/content_filter.rs
- [X] T022 [US2] Implement CLI command for outline in src/cli/commands/outline.rs

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - Chapter Planning (Priority: P1)

**Goal**: Generate detailed chapter breakdown with plot twists every 10 chapters

**Independent Test**: Can be tested by verifying chapter plan has correct plot twist positions

### Tests for User Story 3 (MANDATORY)

- [X] T023 [P] [US3] Contract test for chapter planning in tests/contract/test_chapter_plan.rs

### Implementation for User Story 3

- [X] T024 [P] [US3] Create ChapterPlan model in src/models/chapter.rs
- [X] T025 [P] [US3] Create ChapterSummary model in src/models/chapter.rs
- [X] T026 [US3] Implement chapter planning service in src/services/chapter_planning.rs
- [X] T027 [US3] Add plot twist placement logic in src/services/chapter_planning.rs
- [X] T028 [US3] Implement CLI command for planning in src/cli/commands/plan.rs

**Checkpoint**: At this point, User Stories 1, 2, AND 3 should all work independently

---

## Phase 6: User Story 4 - Automated Chapter Generation (Priority: P1)

**Goal**: Generate actual chapter content maintaining context consistency

**Independent Test**: Can be tested by generating chapters and verifying consistency

### Tests for User Story 4 (MANDATORY)

- [X] T029 [P] [US4] Contract test for chapter generation in tests/contract/test_generation.rs
- [X] T030 [P] [US4] Integration test for generation service in tests/integration/test_generation.rs

### Implementation for User Story 4

- [X] T031 [P] [US4] Create GeneratedChapter model in src/models/chapter.rs
- [X] T032 [P] [US4] Implement LLM client wrapper in src/services/llm/mod.rs
- [X] T033 [P] [US4] Setup Qwen API client in src/services/llm/qwen.rs
- [X] T034 [P] [US4] Setup Claude fallback client in src/services/llm/claude.rs
- [X] T035 [US4] Implement chapter generation service in src/services/generation.rs
- [X] T036 [US4] Add context retrieval in src/services/context.rs
- [X] T037 [US4] Implement CLI command for generate in src/cli/commands/generate.rs

**Checkpoint**: At this point, User Stories 1-4 should all work independently

---

## Phase 7: User Story 5 - Fanqie Platform Integration (Priority: P2)

**Goal**: Auto-upload chapters to Fanqie Writer platform

**Independent Test**: Can be tested by connecting to Fanqie and verifying chapter upload

### Implementation for User Story 5

- [X] T038 [P] [US5] Create UserCredentials model in src/models/credentials.rs
- [X] T039 [P] [US5] Implement Fanqie client in src/services/fanqie/mod.rs
- [X] T040 [US5] Setup Playwright browser automation in src/services/fanqie/browser.rs
- [X] T041 [US5] Implement chapter upload service in src/services/fanqie/upload.rs
- [X] T042 [US5] Add retry logic with exponential backoff in src/services/fanqie/retry.rs
- [X] T043 [US5] Implement CLI command for publish in src/cli/commands/publish.rs

---

## Phase 8: User Story 6 - Context Management for Million-Word Scale (Priority: P1)

**Goal**: Efficiently manage context for million-character novels

**Independent Test**: Can be tested by generating long novels and verifying consistency

### Tests for User Story 6 (MANDATORY - Constitution IV)

- [X] T043b [P] [US6] Test for context consistency verification in tests/integration/test_consistency.rs

### Implementation for User Story 6

- [X] T044 [P] [US6] Implement vector storage interface in src/services/vector_store.rs
- [X] T045 [P] [US6] Setup Qdrant client in src/services/vector_store/qdrant.rs
- [X] T046 [US6] Implement semantic search in src/services/context/search.rs
- [X] T047 [US6] Add context compression in src/services/context/compression.rs
- [X] T048 [US6] Implement consistency checker in src/services/consistency.rs
- [X] T049 [US6] Add progress tracking service in src/services/progress.rs

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [X] T050 [P] Add unit tests for all models in tests/unit/
- [X] T051 [P] Add integration tests in tests/integration/
- [X] T052 Run cargo clippy and fix all warnings in src/
- [X] T053 Documentation updates in docs/
- [X] T054 Code cleanup and refactoring
- [X] T055 Performance optimization across all stories
- [X] T056 Run quickstart.md validation

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2)
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 3 (P1)**: Depends on User Story 2 (outline must be approved first)
- **User Story 4 (P1)**: Depends on User Story 3 (chapter plan must be approved first)
- **User Story 5 (P2)**: Can start after Foundational - Independent of other stories
- **User Story 6 (P1)**: Can start after Foundational - Can run in parallel with other stories

### Within Each User Story

- Tests (if included) MUST be written and FAIL before implementation
- Models before services
- Services before endpoints
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, User Stories 1, 2, 5, 6 can start in parallel
- US3 depends on US2 completion
- US4 depends on US3 completion
- All tests for a user story marked [P] can run in parallel
- Models within a story marked [P] can run in parallel

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together:
Task: "Contract test for feasibility analysis endpoint in tests/contract/test_feasibility.rs"
Task: "Integration test for scraping service in tests/integration/test_scraping.rs"

# Launch all models for User Story 1 together:
Task: "Create FeasibilityReport model in src/models/feasibility.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1 - Market Analysis & Feasibility Study
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo
5. Add User Story 4 → Test independently → Deploy/Demo
6. Add User Story 5 → Test independently → Deploy/Demo
7. Add User Story 6 → Test independently → Deploy/Demo
8. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (Market Analysis)
   - Developer B: User Story 2 (Outline Generation)
   - Developer C: User Story 5 (Fanqie Integration) - P2, can start early
   - Developer D: User Story 6 (Context Management) - P1, can start early
3. Stories complete and integrate independently
4. US3 depends on US2, US4 depends on US3 - coordinate accordingly

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
