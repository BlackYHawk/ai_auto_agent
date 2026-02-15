# Feature Specification: Project-Based File Organization

**Feature Branch**: `002-project-storage`
**Created**: 2026-02-15
**Status**: Draft
**Input**: User description: "生成的analysis、outline、chapter plan、chapters都在对应的projects下生成小说目录的中"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Project Directory Structure (Priority: P1)

When users create a novel project, the system automatically creates a dedicated folder structure to organize all generated content.

**Why this priority**: This is the foundational requirement - without proper file organization, users cannot manage their novel projects effectively.

**Independent Test**: Can be tested by creating a new project and verifying that all expected subdirectories are created automatically.

**Acceptance Scenarios**:

1. **Given** a new project is created with name "我的修仙小说", **When** the project is initialized, **Then** a folder `projects/<project-id>/` is created with subdirectories: `analysis/`, `outline/`, `chapters/`, `plans/`
2. **Given** a project directory already exists, **When** additional content is generated, **Then** files are saved to the appropriate subdirectory without overwriting existing content

---

### User Story 2 - Analysis Output Storage (Priority: P1)

Feasibility analysis reports are saved in the project's analysis directory.

**Why this priority**: Analysis reports are essential for project decision-making and must be preserved alongside the project.

**Independent Test**: Can be tested by running feasibility analysis and verifying the report is saved in `projects/<project-id>/analysis/`.

**Acceptance Scenarios**:

1. **Given** a project exists, **When** feasibility analysis is run, **Then** a JSON report is saved to `projects/<project-id>/analysis/feasibility.json`
2. **Given** multiple analysis runs, **When** new analysis is generated, **Then** previous reports are preserved with timestamps

---

### User Story 3 - Outline Storage (Priority: P1)

Novel outlines are saved in the project's outline directory.

**Why this priority**: The outline is the blueprint for the entire novel and must be easily accessible for reference.

**Independent Test**: Can be tested by generating an outline and verifying it's saved in `projects/<project-id>/outline/`.

**Acceptance Scenarios**:

1. **Given** a project exists, **When** outline is generated, **Then** the outline is saved to `projects/<project-id>/outline/outline.json`
2. **Given** an outline already exists, **When** a new outline is generated, **Then** the old outline is backed up before overwriting

---

### User Story 4 - Chapter Plan Storage (Priority: P1)

Chapter plans are saved in the project's plans directory.

**Why this priority**: Chapter plans define the novel structure and must be preserved for generation reference.

**Independent Test**: Can be tested by generating a chapter plan and verifying it's saved in `projects/<project-id>/plans/`.

**Acceptance Scenarios**:

1. **Given** a project exists, **When** chapter plan is generated, **Then** the plan is saved to `projects/<project-id>/plans/chapter_plan.json`
2. **Given** chapter plans are regenerated, **Then** previous versions are preserved

---

### User Story 5 - Chapter Content Storage (Priority: P1)

Generated chapters are saved in the project's chapters directory.

**Why this priority**: The actual novel content must be stored in an organized manner for review and publishing.

**Independent Test**: Can be tested by generating chapters and verifying they appear in `projects/<project-id>/chapters/`.

**Acceptance Scenarios**:

1. **Given** a project exists, **When** chapters are generated, **Then** each chapter is saved as `projects/<project-id>/chapters/chapter_<N>.json`
2. **Given** chapters are generated incrementally, **When** chapter N+1 is added, **Then** previous chapters remain untouched

---

### Edge Cases

- What happens when the project directory is deleted externally but the system tries to save to it?
- How does the system handle disk full errors during file writes?
- What happens when file permissions prevent writing to the project directory?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST create a project directory structure automatically when a new project is initialized
- **FR-002**: System MUST save feasibility analysis reports to `projects/<project-id>/analysis/`
- **FR-003**: System MUST save novel outlines to `projects/<project-id>/outline/`
- **FR-004**: System MUST save chapter plans to `projects/<project-id>/plans/`
- **FR-005**: System MUST save generated chapters to `projects/<project-id>/chapters/`
- **FR-006**: System MUST preserve existing files when new content is generated (no overwriting without backup)
- **FR-007**: System MUST create subdirectories automatically if they don't exist

### Key Entities

- **Project Directory**: Root folder for each novel project, identified by project UUID
- **Analysis Folder**: Contains feasibility analysis JSON files
- **Outline Folder**: Contains novel outline JSON files
- **Plans Folder**: Contains chapter plan JSON files
- **Chapters Folder**: Contains individual chapter JSON files

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can locate all project-related files within 3 seconds by navigating to `projects/<project-id>/`
- **SC-002**: All generated content (analysis, outline, plans, chapters) is persisted and retrievable after program restart
- **SC-003**: No data loss occurs when generating new content - previous versions are preserved
- **SC-004**: File organization follows consistent naming conventions across all output types
