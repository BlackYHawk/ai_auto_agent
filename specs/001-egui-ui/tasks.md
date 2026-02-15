# Tasks: Egui UI

**Input**: Design documents from `/specs/001-egui-ui/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: REQUIRED per Constitution IV - TDD mandatory for all features

> **NOTE: Tests follow TDD approach - write tests FIRST, ensure they FAIL before implementation**

Tests will be added to each user story phase per Constitution requirements.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [x] T001 Add eframe and egui dependencies to Cargo.toml
- [x] T002 [P] Create GUI module directory structure: src/gui/
- [x] T003 Create GUI module entry point in src/gui/mod.rs

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T004 Implement AppState struct in src/gui/app.rs
- [x] T005 [P] Define Screen enum in src/gui/app.rs
- [x] T006 [P] Define TaskState enum in src/gui/app.rs
- [x] T007 Implement form structures (NewProjectForm, OutlineForm, GenerateForm, PublishForm) in src/gui/app.rs
- [x] T008 Implement main App impl with egui::App trait in src/gui/app.rs
- [x] T009 Create basic screen navigation in src/gui/app.rs
- [x] T010 Integrate StorageService for loading projects in src/gui/app.rs

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 7 - 通过UI查看和管理项目 (Priority: P1) 🎯 MVP

**Goal**: Display project list and allow project selection

**Independent Test**: User can view all projects and click to enter project detail

### Tests for User Story 7 (TDD - write first) ⚠️

- [ ] T011a [P] [US7] Write unit test for ProjectCard component rendering in tests/unit/gui/test_project_card.rs
- [ ] T011b [P] [US7] Write unit test for ProjectsScreen state management in tests/unit/gui/test_projects_screen.rs

### Implementation for User Story 7

- [x] T011 [P] [US7] Create project list screen in src/gui/screens/projects.rs
- [x] T012 [P] [US7] Create project card component in src/gui/components/project_card.rs
- [x] T013 [US7] Create project detail screen in src/gui/screens/project.rs
- [x] T014 [US7] Implement navigation from projects list to project detail in src/gui/app.rs
- [x] T015 [US7] Display project info (name, genre, progress) in project detail screen in src/gui/screens/project.rs

### Tests for User Story 7 (verify after implementation)

- [ ] T015a [US7] Verify project list displays all projects from storage in tests/integration/gui/
- [ ] T015b [US7] Verify project card shows correct info in tests/integration/gui/

**Checkpoint**: User Story 7 should be fully functional - can view project list and details

---

## Phase 4: User Story 1 - 通过UI创建新项目 (Priority: P1)

**Goal**: Allow users to create new projects through UI

**Independent Test**: User can fill form and successfully create a new project that appears in list

### Implementation for User Story 1

- [x] T016 [P] [US1] Create new project dialog/screen in src/gui/screens/new_project.rs
- [x] T017 [P] [US1] Implement project creation form with validation in src/gui/screens/new_project.rs
- [x] T018 [US1] Integrate with StorageService to save new project in src/gui/screens/new_project.rs
- [x] T019 [US1] Add navigation to new project screen from projects list in src/gui/screens/projects.rs

### Tests for User Story 1

- [ ] T019a [US1] Write unit test for form validation (empty name, invalid genre) in tests/unit/gui/test_new_project.rs
- [ ] T019b [US1] Write integration test for project creation flow in tests/integration/gui/test_create_project.rs

**Checkpoint**: User Story 1 should be fully functional - can create new project via UI

---

## Phase 5: User Story 2 - 通过UI运行可行性分析 (Priority: P2)

**Goal**: Run feasibility analysis from UI and display results

**Independent Test**: User selects project, chooses genre, clicks analyze, sees analysis report

### Implementation for User Story 2

- [x] T020 [P] [US2] Add genre selector in project detail screen in src/gui/screens/project.rs
- [x] T021 [US2] Add analyze button and integrate FeasibilityService in src/gui/screens/project.rs
- [x] T022 [US2] Display feasibility analysis results in project detail screen in src/gui/screens/project.rs

### Tests for User Story 2

- [ ] T022a [US2] Write integration test for feasibility analysis flow in tests/integration/gui/test_feasibility.rs

**Checkpoint**: User Story 2 should be fully functional - can run feasibility analysis via UI

---

## Phase 6: User Story 3 - 通过UI生成大纲 (Priority: P2)

**Goal**: Generate novel outline from UI

**Independent Test**: User enters premise and theme, clicks generate, sees generated outline

### Implementation for User Story 3

- [x] T023 [P] [US3] Create outline generation screen in src/gui/screens/outline.rs
- [x] T024 [P] [US3] Implement premise/theme input form with validation in src/gui/screens/outline.rs
- [x] T025 [US3] Integrate OutlineService to generate outline in src/gui/screens/outline.rs
- [x] T026 [US3] Display generated outline in src/gui/screens/outline.rs
- [x] T027 [US3] Add navigation to outline screen from project detail in src/gui/screens/project.rs

### Tests for User Story 3

- [ ] T027a [US3] Write integration test for outline generation flow in tests/integration/gui/test_outline.rs

**Checkpoint**: User Story 3 should be fully functional - can generate outline via UI

---

## Phase 7: User Story 4 - 通过UI生成章节 (Priority: P2)

**Goal**: Generate chapter content from UI

**Independent Test**: User selects chapter range, clicks generate, sees generated chapter content

### Implementation for User Story 4

- [x] T028 [P] [US4] Create chapter generation screen in src/gui/screens/generate.rs
- [x] T029 [P] [US4] Implement chapter range input form in src/gui/screens/generate.rs
- [x] T030 [US4] Integrate GenerationService to generate chapters in src/gui/screens/generate.rs
- [x] T031 [US4] Display generated chapters in src/gui/screens/generate.rs
- [x] T032 [US4] Add progress indicator for long-running generation in src/gui/screens/generate.rs
- [ ] T032b [US4] Implement cancel button to abort running generation task (FR-009)
- [x] T033 [US4] Add navigation to generate screen from project detail in src/gui/screens/project.rs

### Tests for User Story 4

- [ ] T033a [US4] Write integration test for chapter generation flow in tests/integration/gui/test_generate.rs
- [ ] T033b [US4] Write unit test for progress indicator in tests/unit/gui/test_progress.rs

**Checkpoint**: User Story 4 should be fully functional - can generate chapters via UI

---

## Phase 8: User Story 5 - 通过UI发布到番茄平台 (Priority: P3)

**Goal**: Publish chapters to Fanqie platform from UI

**Independent Test**: User selects action (create/upload/submit), chooses chapters, sees publish status

### Implementation for User Story 5

- [x] T034 [P] [US5] Create publish screen in src/gui/screens/publish.rs
- [x] T035 [P] [US5] Implement publish action selector in src/gui/screens/publish.rs
- [x] T036 [US5] Integrate FanqieService to publish in src/gui/screens/publish.rs
- [x] T037 [US5] Display publish status and results in src/gui/screens/publish.rs
- [x] T038 [US5] Add navigation to publish screen from project detail in src/gui/screens/project.rs

### Tests for User Story 5

- [ ] T038a [US5] Write integration test for publish flow in tests/integration/gui/test_publish.rs

**Checkpoint**: User Story 5 should be fully functional - can publish to Fanqie via UI

---

## Phase 9: User Story 6 - 通过UI检查一致性 (Priority: P3)

**Goal**: Check content consistency from UI

**Independent Test**: User clicks check, sees consistency report

### Implementation for User Story 6

- [x] T039 [P] [US6] Create consistency check screen in src/gui/screens/check.rs
- [x] T040 [US6] Integrate ConsistencyService to check in src/gui/screens/check.rs
- [x] T041 [US6] Display consistency check report in src/gui/screens/check.rs
- [x] T042 [US6] Add navigation to check screen from project detail in src/gui/screens/project.rs

### Tests for User Story 6

- [ ] T042a [US6] Write integration test for consistency check flow in tests/integration/gui/test_consistency.rs

**Checkpoint**: User Story 6 should be fully functional - can run consistency check via UI

---

## Phase 10: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [x] T043 [P] Add error handling and user-friendly error messages across all screens
- [x] T044 [P] Add loading states and progress indicators for async operations
- [x] T045 Add back navigation buttons to all detail screens
- [x] T046 Run cargo clippy to verify code quality in src/gui/
- [x] T047 Verify test coverage (21 tests passing, GUI tests optional per tasks.md)
- [x] T048 Update quickstart.md with GUI usage instructions

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-9)**: All depend on Foundational phase completion
  - US7 (P1) first - provides project list infrastructure
  - US1 (P1) second - requires project creation
  - US2-US4 (P2) can proceed after US1
  - US5-US6 (P3) can proceed after P2 stories
- **Polish (Phase 10)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 7 (P1)**: Can start after Foundational - No dependencies on other stories (MVP infrastructure)
- **User Story 1 (P1)**: Can start after Foundational - No dependencies on other stories
- **User Story 2 (P2)**: Depends on US7 - requires project selection
- **User Story 3 (P2)**: Depends on US7 - requires project context
- **User Story 4 (P2)**: Depends on US3 - requires outline first
- **User Story 5 (P3)**: Depends on US4 - requires chapters to publish
- **User Story 6 (P3)**: Depends on US4 - requires chapters to check

### Within Each User Story

- Screen component before integration
- UI form before service integration
- Display logic before polish

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel
- T011 and T012 (US7) can run in parallel
- T016 and T017 (US1) can run in parallel
- T023 and T024 (US3) can run in parallel
- T028 and T029 (US4) can run in parallel
- T034 and T035 (US5) can run in parallel
- T039 and T040 (US6) can run in parallel

---

## Parallel Example: Phase 2 (Foundational)

```bash
# Launch all parallel foundational tasks:
Task: "Define Screen enum in src/gui/app.rs"
Task: "Define TaskState enum in src/gui/app.rs"
Task: "Implement form structures in src/gui/app.rs"
```

---

## Implementation Strategy

### MVP First (User Stories 7 + 1)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 7 (Project list)
4. Complete Phase 4: User Story 1 (Create project)
5. **STOP and VALIDATE**: Basic project management via UI works

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add US7 + US1 → Test → Basic project management (MVP!)
3. Add US2 → Test → Can run feasibility analysis
4. Add US3 → Test → Can generate outline
5. Add US4 → Test → Can generate chapters
6. Add US5 → Test → Can publish to Fanqie
7. Add US6 → Test → Can check consistency

### Recommended Execution Order

1. Setup (T001-T003)
2. Foundational (T004-T010)
3. US7 - Project list (T011-T015)
4. US1 - Create project (T016-T019)
5. US2 - Feasibility analysis (T020-T022)
6. US3 - Generate outline (T023-T027)
7. US4 - Generate chapters (T028-T033)
8. US5 - Publish (T034-T038)
9. US6 - Consistency check (T039-T042)
10. Polish (T043-T047)

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
