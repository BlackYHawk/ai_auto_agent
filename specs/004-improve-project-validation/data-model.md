# Data Model: 完善项目验证功能

## 1. 现有模型扩展

### NovelProject (扩展)

```rust
pub struct NovelProject {
    pub id: Uuid,
    pub name: String,           // 小说标题
    pub summary: String,        // 小说简介
    pub genre: NovelGenre,      // 小说类型
    pub target_word_count: u64, // 目标字数
    pub created_at: DateTime,
    pub updated_at: DateTime,
    // ... 其他字段
}
```

**验证规则**:
- `name`: 1-100字符，非空
- `summary`: 10-2000字符，非空
- `genre`: 必须是支持的类型之一
- `target_word_count`: > 0 且 <= 10,000,000

## 2. 新增模型

### ValidationResult

```rust
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
}

pub struct ValidationError {
    pub field: String,
    pub message: String,
    pub code: ErrorCode,
}

pub enum ErrorCode {
    MissingField,
    InvalidFormat,
    OutOfRange,
    Duplicate,
}
```

### CopyrightCheckResult

```rust
pub struct CopyrightCheckResult {
    pub character_name: String,
    pub is_potential_duplicate: bool,
    pub risk_level: RiskLevel,
    pub suggested_alternatives: Vec<String>,
    pub source_work: Option<String>, // 可能的侵权来源
}

pub enum RiskLevel {
    Low,       // 可用
    Medium,    // 建议更换
    High,      // 不建议使用
}
```

### ConsistencyCheckResult

```rust
pub struct ConsistencyCheckResult {
    pub is_consistent: bool,
    pub score: f32,           // 0.0 - 1.0
    pub matched_keywords: Vec<String>,
    pub mismatched_elements: Vec<String>,
    pub warnings: Vec<String>,
}
```

### FeasibilityReport (扩展)

```rust
pub struct FeasibilityReport {
    pub genre: NovelGenre,
    pub market_data: MarketData,
    pub analysis: String,
    pub recommendation: Recommendation,
    pub created_at: DateTime,
    pub data_source: DataSource, // Fanqie, Cache, None
}

pub struct MarketData {
    pub total_books: u32,
    pub hot_books: Vec<HotBook>,
    pub average_word_count: u64,
    pub tags: Vec<String>,
}

pub enum DataSource {
    FanqieLive,
    FanqieCache,
    Fallback,
}
```

## 3. 数据关系

```
NovelProject
    ├── validation_results: ValidationResult (存储在项目中)
    ├── outline (大纲)
    │   └── character_checks: Vec<CopyrightCheckResult>
    └── feasibility_report: FeasibilityReport (可选)
```

## 4. 验证规则总结

| 实体 | 字段 | 规则 |
|------|------|------|
| NovelProject | name | required, 1-100 chars |
| NovelProject | summary | required, 10-2000 chars |
| NovelProject | genre | required, valid enum |
| NovelProject | target_word_count | required, > 0 |
| CharacterName | - | check against copyright DB |
| Outline | - | check consistency with genre |
