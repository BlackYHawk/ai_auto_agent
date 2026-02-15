# UI Contracts: Egui GUI

## Component Contracts

### ProjectsScreen

**Responsibility**: 显示项目列表，支持项目选择和新建

**Public API**:
```rust
fn update(&mut self, ctx: &egui::Context, projects: &[NovelProject])
```

**States**:
- Empty: 显示"暂无项目，请新建"提示
- Loading: 显示加载中
- Loaded: 显示项目卡片列表
- Error: 显示错误信息

### ProjectDetailScreen

**Responsibility**: 显示项目详情和操作按钮

**Public API**:
```rust
fn update(&mut self, ctx: &egui::Context, project: &NovelProject)
```

**Components**:
- 项目信息面板
- 操作按钮（分析/大纲/章节/发布/检查）
- 进度显示区域

### NewProjectDialog

**Responsibility**: 新建项目表单

**Public API**:
```rust
fn show(ctx: &egui::Context) -> Option<NewProjectForm>
fn validate(&self) -> Result<(), String>
```

**Fields**:
- name: String (required)
- genre: NovelGenre (required)
- target_word_count: u64 (required, > 0)

### OutlineScreen

**Responsibility**: 大纲生成界面

**Public API**:
```rust
fn update(&mut self, ctx: &egui::Context, project: &NovelProject) -> OutlineForm
```

**States**:
- Form: 输入前提和主题
- Generating: 显示进度
- Result: 显示生成的大纲

### ChapterGenerateScreen

**Responsibility**: 章节生成界面

**Public API**:
```rust
fn update(&mut self, ctx: &egui::Context, project: &NovelProject) -> GenerateForm
```

**States**:
- Form: 输入章节范围
- Generating: 显示进度条
- Result: 显示生成的章节

### PublishScreen

**Responsibility**: 发布到番茄平台

**Public API**:
```rust
fn update(&mut self, ctx: &egui::Context, project: &NovelProject) -> PublishForm
```

**Actions**:
- Create: 创建小说
- Upload: 上传章节
- Submit: 提交审核

### CheckScreen

**Responsibility**: 一致性检查

**Public API**:
```rust
fn update(&mut self, ctx: &egui::Context, project: &NovelProject)
```

**States**:
- Idle: 显示检查按钮
- Checking: 显示检查进度
- Result: 显示检查报告

## Integration Contracts

### Service Integration

GUI通过以下方式调用服务：

```rust
// 同步调用
let projects = StorageService::load_all(&storage_root)?;

// 异步调用（使用block_on）
let analysis = tokio::runtime::Runtime::new()?.block_on(
    FeasibilityService::analyze(&project_id, &genre)
)?;
```

### Error Handling

所有服务调用需要错误处理：

```rust
match service_call() {
    Ok(result) => display(result),
    Err(e) => show_error(e.to_string()),
}
```

## Theme and Styling

### Color Scheme

- Primary: 深蓝色 (#1E3A5F)
- Secondary: 浅灰色 (#F5F5F5)
- Accent: 橙色 (#FF8C00)
- Background: 白色 (#FFFFFF)
- Text: 深灰色 (#333333)

### Typography

- Headings: 18px bold
- Body: 14px regular
- Labels: 12px regular

### Layout

- Sidebar width: 200px
- Content padding: 16px
- Card spacing: 8px
- Button height: 32px
