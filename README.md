# AI Novel Agent

AI驱动的网络小说生成系统，支持百万字级别的长篇小说创作与发布。

## 功能特性

- **项目创建**: 支持多种小说类型（玄幻、都市、仙侠、历史、言情、科幻、游戏、恐怖）
- **可行性分析**: 分析指定类型小说的市场潜力和创作可行性
- **大纲生成**: 基于创意生成完整小说大纲
- **章节规划**: 智能生成章节结构和大纲
- **内容生成**: AI辅助生成章节内容
- **一致性检查**: 自动检测角色、剧情前后一致性问题
- **平台发布**: 支持发布到番茄小说平台
- **双模式交互**: CLI 命令行模式 + GUI 图形界面

## 技术栈

- **语言**: Rust 2021
- **GUI框架**: eframe (egui)
- **CLI解析**: clap
- **异步运行时**: tokio
- **HTTP客户端**: reqwest

## 安装

```bash
# 编译项目
cargo build --release
```

## 使用方式

### CLI 模式

```bash
# 创建新项目（必需：标题、简介、类型、目标字数）
cargo run -- new "小说名称" --summary "小说简介（10-2000字符）" --genre fantasy --target 1000000

# 可行性分析（基于番茄小说真实数据）
cargo run -- feasibility --genre fantasy

# 生成大纲（自动使用项目summary，自动进行一致性检查和版权检查）
cargo run -- outline --project-id <ID>
# 也可手动指定premise
cargo run -- outline --project-id <ID> --premise "小说简介"

# 生成章节计划
cargo run -- plan --project-id <ID>

# 生成章节内容
cargo run -- generate --project-id <ID> --chapters "1-10"

# 一致性检查
cargo run -- check --project-id <ID>

# 发布到番茄小说
cargo run -- publish --project-id <ID> create
cargo run -- publish --project-id <ID> upload --chapters "1-10"
cargo run -- publish --project-id <ID> submit --chapters "1-10"
```

### GUI 模式

```bash
cargo run -- gui
```

启动图形界面，支持更直观的项目管理、大纲编辑和内容生成。

## 命令行参数

| 命令 | 说明 | 短选项 | 参数 |
|------|------|--------|------|
| `new` | 创建新项目 | `-s`, `-g`, `-t` | `name`, `--summary`, `--genre`, `--target` |
| `feasibility` | 可行性分析 | `-i`, `-g` | `--genre`, `--project-id` (可选) |
| `outline` | 生成大纲 | `-i`, `-m`, `-t`, `-w`, `-g` | `--project-id`, `[--premise]`, `--theme`, `--target`, `--genre` |
| `plan` | 生成章节计划 | `-i` | `--project-id` |
| `generate` | 生成章节 | `-i`, `-c` | `--project-id`, `--chapters` |
| `publish` | 发布到番茄 | `-i` | `--project-id`, `create\|upload\|submit` |
| `check` | 一致性检查 | `-i` | `--project-id` |
| `gui` | 启动GUI | - | - |

### 短选项说明

- `-i, --project-id`: 项目ID
- `-m, --premise`: 小说简介/创意
- `-s, --summary`: 小说简介 (必填, 10-2000字符)
- `-t, --theme`: 小说主题
- `-w, --target`: 目标字数
- `-g, --genre`: 小说类型
- `-c, --chapters`: 章节范围 (如 "1-10")
- `-p, --project-id`: 项目ID (feasibility/publish 命令)
- `-c, --config`: 配置文件路径 (全局)
- `-v, --verbose`: 详细日志 (可叠加: `-vvv`)

### 通用参数

- `-c, --config <PATH>`: 配置文件路径 (默认: `config.toml`)
- `-v, --verbose`: 启用详细日志 (可叠加: `-vvv`)

## 项目结构

```
src/
├── cli/           # CLI 命令实现
│   └── commands/ # 具体命令
├── config/        # 配置管理
├── gui/           # GUI 实现
│   ├── components/  # UI 组件
│   └── screens/     # 界面页面
├── models/        # 数据模型
├── services/      # 业务服务
│   ├── feasibility.rs  # 可行性分析
│   ├── outline.rs       # 大纲生成
│   ├── chapter_planning.rs  # 章节规划
│   ├── generation.rs    # 内容生成
│   ├── consistency.rs   # 一致性检查
│   ├── fanqie.rs        # 番茄小说集成
│   ├── scraping.rs      # 网页抓取
│   └── storage.rs       # 数据存储
├── lib.rs         # 核心库
└── main.rs        # 入口点

tests/
├── unit/          # 单元测试
├── integration/   # 集成测试
└── contract/     # 契约测试
```

## 配置

项目支持通过 `config.toml` 配置文件进行配置:

```toml
[llm]
provider = "openai"  # 或其他LLM提供商
model = "gpt-4"
api_key = "${OPENAI_API_KEY}"  # 支持环境变量

[fanqie]
username = "${FANQIE_USERNAME}"
password = "${FANQIE_PASSWORD}"

[storage]
base_path = "projects"
```

## 测试

```bash
# 运行所有测试
cargo test

# 运行单元测试
cargo test --unit

# 运行集成测试
cargo test --integration

# 运行契约测试
cargo test --contract
```

## 开发

```bash
# 代码检查
cargo clippy

# 格式化
cargo fmt
```

## 支持的小说类型

- `fantasy` - 玄幻
- `urban` - 都市
- `xianxia` - 仙侠
- `historical` - 历史
- `romance` - 言情
- `scifi` - 科幻
- `game` - 游戏
- `horror` - 恐怖

## License

MIT
