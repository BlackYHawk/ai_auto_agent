# Quickstart: 完善项目验证功能

## 1. 项目创建（需完整信息）

```bash
# 创建项目 - 必须提供所有必填字段
cargo run -- new "明朝创业记" \
  --summary "现代社畜穿越到明朝富商之子，从小开始创业致富" \
  --genre historical \
  --target 1000000
```

### 必填字段验证

| 字段 | 说明 | 验证规则 |
|------|------|----------|
| 标题 | 小说名称 | 1-100字符 |
| 简介 | 故事概述 | 10-2000字符 |
| 类型 | 小说分类 | 8种类型之一 |
| 目标字数 | 预期字数 | > 0 |

## 2. 可行性分析（基于真实数据）

```bash
# 分析某类型的市场可行性
cargo run -- feasibility --genre xianxia
```

输出包含:
- 番茄小说该分类热门作品数据
- 市场分析结论
- 创作建议

## 3. 大纲生成（含验证）

```bash
# 生成大纲 - 系统自动进行一致性检查
cargo run -- outline \
  --project-id <ID> \
  --premise "主角穿越明朝创业的故事"
```

自动验证:
- 大纲与类型一致性
- 角色名字版权检查

## 4. 一致性检查

```bash
# 手动运行一致性检查
cargo run -- check --project-id <ID>
```

检查项目:
- 大纲与项目信息一致性
- 角色名字版权
- 剧情逻辑连贯性

## 常见错误

### 缺少必填字段

```
Error: Missing required fields: summary, genre, target
```

### 目标字数无效

```
Error: target_word_count must be greater than 0
```

### 角色名字侵权警告

```
Warning: Character name "萧炎" may infringe copyright
Suggested alternatives: ["萧云", "萧天"]
```
