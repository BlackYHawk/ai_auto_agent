# Research: Egui UI Implementation

## Technology Decision

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

## Integration Patterns

### CLI to GUI Integration

研究现有CLI模块与GUI的集成方式：

1. **复用services模块**
   - 现有services (Storage, Generation, Fanqie等) 可直接被GUI调用
   - 需要处理异步到同步的转换（tokio runtime）

2. **复用models模块**
   - 项目数据模型保持不变
   - GUI使用相同的序列化/反序列化逻辑

3. **状态管理**
   - 使用egui的App状态管理模式
   - 屏幕间通过App状态共享数据

## UI Architecture

### Multi-Window Support

egui支持多窗口模式，但推荐使用单窗口+多面板设计：

- **侧边栏**: 导航菜单
- **主区域**: 当前屏幕内容
- **底部**: 状态栏/进度显示

### Component Patterns

1. **Screen Trait**: 每个页面实现独立的update/render逻辑
2. **Component**: 可复用的UI组件（按钮、表单、列表等）
3. **Dialog**: 模态对话框用于快速操作

## Async Handling in Egui

egui是同步框架，需要特殊处理异步操作：

1. **tokio spawn**: 在后台线程执行异步任务
2. **Channels**: 使用mpsc channel在异步任务和UI之间通信
3. **状态轮询**: UI定期检查任务状态并更新显示

## File Structure Recommendation

```
src/gui/
├── main.rs           # GUI入口点 (fn main调用 eframe::run_native)
├── app.rs            # App结构体，实现egui::App trait
├── screens/
│   ├── mod.rs
│   ├── projects.rs   # 项目列表
│   ├── project.rs    # 项目详情
│   ├── new.rs        # 新建项目
│   ├── outline.rs    # 大纲生成
│   ├── generate.rs   # 章节生成
│   ├── publish.rs    # 发布
│   └── check.rs      # 一致性检查
└── components/
    ├── mod.rs
    ├── project_card.rs
    ├── progress.rs
    └── form.rs
```

## Dependencies

```toml
# 添加到 Cargo.toml
eframe = "0.29"
egui = "0.29"
```

## Next Steps

1. 添加eframe依赖到Cargo.toml
2. 创建GUI入口点
3. 实现项目列表页面
4. 逐步添加其他功能页面
