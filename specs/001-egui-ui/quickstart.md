# Quickstart: Egui GUI Application

## Building the GUI

### Prerequisites

- Rust 2021 edition
- cargo

### Build Commands

```bash
# Build GUI binary
cargo build --release --package ai-novel-agent

# Or run in development mode
cargo run --package ai-novel-agent
```

### Dependencies

Add to `Cargo.toml`:

```toml
[dependencies]
eframe = "0.29"
egui = "0.29"
```

## Running the Application

### Binary Name

GUI应用通过以下方式运行：

```bash
# 运行CLI（已有）
cargo run --release --package ai-novel-agent -- new "我的小说" --genre fantasy

# GUI模式
cargo run -- gui

# 或使用release模式
cargo run --release -- gui
```

### Configuration

GUI应用使用与CLI相同的配置文件 `config.toml`：
- LLM API配置
- 番茄平台Cookies（用于发布功能）

## Usage Guide

### First Launch

1. 启动应用程序
2. 默认显示项目列表页面
3. 如果没有项目，点击"新建项目"按钮

### Creating a Project

1. 在项目列表页面，点击"新建项目"
2. 填写项目名称（如"我的玄幻小说"）
3. 选择类型（玄幻/都市/仙侠等）
4. 输入目标字数（默认100万字）
5. 点击"创建"按钮

### Generating Content

1. 选择项目进入详情页
2. **生成大纲**: 输入小说前提和主题，点击"生成大纲"
3. **生成章节**: 输入章节范围（如1-10），点击"生成章节"
4. 等待生成完成，查看结果

### Publishing

1. 在项目详情页，点击"发布"
2. 选择发布操作（创建小说/上传章节/提交审核）
3. 选择章节范围
4. 确认发布

### Consistency Check

1. 在项目详情页，点击"一致性检查"
2. 等待检查完成
3. 查看检查报告

## Troubleshooting

### GUI doesn't start

- 检查Rust环境: `rustc --version`
- 检查依赖编译: `cargo check`
- 查看错误信息

### Project not loading

- 检查config.toml配置
- 检查项目目录权限

### Generation fails

- 检查LLM API配置
- 检查网络连接
- 查看详细错误信息

## Architecture Overview

```
src/
├── gui/              # 新增GUI模块
│   ├── mod.rs        # 模块入口
│   ├── app.rs        # App状态管理
│   ├── screens/      # 页面组件
│   └── components/   # 复用组件
├── models/           # 复用现有模型
├── services/         # 复用现有服务
└── cli/              # 保留CLI模块

main.rs               # CLI和GUI共用入口，通过 gui 子命令启动
```

## Testing

```bash
# 运行所有测试
cargo test

# 只运行GUI模块测试（创建后）
cargo test --package gui
```
