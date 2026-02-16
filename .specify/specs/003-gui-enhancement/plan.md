# Implementation Plan: GUI增强与项目管理系统

## Technical Context

### Current Stack
- **Language**: Rust
- **GUI Framework**: egui (v0.29)
- **Backend**: Rust with tokio async
- **Storage**: Local file system (JSON)

### Architecture
- CLI + GUI in same binary
- Services layer for business logic
- Models for data structures

### Entities (已有)
- NovelProject
- Chapter
- Outline
- FeasibilityReport

### New Entities Needed
- ProjectModelConfig (项目级模型配置)
- ImportConfig (导入配置)

## Feature Breakdown

### Phase 1: 项目列表与详情展示

#### P1.1: 项目列表页面
- 修改 projects screen 显示所有项目
- 扫描 ./projects/ 目录获取项目列表
- 显示卡片：名称、类型、进度、创建时间

#### P1.2: 项目详情页面
- 新增 project_detail screen
- 显示基本信息、可行性研究、大纲、章节列表

#### P1.3: 章节查看
- 新增 chapter_view screen
- 显示章节内容和元数据

### Phase 2: 小说导入功能

#### P2.1: 导入UI
- 新增 import_screen
- 文件选择器（支持TXT）
- 解析预览

#### P2.2: 导入逻辑
- 解析TXT文件（按章节分割）
- 生成项目结构
- 保存到projects目录

### Phase 3: LLM模型配置

#### P3.1: 设置页面
- 新增 settings_screen
- 全局LLM配置
- API Key输入
- 模型选择

#### P3.2: 项目级模型
- 在NovelProject添加 model_config 字段
- 项目详情页显示/切换模型

## File Structure

```
src/
├── gui/
│   ├── app.rs              # 新增 Screen::Settings
│   └── screens/
│       ├── projects.rs     # 修改：显示项目列表
│       ├── project.rs     # 新增/修改：项目详情
│       ├── chapter.rs    # 新增：章节查看
│       ├── import.rs     # 新增：导入页面
│       └── settings.rs    # 新增：设置页面
├── models/
│   └── novel.rs           # 新增：model_config字段
├── services/
│   └── import.rs          # 新增：导入服务
```

## Implementation Order

1. **项目列表** - 扫描projects目录，渲染卡片列表
2. **项目详情** - 加载并显示项目完整信息
3. **章节查看** - 显示章节内容
4. **小说导入** - 解析TXT，生成项目
5. **设置页面** - LLM全局配置
6. **项目模型** - 项目级模型切换

## Dependencies

- egui (已有)
- tokio (已有)
- serde (已有)

## Notes

- 使用现有StorageService扫描项目
- 导入功能需要处理编码（UTF-8）
- 模型配置保存到config.toml
