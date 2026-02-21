# Implementation Plan: 完善项目验证功能

**Branch**: `004-improve-project-validation` | **Date**: 2026-02-21 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/004-improve-project-validation/spec.md`

## Summary

完善项目创建验证流程，实现以下功能：
1. 项目创建时必须提供标题、简介、类型、目标字数并进行验证
2. 可行性分析基于番茄小说网站真实数据
3. 大纲生成时验证与项目信息一致性
4. 角色名字版权检查

## Technical Context

**Language/Version**: Rust 2021 edition (stable)
**Primary Dependencies**: tokio, reqwest, serde, clap, anyhow
**Storage**: Local file system (projects/{id}/*.json)
**Testing**: cargo test, tokio-test, mockall
**Target Platform**: macOS/Linux CLI
**Project Type**: Single CLI tool (no GUI)
**Performance Goals**: CLI响应 < 1秒, 可行性分析 < 30秒
**Constraints**: 删除GUI相关代码, 仅保留CLI

## Constitution Check

GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Context Consistency | N/A | 本功能不涉及章节生成 |
| II. Outline-Driven Generation | ✅ Pass | 大纲一致性验证符合此原则 |
| III. Memory Management | N/A | 本功能不涉及长文本 |
| IV. Test-First Development | ✅ Pass | 需编写单元测试和集成测试 |
| V. Observability | ✅ Pass | 验证结果需记录日志 |
| VI. Modularity | ✅ Pass | 验证逻辑作为独立服务模块 |
| VII. Documentation Sync | ✅ Pass | CLI变更需同步README |

**Gate Result**: All gates pass - proceed with implementation

## Project Structure

### Documentation (this feature)

```
specs/004-improve-project-validation/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
└── tasks.md            # Phase 2 output (/speckit.tasks)
```

### Source Code (repository root)

```text
src/
├── cli/                 # CLI commands (existing)
│   ├── commands/        # new, outline, feasibility 等命令
│   └── mod.rs
├── config/              # 配置管理
├── models/              # 数据模型
│   ├── novel.rs         # NovelProject 结构
│   ├── outline.rs        # 大纲结构
│   └── validation.rs     # [NEW] 验证相关模型
├── services/            # 业务服务
│   ├── storage.rs       # 项目存储
│   ├── scraping.rs       # 网页抓取
│   ├── feasibility.rs   # 可行性分析
│   ├── outline.rs       # 大纲生成
│   └── validation.rs     # [NEW] 验证服务
│       ├── project.rs    # 项目验证
│       ├── copyright.rs # 版权检查
│       └── consistency.rs # 一致性检查
├── lib.rs
└── main.rs

tests/
├── unit/
│   └── test_validation.rs   # [NEW] 验证单元测试
├── integration/
│   └── test_validation.rs   # [NEW] 验证集成测试
└── contract/
```

**Structure Decision**: 使用现有项目结构，新增 validation 模块处理验证逻辑

## Phase 0: Research

### Research Tasks

1. **Character Name Copyright Database**
   - 如何存储和查询知名小说角色名字
   - 本地JSON文件 vs 向量数据库
   - 覆盖率目标: Top 100 知名小说

2. **Fanqie Website Scraping**
   - 当前已有的 scraping.rs 实现
   - 需要扩展支持分类页面抓取
   - 反爬策略应对

3. **Outline Consistency Validation**
   - 基于类型关键词匹配
   - LLM辅助验证方案

### Output: research.md

(Research findings will be documented in research.md)

## Phase 1: Design

### Data Model Changes

1. **NovelProject** - 扩展必填字段验证
2. **ValidationResult** - 验证结果结构
3. **CopyrightCheckResult** - 版权检查结果
4. **ConsistencyCheckResult** - 一致性检查结果

### API Contracts

- CLI命令参数验证
- 验证服务接口

### Output: data-model.md, contracts/, quickstart.md

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| - | - | - |

No complexity violations identified.
