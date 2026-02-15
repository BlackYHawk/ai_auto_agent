# API Contracts

**Feature**: AI Novel Generation Agent
**Date**: 2026-02-14

---

## 1. Novel Project Management

### Create Project

```
POST /api/projects

Request:
{
  name: string,           // Novel title
  genre: NovelGenre,      // Genre enum
  targetWordCount?: number // Default: 1,000,000
}

Response (201):
{
  id: string,
  name: string,
  genre: NovelGenre,
  status: "draft",
  createdAt: Date
}
```

### Get Project

```
GET /api/projects/:id

Response (200):
{
  id: string,
  name: string,
  genre: NovelGenre,
  status: ProjectStatus,
  outline: NovelOutline | null,
  chapterCount: number,
  fanqieNovelId: string | null,
  publicationStatus: PublicationStatus,
  createdAt: Date,
  updatedAt: Date
}
```

### List Projects

```
GET /api/projects

Response (200):
{
  projects: NovelProject[],
  total: number
}
```

---

## 2. Feasibility Analysis

### Generate Feasibility Report

```
POST /api/projects/:id/feasibility

Request:
{
  genre: NovelGenre
}

Response (202):
{
  id: string,
  projectId: string,
  status: "processing",
  progress: 0
}

Webhook/Polling:
GET /api/feasibility/:id

Response (200):
{
  id: string,
  projectId: string,
  genre: NovelGenre,
  totalWorksInGenre: number,
  averageViewsTop100: number,
  scores: {
    marketViability: number,
    competitionLevel: 'low' | 'medium' | 'high',
    differentiationPotential: number
  },
  recommendation: 'proceed' | 'revise' | 'reject',
  status: "completed"
}
```

---

## 3. Outline Generation

### Generate Outline

```
POST /api/projects/:id/outline

Request:
{
  premise: string,         // 1-2 sentence hook
  theme: string,          // Central theme
  characterIdeas?: string,
  worldIdeas?: string
}

Response (202):
{
  id: string,
  projectId: string,
  status: "processing"
}

GET /api/outline/:id

Response (200):
{
  id: string,
  projectId: string,
  premise: string,
  theme: string,
  arcs: PlotArc[],
  protagonist: CharacterArc,
  supportingCharacters: CharacterArc[],
  worldSettings: WorldSetting,
  sensitiveContentCheck: SensitiveContentResult,
  status: "draft"
}
```

### Approve Outline

```
POST /api/outline/:id/approve

Response (200):
{
  id: string,
  status: "approved"
}
```

---

## 4. Chapter Planning

### Generate Chapter Plan

```
POST /api/projects/:id/chapters/plan

Response (202):
{
  id: string,
  projectId: string,
  status: "processing"
}

GET /api/chapter-plan/:id

Response (200):
{
  id: string,
  projectId: string,
  totalChapters: number,
  chapters: ChapterSummary[],
  plotTwistPositions: number[],
  status: "draft"
}
```

### Approve Chapter Plan

```
POST /api/chapter-plan/:id/approve

Response (200):
{
  id: string,
  status: "locked"
}
```

---

## 5. Chapter Generation

### Generate Chapter

```
POST /api/projects/:id/chapters/:chapterNumber/generate

Request:
{
  regenerate?: boolean    // Regenerate existing chapter
}

Response (202):
{
  id: string,
  chapterNumber: number,
  status: "processing"
}

GET /api/chapters/:id

Response (200):
{
  id: string,
  projectId: string,
  chapterNumber: number,
  title: string,
  content: string,
  wordCount: number,
  consistencyCheck: ConsistencyCheckResult,
  outlineAlignment: number,
  status: "draft" | "review" | "approved"
}
```

### Batch Generate Chapters

```
POST /api/projects/:id/chapters/batch

Request:
{
  startChapter: number,
  endChapter: number
}

Response (202):
{
  jobId: string,
  status: "processing",
  progress: {
    completed: number,
    total: number
  }
}
```

---

## 6. Fanqie Platform Integration

### Save Credentials

```
POST /api/credentials

Request:
{
  platform: "fanqie",
  cookie: string,
  username: string
}

Response (201):
{
  id: string,
  platform: "fanqie",
  username: string,
  status: "valid"
}
```

### Create Novel on Fanqie

```
POST /api/projects/:id/fanqie/create

Request:
{
  title: string,
  genre: string,
  description: string,
  tags: string[]
}

Response (200):
{
  fanqieNovelId: string,
  fanqieNovelUrl: string,
  status: "created"
}
```

### Upload Chapter

```
POST /api/chapters/:id/publish

Request:
{
  title: string,
  content: string,
  autoSubmit?: boolean    // Submit for review after upload
}

Response (200):
{
  fanqieChapterId: string,
  status: "published" | "pending_review"
}
```

### Batch Upload

```
POST /api/projects/:id/fanqie/batch-publish

Request:
{
  chapters: number[]      // Chapter numbers to upload
}

Response (202):
{
  jobId: string,
  status: "processing",
  progress: {
    uploaded: number,
    total: number,
    failed: number[]
  }
}
```

---

## 7. Scoring

### Score Novel Concept

```
POST /api/projects/:id/score

Response (200):
{
  projectId: string,
  scores: {
    totalScore: number,
    marketScore: number,
    contentScore: number,
    feasibilityScore: number
  },
  recommendation: 'proceed' | 'revise' | 'reject',
  suggestedImprovements: string[]
}
```

### Get Score History

```
GET /api/projects/:id/scores

Response (200):
{
  scores: NovelScore[],
  currentScore: NovelScore
}
```

---

## 8. Context Management

### Search Context

```
POST /api/projects/:id/context/search

Request:
{
  query: string,
  topK?: number          // Default: 5
}

Response (200):
{
  results: {
    chapterNumber: number,
    content: string,
    relevanceScore: number
  }[]
}
```

### Check Consistency

```
POST /api/projects/:id/consistency/check

Response (200):
{
  projectId: string,
  passed: boolean,
  issues: ConsistencyIssue[]
}
```

---

## Error Responses

```json
{
  "error": {
    "code": "ERROR_CODE",
    "message": "Human readable message": {}
",
    "details  }
}
```

Common error codes:
- `PROJECT_NOT_FOUND`: Project does not exist
- `INVALID_STATUS`: Operation not allowed in current status
- `FEASIBILITY_REQUIRED`: Must complete feasibility before proceeding
- `OUTLINE_REQUIRED`: Must approve outline before planning
- `PLAN_REQUIRED`: Must approve chapter plan before generation
- `CREDENTIALS_INVALID`: Platform credentials expired
- `PLATFORM_ERROR`: External platform API error
- `CONTEXT_EXCEEDED`: Context window limit reached
