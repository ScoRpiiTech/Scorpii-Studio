# SCORPII STUDIO — ARCHITECTURE DECISION RECORDS (ADR)

This document tracks all significant architectural decisions, alternatives evaluated, trade-offs, and approval owners.

---

## ADR-0001: Technical Foundation Candidate Evaluation
* **Date**: 2026-10-14
* **Status**: Proposed / Under Evaluation
* **Owner**: Scorpii Tech Architecture Agent

### Context
Developing a native desktop video editing application from scratch requires substantial effort (timeline management, media decoding, GPU compositing, audio mixing, export codecs). We require a modern, robust, and extensible baseline.

### Decision
Evaluate [Concat](https://github.com/jub0t/Concat) (Rust, native GPU compositor, Slint UI, automation APIs) as the initial technical candidate at a pinned commit.

### Consequences & Trade-offs
- **Pros**: Native Rust performance, cross-platform architecture, existing GPU compositor, local AI interfaces, automation hooks (MCP/CLI).
- **Cons**: AGPL-3.0-or-later license with GPL components; requires disciplined fork management and strict separation of product features via adapters/plugins to manage upstream rebase complexity.

---

## ADR-0002: Four-Tier Integration Hierarchy
* **Date**: 2026-10-14
* **Status**: Approved
* **Owner**: Scorpii Tech Engineering

### Context
Upstream changes in Concat could introduce merge conflicts if Scorpii Studio makes scattered edits throughout the core engine codebase.

### Decision
Enforce a four-tier integration hierarchy:
1. **Tier 1 (Preferred)**: Public supported interfaces (CLI, MCP, JSON-RPC, gRPC, Plugins).
2. **Tier 2**: Standalone product crates / modules interacting through documented adapters.
3. **Tier 3**: Minimal, well-isolated integration hooks in the host/UI layer.
4. **Tier 4 (Last Resort)**: Core engine modifications (timeline, render, export). Requires human review and an entry in `UPSTREAM_PATCHES.md`.

### Consequences
Significantly reduces rebase friction and ensures upstream improvements can be merged with minimal conflict.
