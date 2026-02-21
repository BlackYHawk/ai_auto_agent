# Research: 完善项目验证功能

## 1. Character Name Copyright Database

### Decision: Local JSON File

**Rationale**:
- Top 100 知名小说角色数量有限，本地JSON足够
- 无需额外依赖向量数据库
- 易于维护和更新

**Alternatives Considered**:
- Vector database (Qdrant): 过度设计，角色名字是精确匹配
- Remote API: 增加网络依赖和延迟

**Implementation**:
```json
// data/copyright_characters.json
{
  "fantasy": ["萧炎", "唐三", "林动", ...],
  "xianxia": ["韩立", "张小凡", "陆雪琪", ...],
  "urban": ["陈北冥", "叶凡", ...]
}
```

## 2. Fanqie Website Scraping

### Decision: 扩展现有 scraping.rs + reqwest

**Rationale**:
- 项目已有 scraping.rs 实现
- reqwest 已处理 HTTP 请求
- 番茄小说分类页面结构相对稳定

**Alternatives Considered**:
- Playwright: 过于重量级，反爬场景不需要JS渲染
- 第三方API: 增加成本和数据依赖

**Implementation**:
- 扩展 `ScrapingService` 支持分类页面
- 添加缓存机制（24小时过期）
- 失败时返回降级数据

## 3. Outline Consistency Validation

### Decision: 关键词匹配 + LLM 辅助

**Rationale**:
- 关键词匹配实现简单，效果直接
- LLM 可处理复杂一致性判断
- 两者结合提高准确率

**Alternatives Considered**:
- 纯规则匹配: 难以覆盖所有边界情况
- 纯LLM验证: 成本高，延迟大

**Implementation**:
- 类型关键词库（如仙侠: 修炼、灵石、宗门等）
- 大纲生成后运行一致性检查
- 返回匹配度评分和警告

## Summary

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Copyright DB | Local JSON | 简单、精确匹配、无额外依赖 |
| Fanqie Scraping | reqwest扩展 | 复用现有代码、轻量级 |
| Consistency | 关键词+LLM | 平衡准确率和成本 |
