# Feature Specification: 完善项目验证功能

**Feature Branch**: `004-improve-project-validation`
**Created**: 2026-02-21
**Status**: Draft
**Input**: User description: "完善项目，创建项目时需要提供小说标题、简介、类型、目标字数；可行性分析需要根据真实番茄网站的内容去验证方可创建；生成的大纲需要跟项目简介、类型匹配，不能脱离，主角、配角的名字不能跟其他小说重复导致版权问题；"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - 项目创建时完整信息验证 (Priority: P1)

用户在创建新小说项目时，必须提供完整的信息：小说标题、简介、类型和目标字数。系统需要验证所有必填字段的完整性和合法性。

**Why this priority**: 确保项目数据完整性和一致性，是后续所有功能的基础

**Independent Test**: 可通过CLI命令独立测试：创建项目时缺少任意必填字段应被拒绝，提供完整信息应成功创建

**Acceptance Scenarios**:

1. **Given** 用户执行 `new` 命令，**When** 只提供名称而缺少简介、类型、目标字数，**Then** 系统拒绝创建并提示缺少必要字段
2. **Given** 用户执行 `new` 命令，**When** 提供完整信息（标题、简介、类型、目标字数），**Then** 系统成功创建项目并返回项目ID
3. **Given** 用户执行 `new` 命令，**When** 目标字数为负数或0，**Then** 系统拒绝创建并提示目标字数必须为正整数

---

### User Story 2 - 可行性分析基于真实数据验证 (Priority: P1)

在创建特定类型的小说项目前，系统需要通过抓取番茄小说平台的真实数据来验证该类型的可行性分析结果，确保分析基于真实市场数据。

**Why this priority**: 确保可行性分析结果可信，帮助用户做出正确的创作决策

**Independent Test**: 可独立测试可行性分析功能：输入指定类型，系统应返回基于番茄小说真实数据的分析结果

**Acceptance Scenarios**:

1. **Given** 用户执行 `feasibility` 命令，**When** 指定类型为"仙侠"，**Then** 系统抓取番茄小说仙侠类热门作品数据并生成可行性报告
2. **Given** 用户执行 `feasibility` 命令，**When** 网络请求失败或超时，**Then** 系统返回缓存数据或提示无法获取最新数据
3. **Given** 用户执行 `feasibility` 命令，**When** 输入不支持的类型，**Then** 系统返回错误提示支持的类型列表

---

### User Story 3 - 大纲与项目信息匹配验证 (Priority: P1)

生成的小说大纲必须与项目简介和类型匹配，系统需要验证大纲内容是否偏离了原始创意和类型设定。

**Why this priority**: 确保生成的内容符合用户预期，不偏离创作初衷

**Independent Test**: 可独立测试大纲生成功能：输入特定简介和类型，生成的大纲应与之一致

**Acceptance Scenarios**:

1. **Given** 用户执行 `outline` 命令，输入"都市商战"类型，**When** 系统生成大纲，**Then** 大纲内容应符合都市背景和商业竞争主题
2. **Given** 用户执行 `outline` 命令，**When** 生成的章节大纲与输入的类型明显不符（如仙侠类型出现现代科技），**Then** 系统应标记为一致性警告
3. **Given** 用户执行 `outline` 命令，**When** 用户提供了详细的简介，**Then** 大纲应围绕简介中的核心冲突展开

---

### User Story 4 - 角色名字版权检查 (Priority: P1)

生成主角和配角名字时，系统需要检查是否与现有知名小说角色重复，避免版权纠纷。

**Why this priority**: 避免因角色名字侵权导致法律问题，保护创作者权益

**Independent Test**: 可独立测试名字检查功能：输入可能侵权的名字应被标记

**Acceptance Scenarios**:

1. **Given** 大纲生成过程中，**When** 主角名字为"萧炎"（斗破苍穹主角），**Then** 系统应警告该名字可能侵权，建议更换
2. **Given** 大纲生成过程中，**When** 角色名字为常用虚构名字（如"张三"），**Then** 系统应允许使用
3. **Given** 用户坚持使用被警告的名字，**Then** 系统应记录用户确认并允许生成

---

### Edge Cases

- 番茄小说网站结构变化导致抓取失败时的降级处理策略
- 用户提供的简介过于简短导致大纲匹配度难以验证的处理方式
- 名字版权数据库无法覆盖所有知名角色时的免责声明

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: 系统必须在项目创建时要求提供小说标题、简介、类型和目标字数四个必填字段
- **FR-002**: 系统必须在创建项目前验证目标字数为正整数
- **FR-003**: 系统必须在执行可行性分析时从番茄小说网站抓取真实数据
- **FR-004**: 系统必须将抓取的数据与LLM分析结合，生成基于真实市场情况的可行性报告
- **FR-005**: 系统必须在大纲生成时验证大纲内容与项目类型的一致性
- **FR-006**: 系统必须在生成角色名字时检查是否与知名小说角色重复
- **FR-007**: 系统必须在检测到潜在版权问题时提供警告和替换建议
- **FR-008**: 系统必须在项目数据中记录所有验证结果和警告信息

### Key Entities

- **NovelProject**: 小说项目，包含标题、简介、类型、目标字数、创建时间等属性
- **FeasibilityReport**: 可行性分析报告，包含市场数据、分析结论、风险提示
- **CharacterNameCheck**: 角色名字检查结果，包含名字、是否重复、风险等级、建议
- **OutlineConsistencyCheck**: 大纲一致性检查结果，包含匹配度评分、偏离描述

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 用户创建项目时，缺少任意必填字段则在1秒内返回明确错误提示
- **SC-002**: 可行性分析在网络正常情况下应在30秒内返回基于番茄真实数据的报告
- **SC-003**: 大纲与类型一致性验证准确率达到90%以上
- **SC-004**: 角色名字版权检查应覆盖Top 100知名小说角色，误判率低于5%
- **SC-005**: 用户验证流程完成率从当前水平提升至95%以上

## Clarifications

### Session 2026-02-21

- Q: 是否需要实现GUI界面？ → A: 仅实现CLI，删除GUI相关代码
- Q: outline命令的premise参数是否需要改为使用项目创建时的summary？ → A: 是，outline命令不再需要--premise参数，直接使用NovelProject.summary

## Scope Constraints

- **Out of Scope**: GUI界面实现、图形化交互
- **Implementation Target**: 仅CLI命令行工具
