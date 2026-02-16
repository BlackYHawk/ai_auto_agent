# Data Model: GUI增强功能

## 新增字段

### NovelProject 新增字段

```rust
// src/models/novel.rs

/// 项目级LLM模型配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectModelConfig {
    /// 是否启用项目级配置（否则使用全局配置）
    pub enabled: bool,
    /// 使用的模型提供商
    pub provider: Option<String>,
    /// 模型名称
    pub model: Option<String>,
}

impl Default for ProjectModelConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            provider: None,
            model: None,
        }
    }
}

// 在 NovelProject 中添加
pub struct NovelProject {
    // ... 现有字段
    /// 项目级模型配置
    pub model_config: ProjectModelConfig,
}
```

## 数据关系

```
ProjectList
  └── projects/<id>/project.json
        ├── analysis/feasibility.json
        ├── outline/outline.json
        └── chapters/
              ├── 1.json
              ├── 2.json
              └── ...
```

## 导入格式

### TXT文件解析规则

1. 按 "第X章" 或 "Chapter X" 分割章节
2. 每章节第一行为标题
3. 剩余内容为正文

### 导入生成的结构

```
projects/<new-id>/
├── project.json          # 项目信息
├── analysis/
│   └── feasibility.json # 空（待生成）
├── outline/
│   └── outline.json      # 空（待生成）
└── chapters/
    ├── 1.json
    ├── 2.json
    └── ...
```

## API Contracts

### GUI Services

```rust
// 项目列表服务
trait ProjectService {
    fn list_projects() -> Result<Vec<NovelProject>>;
    fn get_project(id: Uuid) -> Result<NovelProject>;
    fn update_project(project: NovelProject) -> Result<()>;
}

// 导入服务
trait ImportService {
    fn import_txt(path: &Path) -> Result<ImportPreview>;
    fn confirm_import(preview: ImportPreview) -> Result<Uuid>;
}

// 模型配置服务
trait ModelConfigService {
    fn get_global_config() -> Result<LlmConfig>;
    fn set_global_config(config: LlmConfig) -> Result<()>;
    fn get_project_config(project_id: Uuid) -> Result<ProjectModelConfig>;
    fn set_project_config(project_id: Uuid, config: ProjectModelConfig) -> Result<()>;
}
```
