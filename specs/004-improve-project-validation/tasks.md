# Tasks: 完善项目验证功能

**Feature**: 004-improve-project-validation
**Created**: 2026-02-21
**Input**: Feature specification from `spec.md`

## Phase 1: Setup (项目初始化)

- [x] T001 Create validation models in src/models/validation.rs
- [x] T002 Create validation services in src/services/validation/ directory
- [x] T003 [P] Create copyright character database in data/copyright_characters.json
- [x] T004 [P] Add genre keywords data in data/genre_keywords.json

## Phase 2: Foundational (基础任务 - 阻塞所有用户故事)

- [x] T005 [P] Implement ValidationResult struct in src/models/validation.rs
- [x] T006 [P] Implement ValidationError and ErrorCode enum in src/models/validation.rs
- [x] T007 [P] Implement project validation service in src/services/validation/project.rs
- [x] T008 Implement CLI argument validation for new command in src/cli/commands/mod.rs
- [ ] T009 [SKIP] Update Cargo.toml to remove GUI dependencies (GUI not removed per user request)

## Phase 3: User Story 1 - 项目创建时完整信息验证

**Goal**: 用户创建项目时必须提供标题、简介、类型、目标字数
**Independent Test**: 执行 `new` 命令缺少字段时被拒绝，提供完整信息时成功创建

- [x] T010 [US1] Add summary field to CLI new command in src/main.rs
- [x] T011 [US1] Add target validation in src/services/validation/project.rs
- [x] T012 [US1] Implement field validation logic in src/services/validation/project.rs
- [x] T013 [US1] Update new command to require all fields in src/cli/commands/mod.rs
- [x] T014 [US1] Add unit tests for project validation in tests/unit/test_validation.rs

## Phase 4: User Story 2 - 可行性分析基于真实数据

**Goal**: 可行性分析基于番茄小说网站真实数据
**Independent Test**: 执行 `feasibility` 命令返回基于番茄真实数据的分析结果

- [x] T015 [US2] [P] Extend scraping service to support genre pages in src/services/scraping.rs
- [x] T016 [US2] Implement MarketData struct in src/models/feasibility.rs
- [x] T017 [US2] Implement FeasibilityReport with data_source field in src/models/feasibility.rs
- [x] T018 [US2] Add cache mechanism for scraping results in src/services/scraping.rs
- [x] T019 [US2] Update feasibility analysis to use real data in src/services/feasibility.rs
- [x] T020 [US2] Add fallback handling for network failures in src/services/feasibility.rs

## Phase 5: User Story 3 - 大纲与项目信息匹配验证

**Goal**: 大纲生成时验证与项目简介、类型一致性
**Independent Test**: 执行 `outline` 命令，生成大纲与输入类型一致

- [x] T021 [US3] [P] Create genre keywords mapping in data/genre_keywords.json
- [x] T022 [US3] Implement ConsistencyCheckResult struct in src/models/validation.rs
- [x] T023 [US3] Implement consistency validation service in src/services/validation/consistency.rs
- [x] T024 [US3] Add keyword matching logic in src/services/validation/consistency.rs
- [x] T025 [US3] Integrate consistency check into outline generation in src/services/outline.rs

## Phase 6: User Story 4 - 角色名字版权检查

**Goal**: 生成角色名字时检查是否与知名小说角色重复
**Independent Test**: 主角名字为"萧炎"时系统警告侵权

- [x] T026 [US4] [P] Create copyright characters JSON database in data/copyright_characters.json
- [x] T027 [US4] Implement CopyrightCheckResult struct in src/models/validation.rs
- [x] T028 [US4] Implement RiskLevel enum in src/models/validation.rs
- [x] T029 [US4] Implement copyright check service in src/services/validation/copyright.rs
- [x] T030 [US4] Add name loading from JSON database in src/services/validation/copyright.rs
- [x] T031 [US4] Integrate copyright check into outline generation in src/services/outline.rs

## Phase 7: Integration & Polish

- [x] T032 Update check command to include all validations in src/cli/commands/check.rs
- [x] T033 Add integration tests in tests/integration/test_validation.rs
- [x] T034 Update README.md with new CLI parameters in README.md
- [x] T035 Run cargo clippy and fix warnings
- [x] T036 Run cargo test to verify all tests pass

## Dependencies

```
Phase 1 (Setup)
    │
    ├── T001 ──► T005 ──► T010 ──► T013 ──► T014 ──► Phase 7
    │                       │
    │                       └──► T011 ──► T012
    │
    ├── T002 ──► T006 ──► T015 ──► T018 ──► T019 ──► T020 ──► Phase 7
    │
    ├── T003 ──► T026 ──► T028 ──► T030 ──► T031 ──► Phase 7
    │
    └── T004 ──► T021 ──► T022 ──► T023 ──► T024 ──► T025 ──► T032 ──► Phase 7

Phase 2 blocks all User Stories
Phase 7 integrates all User Stories
```

## Parallel Execution Opportunities

| Tasks | Can Run In Parallel | Reason |
|-------|---------------------|--------|
| T003, T004 | Yes | Different files, no dependencies |
| T005, T006, T007 | Yes | Different structs in same module |
| T015, T021, T026 | Yes | Different data files |
| T016, T017 | Yes | Different structs |
| T022, T027 | Yes | Different structs |
| T028, T029 | Yes | Different functions |
| T030, T031 | Yes | Different integration points |

## Independent Test Criteria

| User Story | Test Criteria |
|------------|---------------|
| US1 | `new` 缺少字段时被拒绝，提供完整信息时成功创建 |
| US2 | `feasibility` 命令返回基于番茄真实数据的报告 |
| US3 | `outline` 生成大纲与输入类型一致 |
| US4 | 主角名字为"萧炎"时系统警告侵权 |

## MVP Scope

**建议MVP**: Phase 1-3 (User Story 1 完成)
- 最少任务数: T001-T014
- 核心价值: 项目创建时完整信息验证

## Implementation Strategy

1. **MVP First**: 先完成项目创建验证 (US1)
2. **Incremental Delivery**: 依次完成 US2, US3, US4
3. **Cross-Cutting**: Phase 7 进行整合和文档同步

**Total Tasks**: 36
