# AI Novel Agent Constitution

## Core Principles

### I. Context Consistency (NON-NEGOTIABLE)
The system MUST maintain narrative consistency across all generated chapters. This includes: character personality traits, plot threads, world-building details, and thematic elements. Every generation request must have access to relevant context from prior chapters. Violations of consistency constitute critical bugs requiring immediate resolution.

### II. Outline-Driven Generation
All novel content MUST be generated according to the project outline. The outline serves as the single source of truth for story structure, chapter placement, and plot development. The system must autonomously update the outline when new plot developments emerge during generation. Divergence from outline requires explicit justification and documentation.

### III. Memory Management for Long-Form Content
The system MUST efficiently handle context windows exceeding one million characters. This requires: chunked context retrieval, semantic compression of historical content, priority-based context inclusion, and graceful degradation when context exceeds model limits. Memory management is a core architectural concern, not an afterthought.

### IV. Test-First Development (NON-NEGOTIABLE)
TDD is mandatory for all features: Tests written → Tests fail → Then implement. Red-Green-Refactor cycle strictly enforced. Tests must cover context consistency verification, outline alignment validation, and generation quality metrics. No feature is complete without corresponding test coverage.

### V. Observability & Reproducibility
Every generation operation MUST be logged with sufficient context for reproduction. This includes: outline state, chapter history, generation parameters, and seed values. Structured logging required for all agent operations. Generation outputs must be traceable to their input state.

### VI. Modularity & Reusability
Core components MUST be designed as reusable libraries: context management, outline handling, generation strategies, and quality assessment. Each library must be independently testable and documented. No organizational-only modules without clear functional purpose.

## Technical Constraints

### Rust Implementation
The project MUST be implemented in Rust for performance and memory safety. Rust version must track stable releases. All code must pass `cargo clippy` with zero warnings. Dependencies must be audited for security and maintenance status.

### Performance Standards
- Generation latency: < 30 seconds per chapter (10k characters)
- Memory usage: < 2GB for typical novel session
- Context retrieval: < 500ms for semantic search
- Startup time: < 3 seconds cold start

### Quality Gates
- 80% minimum test coverage on core modules
- All tests must pass before merge
- No `unsafe` code without documented justification
- Documentation required for all public APIs

## Development Workflow

### Code Review Requirements
All PRs must verify:
1. Context consistency tests included/updated
2. Outline alignment verified
3. Performance impact assessed
4. Documentation updated

### Complexity Justification
Any architectural complexity must be justified with:
- Specific problem being solved
- Simpler alternative considered and rejected
- Migration path documented

## Governance

This constitution supersedes all other practices. Amendments require:
1. Draft proposal with rationale
2. Impact assessment on existing code
3. Migration plan if applicable
4. Documentation update

All team members must verify compliance with these principles in every PR.

**Version**: 1.0.0 | **Ratified**: 2026-02-14 | **Last Amended**: 2026-02-14
