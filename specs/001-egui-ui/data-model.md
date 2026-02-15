# Data Model: Egui GUI

## Overview

GUI模块复用现有的数据模型，无需创建新模型。

## Existing Models (to be reused)

### NovelProject

从 `src/models/novel.rs` 复用，包含：
- id: UUID
- name: String
- genre: NovelGenre
- target_word_count: u64
- current_word_count: u64
- created_at: DateTime
- updated_at: DateTime
- outline: Option<Outline>
- chapters: Vec<Chapter>

### NovelGenre

从 `src/models/novel.rs` 复用：
- Fantasy
- Urban
- Xianxia
- Historical
- Romance
- Scifi
- Game
- Horror

### Outline

从 `src/models/outline.rs` 复用：
- chapters: Vec<ChapterOutline>

### ChapterOutline

从 `src/models/outline.rs` 复用：
- number: u32
- title: String
- summary: String
- word_count_estimate: u64

### Chapter

从 `src/models/chapter.rs` 复用：
- number: u32
- title: String
- content: String
- status: ChapterStatus
- word_count: u64

### ChapterStatus

从 `src/models/chapter.rs` 复用：
- Pending
- Generating
- Completed
- Published

## GUI-Specific State

### AppState

GUI应用状态，存储在eframe App中：

```rust
struct AppState {
    // 当前视图
    current_screen: Screen,

    // 项目列表
    projects: Vec<NovelProject>,

    // 当前选中的项目
    selected_project_id: Option<Uuid>,

    // 异步任务状态
    running_tasks: HashMap<String, TaskState>,

    // 错误消息
    error_message: Option<String>,

    // 表单输入状态
    new_project_form: NewProjectForm,
    outline_form: OutlineForm,
    generate_form: GenerateForm,
}
```

### Screen Enum

表示当前显示的页面：

```rust
enum Screen {
    Projects,      // 项目列表
    ProjectDetail, // 项目详情
    NewProject,    // 新建项目
    Outline,       // 大纲生成
    Generate,      // 章节生成
    Publish,       // 发布
    Check,         // 一致性检查
}
```

### TaskState

表示异步任务状态：

```rust
enum TaskState {
    Idle,
    Running { progress: f32, message: String },
    Completed,
    Failed { error: String },
}
```

### Form Structures

#### NewProjectForm

```rust
struct NewProjectForm {
    name: String,
    genre: NovelGenre,
    target_word_count: String, // 字符串便于UI输入
}
```

#### OutlineForm

```rust
struct OutlineForm {
    premise: String,
    theme: Option<String>,
}
```

#### GenerateForm

```rust
struct GenerateForm {
    chapter_range: String, // e.g., "1-10"
}
```

#### PublishForm

```rust
struct PublishForm {
    action: PublishAction, // Create, Upload, Submit
    chapter_range: String,
}
```

## State Transitions

```
Projects -> NewProject -> Projects (after creation)
Projects -> ProjectDetail -> [Outline, Generate, Publish, Check]
ProjectDetail -> Outline -> ProjectDetail (after generation)
ProjectDetail -> Generate -> ProjectDetail (after generation)
ProjectDetail -> Publish -> ProjectDetail (after publishing)
ProjectDetail -> Check -> ProjectDetail (after checking)
```

## Validation Rules

基于spec.md中的功能需求：

1. **NewProjectForm**
   - name: 必填，非空
   - genre: 必选，有效的NovelGenre
   - target_word_count: 必填，正整数

2. **OutlineForm**
   - premise: 必填，非空

3. **GenerateForm**
   - chapter_range: 必填，格式 "数字-数字" 或 "数字"
   - 章节范围必须在项目大纲范围内

4. **PublishForm**
   - action: 必选
   - chapter_range: 必填，格式同GenerateForm
   - 需要番茄平台配置

## Persistence

GUI模块不直接持久化数据，而是通过调用现有的StorageService进行数据操作：
- 加载项目列表: StorageService::load_all()
- 保存项目: StorageService::save()
- 加载单个项目: StorageService::load(project_id)
