# Research: AI Novel Generation Agent

**Date**: 2026-02-14
**Feature**: 001-ai-novel-gen

---

## 1. LLM Model Selection for Chinese Novel Generation

### Decision: Qwen2.5 (Primary) + OpenAI Claude (Fallback)

| Model | Chinese Quality | Context Window | Pricing | Rust Support |
|-------|---------------|----------------|---------|--------------|
| Qwen2.5 (Alibaba) | Excellent | 128K-1M | ~$0.40-1.20/M tokens | reqwest |
| ChatGLM4 (Zhipu) | Excellent | 128K-1M | ~$0.50/M tokens | reqwest |
| Claude 3.5 Sonnet | Very Good | 200K | $3.00/$15.00/M | reqwest |
| GPT-4o | Very Good | 128K | $5.00/$15.00/M | reqwest |

**Rationale**:
- Qwen2.5 and ChatGLM4 are optimized specifically for Chinese language
- Large context windows (128K-1M) essential for million-word novels
- Competitive pricing for high-volume generation
- Using reqwest for HTTP client (simple, flexible)

**Alternatives Considered**:
- OpenAI GPT-4o: Good but higher cost, potential latency
- Claude 3.5 Sonnet: Excellent reasoning, good fallback
- Local deployment: Too resource-intensive for initial version

---

## 2. Fanqie Platform Integration

### Finding: No Official Public API Available

**Status**: Fanqie Novel (番茄小说) does NOT have a documented public API for third-party integration.

### Integration Approaches

| Approach | Pros | Cons | Recommendation |
|----------|------|------|----------------|
| Web Scraping | Full control | Fragile, may break | Use with caution |
| Browser Automation | Reliable UI automation | Slower, requires browser | Recommended |
| Reverse Engineered API | Fast, programmatic | Legal/ethical concerns | Not recommended |

**Recommended Implementation**:
1. **Playwright/Selenium** for browser automation
2. Cookie-based authentication
3. Handle anti-bot measures gracefully

**Key URLs**:
- Reader: https://fanqienovel.com/
- Writer: https://fanqienovel.com/main/writer/

**Risk Mitigation**:
- Build abstraction layer for platform changes
- Log all interactions for debugging
- Implement retry with exponential backoff

---

## 3. Novel Scoring & Evaluation Mechanism

### Framework: Three-Dimensional Scoring

#### A. Market Viability (40%)

| Metric | Data Source | Weight |
|--------|-------------|--------|
| Genre Demand | Fanqie category work count | 15% |
| Competition Density | Top 100 average views | 15% |
| Trend Score | Recent growth in category | 10% |

#### B. Content Potential (35%)

| Metric | Evaluation Method | Weight |
|--------|------------------|--------|
| Originality | Trope combination uniqueness | 10% |
| Emotional Resonance | Genre-specific hooks | 10% |
| Character Appeal | Protagonist archetype strength | 10% |
| World Building | Setting depth potential | 5% |

#### C. Technical Feasibility (25%)

| Metric | Evaluation Method | Weight |
|--------|------------------|--------|
| Plot Scalability | Can sustain 300-500 chapters | 10% |
| Conflict Sustainability | Conflict source diversity | 10% |
| Consistency Feasibility | Context complexity | 5% |

### Scoring Output

```typescript
interface NovelScore {
  totalScore: number;        // 0-100
  recommendation: 'proceed' | 'revise' | 'reject';
  marketScore: number;
  contentScore: number;
  feasibilityScore: number;
  suggestedImprovements: string[];
}
```

**Thresholds**:
- Score >= 70: Proceed with generation
- Score 50-69: Revise concept, then re-score
- Score < 50: Reject, try different genre

---

## 4. Context Management for Long-Form Content

### Architecture Decision: Semantic Retrieval + Summary Compression

#### Strategy

1. **Semantic Search Layer**
   - Embeddings for all chapters
   - Vector database for similarity search
   - Retrieve top-K relevant chapters for context

2. **Context Compression**
   - Summarize older chapters (>100 ago)
   - Keep key plot points, character states
   - Compressed summaries in context window

3. **State Tracking**
   - Character status database
   - Plot thread tracker
   - World-building knowledge base

#### Tools (Rust Ecosystem)

| Component | Library | Purpose |
|-----------|---------|---------|
| Embeddings | qdrant-client | Vector storage/search |
| HTTP | reqwest | API calls |
| Database | sled or SQLite | Local state |
| Serialization | serde | Data handling |

---

## 5. Dependencies Final Selection

### Cargo.toml Dependencies

```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
tracing = "0.1"
tracing-subscriber = "0.3"
anyhow = "1.0"

# Optional for future phases
# qdrant = "1.0"     # Vector DB
# scraper = "0.6"    # HTML parsing
# playwright = "0.1" # Browser automation
```

---

## Summary

| Decision | Choice | Rationale |
|----------|--------|-----------|
| LLM Provider | Qwen2.5 + Claude fallback | Best Chinese quality, large context |
| Fanqie Integration | Browser automation | No official API available |
| Scoring | Three-dimensional framework | Market + Content + Feasibility |
| Context | Semantic retrieval + compression | Efficient for million-word scale |
| HTTP Client | reqwest | Simple, Rust-native |

---

## Next Steps

1. Design data model for novel entities
2. Define API contracts for external integrations
3. Create quickstart guide for users
