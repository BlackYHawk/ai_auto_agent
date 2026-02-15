# Implementation Plan: Egui UI

**Branch**: `001-egui-ui` | **Date**: 2026-02-15 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-egui-ui/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

创建一个基于Egui的桌面GUI应用程序，将现有CLI命令功能通过图形界面呈现。主要目标是实现项目管理和操作功能的UI化，包括创建项目、可行性分析、生成大纲、生成章节、发布到番茄平台、一致性检查等核心功能。技术方案采用在现有Rust CLI项目基础上添加GUI入口点，使用eframe作为Egui框架的入口。

## Technical Context

**Language/Version**: Rust 2021 edition
**Primary Dependencies**: eframe (egui框架), clap (CLI解析 - 已有), tokio (异步运行时 - 已有)
**Storage**: 本地文件系统（与CLI共享同一存储结构）
**Testing**: cargo test（现有项目标准）
**Target Platform**: 桌面平台 (Windows, macOS, Linux)
**Project Type**: 单项目 CLI + GUI 双入口架构
**Performance Goals**: GUI响应性 < 100ms, 生成性能符合现有标准 (< 30秒/章)
**Constraints**: 必须复用现有CLI模块，保持功能一致性
**Scale/Scope**: 单用户桌面应用，支持本地多项目管理

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Rust Implementation
- [x] 使用Rust实现 - 现有项目为Rust，egui是Rust原生框架
- [x] 代码通过cargo clippy检查 - GUI代码需同样遵守
- [x] 依赖安全审计 - egui/eframe是成熟稳定库

### Performance Standards
- [x] 生成延迟符合标准 - 底层CLI逻辑不变，仅UI封装
- [x] 内存使用符合标准 - egui内存占用可控
- [x] 启动时间符合标准 - 需优化冷启动时间

### Quality Gates
- [x] 测试覆盖率80%+ - GUI模块需添加单元测试
- [x] 无unsafe代码 - GUI代码无需unsafe
- [x] 公共API文档 - GUI组件需文档

### Core Principles Check
- [x] 上下文一致性 - 底层CLI逻辑保持不变
- [x] 大纲驱动生成 - 底层逻辑不变
- [x] 内存管理 - 复用现有实现
- [x] 可观测性 - 复用现有日志系统

## Project Structure

### Documentation (this feature)

```text
specs/001-egui-ui/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output (UI component contracts)
└── tasks.md            # Phase 2 output (NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src/
├── models/              # 已有
├── services/           # 已有
├── cli/                # 已有
├── gui/                # 新增: GUI模块
│   ├── app.rs          # 主应用结构
│   ├── screens/        # 页面组件
│   │   ├── projects.rs # 项目列表
│   │   ├── project.rs  # 项目详情
│   │   ├── new.rs      # 新建项目
│   │   ├── outline.rs  # 大纲生成
│   │   └── publish.rs  # 发布页面
│   └── components/     # 通用组件
└── lib.rs

Cargo.toml              # 添加 eframe 依赖
```

**Structure Decision**: 在现有单项目结构中添加 `src/gui/` 模块，作为独立的GUI入口点。通过库方式调用现有的CLI模块（models, services），实现功能复用。

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| 无 | 本功能为UI封装层，未引入架构复杂度 | N/A |

## Phase 0: Research

由于用户明确指定使用egui框架，无需进行额外技术调研。用户描述"基于egui框架创建界面"已经明确了技术选型。

### Research Summary

**Decision**: 使用 eframe + egui 构建桌面GUI应用

**Rationale**:
1. 用户明确要求使用egui框架
2. egui是Rust原生GUI框架，与现有项目语言一致
3. eframe提供了完整的桌面应用入口
4. egui社区活跃，文档完善

**Alternatives Considered**:
- iced: 另一个纯Rust GUI框架，但egui更成熟且用户已指定
- tauri + web frontend: 需要额外前端技术栈，不符合现有Rust项目架构
- druid: 已停止维护

## Phase 1: Design

### UI Component Design

基于功能需求，设计以下UI组件：

1. **ProjectsScreen** (项目列表页面)
   - 显示所有项目卡片
   - 支持新建项目按钮
   - 支持选择项目进入详情

2. **ProjectDetailScreen** (项目详情页面)
   - 显示项目信息
   - 显示大纲概览
   - 显示章节进度
   - 操作按钮: 分析、生成大纲、生成章节、发布、一致性检查

3. **NewProjectDialog** (新建项目对话框)
   - 项目名称输入
   - 类型选择下拉框
   - 目标字数输入
   - 创建/取消按钮

4. **OutlineScreen** (大纲生成页面)
   - 前提输入框
   - 主题输入框
   - 生成按钮
   - 大纲显示区域

5. **ChapterGenerateScreen** (章节生成页面)
   - 章节范围选择
   - 生成进度显示
   - 章节内容展示

6. **PublishScreen** (发布页面)
   - 发布操作选择(创建/上传/提交)
   - 章节范围选择
   - 状态显示

7. **CheckScreen** (一致性检查页面)
   - 检查进度显示
   - 检查报告展示

### Integration Points

GUI模块需要与现有模块的集成：

1. **StorageService** - 项目数据持久化
2. **FeasibilityService** - 可行性分析
3. **OutlineService** - 大纲生成
4. **ChapterPlanningService** - 章节规划
5. **GenerationService** - 内容生成
6. **FanqieService** - 番茄平台发布
7. **ConsistencyService** - 一致性检查

### Async Handling

由于现有服务均为异步实现，GUI层需要处理：
- 使用 tokio 运行时执行异步操作
- 在UI线程中使用异步任务
- 显示加载状态和进度
