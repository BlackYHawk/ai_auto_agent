# Tasks: GUI增强与项目管理系统

**Feature**: GUI增强与项目管理系统
**Branch**: 003-gui-enhancement

## Phase 1: Setup (项目初始化)

- [X] T001 创建项目级模型配置结构，在 src/models/novel.rs 中添加 ProjectModelConfig 结构体
- [X] T002 在 NovelProject 中添加 model_config 字段支持项目级模型配置
- [X] T003 更新 StorageService 添加扫描项目目录列表的功能

## Phase 2: Foundational (基础功能)

- [X] T004 创建项目列表服务，扫描 projects/ 目录获取所有项目
- [X] T005 [P] 创建导入服务基础结构，解析 TXT 文件格式

## Phase 3: User Story 1 - 项目列表展示 [US1]

- [X] T006 [US1] 修改 src/gui/screens/projects.rs 显示所有项目卡片列表
- [X] T007 [US1] 在项目卡片中显示：名称、类型、进度百分比、创建时间
- [X] T008 [US1] 添加按创建时间或名称排序功能
- [X] T009 [US1] 添加搜索项目功能

**Independent Test Criteria**: 用户打开应用后能看到所有项目卡片

## Phase 4: User Story 2 - 项目详情查看 [US2]

- [X] T010 [US2] 新增 src/gui/screens/project.rs 项目详情页面
- [X] T011 [US2] 显示项目基本信息（名称、类型、目标字数、当前字数）
- [X] T012 [US2] 显示可行性研究报告摘要
- [X] T013 [US2] 显示小说大纲（章节规划）
- [X] T014 [US2] 显示章节列表（带分页）

**Independent Test Criteria**: 点击项目卡片能看到完整项目信息

## Phase 5: User Story 3 - 章节内容查看 [US3]

- [X] T015 [US3] 新增 src/gui/screens/chapter.rs 章节查看页面
- [X] T016 [US3] 显示章节标题和正文
- [X] T017 [US3] 显示章节元数据（字数、状态、创建时间）
- [X] T018 [US3] 支持编辑章节内容

**Independent Test Criteria**: 点击章节能看到完整内容和元数据

## Phase 6: User Story 4 - 小说导入 [US4]

- [X] T019 [US4] 新增 src/gui/screens/import.rs 导入页面
- [X] T020 [US4] 实现文件选择器支持 TXT 格式
- [X] T021 [US4] 实现按"第X章"分割章节的解析逻辑
- [X] T022 [US4] 生成项目基本信息并保存到 projects/ 目录

**Independent Test Criteria**: 导入 TXT 文件后能在系统中继续编辑

## Phase 7: User Story 5 - 全局模型配置 [US5]

- [X] T023 [US5] 新增 src/gui/screens/settings.rs 设置页面
- [X] T024 [US5] 在设置页面添加全局 LLM 模型选择（Qwen、MiniMax、OpenAI）
- [X] T025 [US5] 添加 API Key 输入框
- [X] T026 [US5] 添加模型参数配置（temperature、max_tokens）
- [X] T027 [US5] 将配置保存到 config.toml

**Independent Test Criteria**: 在 GUI 中配置模型后，生成小说使用配置的模型

## Phase 8: User Story 6 - 项目级模型切换 [US6]

- [X] T028 [US6] 在项目详情页显示当前使用模型
- [X] T029 [US6] 添加项目级模型切换下拉框
- [X] T030 [US6] 保存项目级模型配置到项目 JSON 文件
- [X] T031 [US6] 生成时优先使用项目级配置

**Independent Test Criteria**: 为特定项目切换模型后，该项目后续生成使用新模型

## Phase 9: Polish (优化)

- [X] T032 处理项目列表为空时的提示界面
- [X] T033 处理导入文件格式不正确的错误提示
- [X] T034 处理 API Key 未配置时的提示
- [X] T035 确保所有界面文本为中文

## Dependencies

- T001, T002, T003 → T004
- T004, T005 → T006 (并行)
- T006 → T010 (T010需要项目列表)
- T010 → T015 (T015在T010之后)
- T001, T002 → T028 (项目级模型需要model_config)

## Parallel Opportunities

- T001, T002, T003 可以并行执行（不同文件）
- T006, T007, T008, T009 可以并行（同一文件的不同区域）
- T010, T011, T012, T013, T014 可以并行（同一文件）
- T023, T024, T025 可以并行（同一文件）

## Implementation Strategy

**MVP**: Phase 3-5 (US1, US2) - 实现项目列表和详情查看
**Phase 2**: Phase 6 (US3) - 章节查看
**Phase 3**: Phase 7 (US4) - 导入功能
**Phase 4**: Phase 8 (US5, US6) - 模型配置
