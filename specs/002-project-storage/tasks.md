# Tasks: Project-Based File Organization

**Input**: Design documents from `specs/002-project-storage/`
**Prerequisites**: plan.md (required), spec.md (required for user stories)

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [X] T001 Review existing StorageService implementation in src/services/storage.rs

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

- [X] T002 Modify StorageService to support project-scoped paths in src/services/storage.rs
- [X] T003 [P] Add project directory creation method in src/services/storage.rs
- [X] T004 Add create_project_dirs function to create analysis/, outline/, plans/, chapters/ subdirectories

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Project Directory Structure (Priority: P1)

**Goal**: Create dedicated folder structure when a new project is initialized

**Independent Test**: Create a new project and verify all expected subdirectories are created

### Implementation for User Story 1

- [X] T005 [US1] Update new command to create project directories in src/main.rs
- [X] T006 [US1] Test project directory creation with cargo run -- new command

**Checkpoint**: Project directories are created automatically

---

## Phase 4: User Story 2 - Analysis Storage (Priority: P1)

**Goal**: Save feasibility analysis to project directory

**Independent Test**: Run feasibility analysis and verify report is saved to projects/<id>/analysis/

### Implementation for User Story 2

- [X] T007 [P] [US2] Update feasibility command to use project-scoped path in src/cli/commands/feasibility.rs
- [X] T008 [US2] Test analysis saving to correct directory

---

## Phase 5: User Story 3 - Outline Storage (Priority: P1)

**Goal**: Save novel outline to project directory

**Independent Test**: Generate outline and verify it's saved to projects/<id>/outline/

### Implementation for User Story 3

- [X] T009 [P] [US3] Update outline command to use project-scoped path in src/cli/commands/outline.rs
- [X] T010 [US3] Test outline saving to correct directory

---

## Phase 6: User Story 4 - Chapter Plan Storage (Priority: P1)

**Goal**: Save chapter plans to project directory

**Independent Test**: Generate chapter plan and verify it's saved to projects/<id>/plans/

### Implementation for User Story 4

- [X] T011 [P] [US4] Update plan command to use project-scoped path in src/cli/commands/plan.rs
- [X] T012 [US4] Test chapter plan saving to correct directory

---

## Phase 7: User Story 5 - Chapter Content Storage (Priority: P1)

**Goal**: Save generated chapters to project directory

**Independent Test**: Generate chapters and verify they appear in projects/<id>/chapters/

### Implementation for User Story 5

- [X] T013 [P] [US5] Update generate command to use project-scoped path in src/cli/commands/generate.rs
- [X] T014 [US5] Test chapter saving to correct directory

---

## Phase 8: Polish & Error Handling

**Purpose**: Improvements and robustness

- [X] T015 Add backup functionality for overwriting existing files
- [X] T016 Add error handling for disk full / permission errors
- [X] T017 Run cargo clippy and fix warnings

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies
- **Foundational (Phase 2)**: Depends on Setup - BLOCKS all user stories
- **User Stories (Phase 3-7)**: All depend on Foundational - can proceed in parallel

### User Story Dependencies

- All user stories depend on Foundational phase (Phase 2)
- US1 must complete before others can be meaningfully tested
- US2-US5 can be implemented in parallel after US1

### Parallel Opportunities

- Foundational tasks T002-T004 can run in parallel
- User stories US2-US5 can be implemented in parallel
- All storage path updates are independent

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1 - Directory Structure
4. **STOP and VALIDATE**: Test project creation creates directories
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Foundational → Directory creation ready
2. Add US2 (Analysis Storage) → Test → Deploy
3. Add US3 (Outline Storage) → Test → Deploy
4. Add US4 (Plan Storage) → Test → Deploy
5. Add US5 (Chapter Storage) → Test → Deploy
6. Polish phase

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests pass after implementation
- Commit after each task or logical group
