# Data Model: AI Novel Generation Agent

**Feature**: 001-ai-novel-gen
**Date**: 2026-02-14

---

## Core Entities

### 1. NovelProject

Represents a complete novel generation effort.

```typescript
interface NovelProject {
  id: string;                    // UUID
  name: string;                 // Novel title
  genre: NovelGenre;            // Genre category
  status: ProjectStatus;        // Current phase

  // Outline
  outline: NovelOutline | null;

  // Chapters
  chapters: Chapter[];

  // Publication
  fanqieNovelId: string | null;  // Fanqie platform novel ID
  publicationStatus: PublicationStatus;

  // Metadata
  createdAt: Date;
  updatedAt: Date;
  targetWordCount: number;       // Default: 1,000,000
}

enum ProjectStatus {
  DRAFT = "draft",              // Initial creation
  FEASIBILITY = "feasibility",  // Market analysis
  OUTLINE = "outline",          // Outline generation
  PLANNING = "planning",        // Chapter planning
  GENERATING = "generating",     // Content generation
  PUBLISHING = "publishing",    // Auto-upload in progress
  COMPLETED = "completed",      // All chapters done
}

enum PublicationStatus {
  NOT_PUBLISHED = "not_published",
  CREATED = "created",          // Novel created on Fanqie
  PUBLISHING = "publishing",    // Chapters being uploaded
  PUBLISHED = "published",      // All chapters published
}
```

### 2. FeasibilityReport

Market analysis document for a genre.

```typescript
interface FeasibilityReport {
  id: string;
  projectId: string;

  // Market Analysis
  genre: NovelGenre;
  totalWorksInGenre: number;
  averageViewsTop100: number;
  averageFavoritesTop100: number;
  trendScore: number;           // -1 to 1

  // Competitive Analysis
  topWorks: CompetitiveWork[];
  marketGaps: string[];

  // Scoring
  scores: {
    marketViability: number;    // 0-100
    competitionLevel: 'low' | 'medium' | 'high';
    differentiationPotential: number;
  };

  recommendation: 'proceed' | 'revise' | 'reject';
  suggestedAngles: string[];

  generatedAt: Date;
}

interface CompetitiveWork {
  title: string;
  author: string;
  views: number;
  favorites: number;
  rating: number;
  uniqueElements: string[];
  tags: string[];
}
```

### 3. NovelOutline

Structured story blueprint.

```typescript
interface NovelOutline {
  id: string;
  projectId: string;

  // Story Structure
  premise: string;              // 1-2 sentence hook
  theme: string;               // Central theme
  targetWordCount: number;

  // Plot Arcs
  arcs: PlotArc[];

  // Character Arcs
  protagonist: CharacterArc;
  supportingCharacters: CharacterArc[];

  // World Building
  worldSettings: WorldSetting;

  // Validation
  sensitiveContentCheck: SensitiveContentResult;

  status: 'draft' | 'approved' | 'locked';
  createdAt: Date;
  updatedAt: Date;
}

interface PlotArc {
  id: string;
  name: string;                 // e.g., "Act 1: Setup"
  startChapter: number;
  endChapter: number;
  summary: string;
  keyEvents: string[];
  climax: string;
}

interface CharacterArc {
  id: string;
  name: string;
  role: 'protagonist' | 'supporting' | 'antagonist';
  description: string;
  personalityTraits: string[];
  arcDescription: string;        // Character development arc
  keyMoments: CharacterMoment[];
}

interface CharacterMoment {
  chapter: number;
  description: string;
  development: string;
}

interface WorldSetting {
  name: string;
  type: 'modern' | 'fantasy' | 'scifi' | 'historical' | 'xianxia';
  description: string;
  rules: string[];              // World rules/magic system
  locations: Location[];
}

interface Location {
  name: string;
  description: string;
  importance: 'major' | 'minor';
}

interface SensitiveContentResult {
  passed: boolean;
  issues: SensitiveContentIssue[];
}

interface SensitiveContentIssue {
  category: 'violence' | 'explicit' | 'political' | 'other';
  description: string;
  severity: 'low' | 'medium' | 'high';
  suggestion: string;
}
```

