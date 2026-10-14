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

---

## ADR-0003: Local Build Environment & Prebuilt Dependency Strategy
* **Date**: 2026-10-14
* **Status**: Approved / Verified
* **Owner**: Scorpii Tech Architecture Agent

### Context
Building Concat on Windows requires linking native C++ libraries (FFmpeg 7+, ONNX Runtime, whisper.cpp, sherpa-onnx) and generating Rust bindings via bindgen.

### Decision
Standardize the local Windows developer and CI build environment:
1. Rust toolchain pinned at `1.93.1-x86_64-pc-windows-msvc`.
2. CMake and LLVM (`libclang.dll`) installed to system paths.
3. Prebuilt native shared libraries placed under `.gitignore`d `vendor/`:
   - `vendor/ffmpeg-win64`: BtbN FFmpeg 8.1 shared build (configured via `FFMPEG_DIR`).
   - `vendor/onnxruntime-win-x64`: Microsoft ONNX Runtime 1.28.0 shared build (configured via `ORT_LIB_LOCATION`).
4. Avoid static linking of ONNX Runtime to maintain binary compatibility across MSVC CRT versions.

### Consequences
- Baseline unmodified Concat builds cleanly on Windows (335+ unit tests passing).
- Zero pollution of git history (vendor directory is ignored).

---

## ADR-0004: Phase 2 Thin Vertical Slice (Creator Fast-Cut Architecture)
* **Date**: 2026-10-14
* **Status**: Approved
* **Owner**: Scorpii Tech Architecture Agent

### Context
Phase 2 requires a thin vertical slice that proves the end-to-end user workflow: Launch $\rightarrow$ Import Media $\rightarrow$ Automate/Reframe $\rightarrow$ Preview $\rightarrow$ Export $\rightarrow$ Reopen Project, while strictly adhering to the Tier 1 / Tier 2 integration hierarchy.

### Decision
1. **Differentiator Scope**: Implement the **Creator Fast-Cut & Social Suite**, enabling 1-click conversion between landscape (16:9) and vertical (9:16 Shorts/Reels/TikTok) formats.
2. **Integration Tier**:
   - Use **Tier 1 (Public Command Interface)** via `Command::SetTimelineVideo`. This ensures all changes cleanly integrate with undo/redo, auto-save (`concat.json`), live GPU monitor preview, and export rendering without patching core engine crates.
   - Use **Tier 3 (Isolated UI Hook)** in Slint UI (`title-bar.slint` / `app.slint`) to present the Scorpii Studio brand identity (*CREATE WITHOUT LIMITS.*) and the 1-click Creator Fast-Cut quick-actions.
3. **Upstream Patch Budget**: 0 modifications to `concat-core`, `concat-media`, `concat-render`, or `concat-export`.

### Consequences
- Clean, conflict-free rebasing against future upstream Concat releases.
- Immediate end-to-end validation of the creator workflow.


