# Feature Specification: GUI增强与项目管理系统

## 1. Overview/Context

本项目是一个基于Rust的AI小说自动生成Agent系统，当前已具备基本的小说生成能力。为了提升用户体验，需要增强GUI功能，实现项目可视化管理和多模型支持。

## 2. Problem Statement

当前系统存在以下问题：
- GUI无法显示已创建的项目列表
- 缺乏项目详情、可行性研究、大纲、章节的展示界面
- 无法导入已有的外部小说内容
- LLM模型配置需要通过配置文件手动修改
- 无法针对不同项目切换不同的LLM模型

## 3. User Stories

### P0 - 核心功能

**用户故事1：项目列表展示**
- 作为用户，我希望在GUI中看到所有已创建的项目列表
- 以便快速了解当前所有小说项目状态

**用户故事2：项目详情查看**
- 作为用户，我希望点击项目后能看到完整的项目信息
- 包括：项目名称、类型、目标字数、当前进度、可行性研究、大纲、章节列表

**用户故事3：章节内容查看**
- 作为用户，我希望查看每个章节的详细内容
- 包括章节标题，正文、字数、状态

### P1 - 导入功能

**用户故事4：小说导入**
- 作为用户，我希望将已有的章节内容导入到系统中
- 系统自动格式化为项目结构，支持继续编辑和生成

### P2 - 配置功能

**用户故事5：全局模型配置**
- 作为用户，我希望在GUI中配置默认使用的LLM模型
- 不再需要手动修改配置文件

**用户故事6：项目级模型切换**
- 作为用户，我希望为特定项目选择不同的LLM模型
- 根据项目需求灵活切换模型

## 4. Functional Requirements

### 4.1 项目列表功能

- [ ] FR1.1: 在首页展示所有项目卡片列表
- [ ] FR1.2: 每个卡片显示：项目名称、类型、进度百分比、创建时间
- [ ] FR1.3: 支持按创建时间或名称排序
- [ ] FR1.4: 支持搜索项目

### 4.2 项目详情功能

- [ ] FR2.1: 点击项目卡片进入详情页
- [ ] FR2.2: 显示项目基本信息（名称、类型、目标字数、当前字数）
- [ ] FR2.3: 显示可行性研究报告摘要
- [ ] FR2.4: 显示小说大纲（章节规划）
- [ ] FR2.5: 显示章节列表（带分页）

### 4.3 章节查看功能

- [ ] FR3.1: 点击章节查看完整内容
- [ ] FR3.2: 显示章节标题和正文
- [ ] FR3.3: 显示章节元数据（字数、状态、创建时间）
- [ ] FR3.4: 支持编辑章节内容

### 4.4 小说导入功能

- [ ] FR4.1: 支持导入TXT格式的小说文件
- [ ] FR4.2: 系统自动按章节分割内容
- [ ] FR4.3: 生成项目基本信息（可编辑）
- [ ] FR4.4: 导入后可在系统中继续编辑和生成

### 4.5 LLM模型配置

- [ ] FR5.1: 在设置页面配置全局默认模型
- [ ] FR5.2: 支持选择：Qwen、MiniMax、OpenAI
- [ ] FR5.3: 可配置API Key和模型参数

### 4.6 项目级模型切换

- [ ] FR6.1: 在项目详情页显示当前使用模型
- [ ] FR6.2: 支持为项目切换不同模型
- [ ] FR6.3: 模型切换后，后续生成使用新模型

## 5. Non-Functional Requirements

- [ ] NFR1: 所有界面支持中文显示
- [ ] NFR2: 项目列表加载时间不超过2秒
- [ ] NFR3: 章节内容渲染流畅，无明显卡顿

## 6. User Scenarios & Testing

### 场景1：查看项目列表
1. 用户打开应用
2. 系统显示所有项目卡片
3. 用户浏览项目状态

### 场景2：查看项目详情
1. 用户点击项目卡片
2. 系统显示项目详情页
3. 用户查看大纲和章节

### 场景3：导入小说
1. 用户点击"导入项目"按钮
2. 选择TXT文件
3. 系统解析并创建项目
4. 用户编辑项目信息

### 场景4：切换模型
1. 用户进入项目详情页
2. 点击"切换模型"
3. 选择新模型
4. 后续生成使用新模型

## 7. Edge Cases

- 项目列表为空时显示提示
- 导入文件格式不正确时提示错误
- API Key未配置时提示用户配置
- 网络请求失败时显示错误信息

## 8. Success Criteria

- [ ] SC1: 用户可在5秒内完成项目列表到详情的查看
- [ ] SC2: 导入功能成功率100%（格式正确的TXT文件）
- [ ] SC3: 模型切换在1秒内完成
- [ ] SC4: 所有用户可见文本均为中文

## 9. Key Entities

### Project
- id: UUID
- name: String
- genre: Enum
- target_word_count: u64
- current_word_count: u64
- status: Enum
- created_at: DateTime

### Chapter
- id: UUID
- project_id: UUID
- chapter_number: u32
- title: String
- content: String
- word_count: u32
- status: Enum

### FeasibilityReport
- id: UUID
- project_id: UUID
- genre_analysis: JSON
- market_score: f32
- content_score: f32
- feasibility_score: f32
- recommendation: String

### Outline
- id: UUID
- project_id: UUID
- premise: String
- theme: String
- chapters: Vec<ChapterPlan>

### LLMConfig
- provider: String
- api_key: String
- model: String
- group_id: Option<String>
- temperature: f32
- max_tokens: u32