### 4. ChapterPlan

Detailed breakdown of individual chapters.

```typescript
interface ChapterPlan {
  id: string;
  projectId: string;

  // Structure
  totalChapters: number;
  chapters: ChapterSummary[];

  // Pacing Validation
  plotTwistPositions: number[]; // Every 10 chapters validated

  status: 'draft' | 'approved' | 'locked';
  createdAt: Date;
  updatedAt: Date;
}

interface ChapterSummary {
  number: number;
  title: string;
  summary: string;
  keyEvents: string[];
  protagonistDevelopment: string;
  wordCountEstimate: number;
  isPlotTwistChapter: boolean;   // Every 10th chapter
  plotTwistDescription?: string;
}
```

### 5. GeneratedChapter

Actual novel content with metadata.

```typescript
interface GeneratedChapter {
  id: string;
  projectId: string;
  chapterNumber: number;

  // Content
  title: string;
  content: string;
  wordCount: number;

  // Context for consistency
  contextUsed: ContextReference[];

  // Generation metadata
  generationParams: {
    model: string;
    temperature: number;
    maxTokens: number;
  };

  // Quality checks
  consistencyCheck: ConsistencyCheckResult;
  outlineAlignment: number;      // 0-100

  // Status
  status: 'draft' | 'review' | 'approved' | 'published';
  fanqieChapterId: string | null;

  createdAt: Date;
  updatedAt: Date;
}

interface ContextReference {
  type: 'chapter' | 'character' | 'plot' | 'world';
  referenceId: string;
  relevanceScore: number;
}

interface ConsistencyCheckResult {
  passed: boolean;
  issues: ConsistencyIssue[];
}

interface ConsistencyIssue {
  type: 'character' | 'plot' | 'setting' | 'timeline';
  description: string;
  chapterReference: number;
}
```

### 6. UserCredentials

Fanqie platform authentication.

```typescript
interface UserCredentials {
  id: string;

  // Platform
  platform: 'fanqie';

  // Auth (encrypted storage required)
  cookie: string;               // Session cookie
  csrfToken: string | null;

  // Metadata
  username: string;
  lastValidated: Date;
  status: 'valid' | 'expired' | 'error';
}
```

---

## Enums

```typescript
enum NovelGenre {
  FANTASY = "fantasy",         // 玄幻
  URBAN = "urban",             // 都市
  XIANXIA = "xianxia",         // 仙侠
  HISTORICAL = "historical",   // 历史
  ROMANCE = "romance",         // 都市言情
  SCIFI = "scifi",             // 科幻
  GAME = "game",               // 游戏
  HORROR = "horror",           // 悬疑
  OTHER = "other",
}
```

---

## Relationships

```
NovelProject (1) ────── (1) FeasibilityReport
      │
      ├── (1) ────── (1) NovelOutline
      │                 │
      │                 ├── (*) PlotArc
      │                 ├── (*) CharacterArc
      │                 └── (1) WorldSetting
      │
      ├── (1) ────── (1) ChapterPlan
      │                 │
      │                 └── (*) ChapterSummary
      │
      └── (*) ────── (1) GeneratedChapter
```

---

## State Transitions

### Project Status Flow

```
DRAFT → FEASIBILITY → OUTLINE → PLANNING → GENERATING → PUBLISHING → COMPLETED
   ↑                                                            │
   └────────────────────────────────────────────────────────────┘
```

### Chapter Status Flow

```
DRAFT → REVIEW → APPROVED → PUBLISHED
  ↑                           │
  └───────────────────────────┘
```

---

## Storage

- **File-based**: JSON files in project directory
- **Database**: SQLite for larger projects (future)
- **Vector DB**: For semantic search (future phase)
