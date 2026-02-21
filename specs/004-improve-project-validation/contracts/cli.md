# CLI Contracts: 完善项目验证功能

## 1. new 命令扩展

### 输入参数

```bash
ai-novel-agent new <TITLE> --summary <SUMMARY> --genre <GENRE> --target <WORDS>
```

| 参数 | 必需 | 类型 | 验证 |
|------|------|------|------|
| TITLE | 是 | string | 1-100字符 |
| SUMMARY | 是 | string | 10-2000字符 |
| GENRE | 是 | enum | fantasy/urban/xianxia/historical/romance/scifi/game/horror |
| TARGET | 是 | u64 | > 0 |

### 输出

**成功**:
```json
{
  "success": true,
  "project_id": "uuid",
  "message": "Project created successfully"
}
```

**失败**:
```json
{
  "success": false,
  "errors": [
    {"field": "summary", "message": "Summary is required", "code": "MissingField"}
  ]
}
```

## 2. feasibility 命令

### 输入参数

```bash
ai-novel-agent feasibility --genre <GENRE> [--project-id <ID>]
```

### 输出

```json
{
  "success": true,
  "report": {
    "genre": "xianxia",
    "market_data": {
      "total_books": 15000,
      "hot_books": [...],
      "average_word_count": 800000
    },
    "analysis": "...",
    "recommendation": "recommended",
    "data_source": "FanqieLive"
  }
}
```

## 3. outline 命令扩展

### 输入参数

```bash
ai-novel-agent outline --project-id <ID> --premise <PREMISE> [--theme <THEME>] [--genre <GENRE>]
```

### 输出

```json
{
  "success": true,
  "outline": {
    "premise": "...",
    "theme": "...",
    "plot_arcs": [...],
    "protagonist": {...},
    "characters": [...]
  },
  "validation": {
    "consistency_check": {
      "is_consistent": true,
      "score": 0.95
    },
    "character_checks": [
      {
        "name": "萧炎",
        "risk_level": "High",
        "suggested_alternatives": ["萧云", "萧天"]
      }
    ]
  }
}
```

## 4. check 命令

### 输入参数

```bash
ai-novel-agent check --project-id <ID>
```

### 输出

```json
{
  "success": true,
  "checks": [
    {
      "type": "consistency",
      "passed": true,
      "details": "大纲与项目类型一致"
    },
    {
      "type": "copyright",
      "passed": true,
      "details": "所有角色名字检查完成"
    }
  ]
}
```
