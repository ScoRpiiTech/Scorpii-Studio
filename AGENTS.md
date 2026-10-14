# SCORPII STUDIO — AGENT OPERATING RULES & CORE DIRECTIVES
> **Product**: SCORPII STUDIO  
> **Tagline**: CREATE WITHOUT LIMITS.  
> **Owner**: Scorpii Tech  
> **Governing Document**: `SCORPII_STUDIO_Agent_Build_Guide.docx`  

---

## 1. Core Operating Directives (Non-Negotiable)

1. **Reliability Before Feature Count**:
   - A video project must create, open, save, seek, preview, and export without corruption, audio desync, or data loss.
   - Compiling does **not** mean done. Functionality must be validated with reproducible media tests.

2. **Reuse Before Rewrite**:
   - Respect the upstream engine foundation ([Concat](https://github.com/jub0t/Concat)).
   - Do **not** rewrite core timeline, GPU compositor, or export pipelines unless an approved requirement cannot be satisfied otherwise.

3. **Strict Integration Hierarchy (Order of Precedence)**:
   - **Level 1 (Highest Priority)**: Use existing supported public interfaces (CLI, MCP, JSON-RPC, gRPC, Plugin API).
   - **Level 2**: Create a separate product crate / companion service connected via a documented adapter.
   - **Level 3**: Add small, isolated integration hooks in the host/UI layer.
   - **Level 4 (Last Resort)**: Modify upstream core files (timeline, compositor, media decoder, export). **Requires explicit human sign-off** and a dedicated record in `UPSTREAM_PATCHES.md`.

4. **Licensing & Legal Compliance Gate**:
   - Concat is licensed under **AGPL-3.0-or-later** with a plugin exception and contains GPL components (Slint GPL option, FFmpeg, espeak-ng).
   - **Closed-source / proprietary distribution is strictly NOT APPROVED** until an official legal review is documented.
   - All source offers, notices, and copyleft obligations must be honored.
   - Distinct brand identity must be maintained; never claim endorsement or affiliation with upstream maintainers.

5. **Privacy by Design**:
   - All video, audio, image, and project processing must be **local-first**.
   - **Zero telemetry, analytics, user account backend, or cloud upload** unless explicitly requested, approved by human owner, and fully disclosed to the user with opt-in consent.
   - Never embed secrets, API keys, tokens, or private credentials in code, tests, or release assets.

---

## 2. Git & Upstream Synchronization Rules

1. **Branch Strategy**:
   - `upstream/main`: Official upstream Concat repository remote tracking branch (untouched baseline).
   - `origin/main` / `main`: Primary default branch for Scorpii Studio on GitHub containing hardened releases and brand presentation.
   - `product/integration`: Combined branch with pinned upstream baseline + approved Scorpii Studio patches.
   - `feat/<name>`: Focused, isolated feature branches branched from `product/integration`.
   - `release/<version>`: Hardened release candidates.

2. **Safety Invariants**:
   - **NEVER** force-push (`git push -f`) to shared or integration branches.
   - **NEVER** run destructive reset/clean commands (`git reset --hard`, `git clean -fd`) without explicit user instruction.
   - **NEVER** assume upstream synchronization is conflict-free or regression-free; every merge/rebase requires full build, unit test, and media export verification.

---

## 3. Mandatory Task Handoff & Reporting Contract

Every agent task completion must report using this structured format:

```markdown
### Task Report
- **Goal / Scope**: [Clear statement of the objective]
- **Repository State**: [Pinned Commit SHA, Active Branch, Working Tree status]
- **Files Inspected / Modified**: [Clickable links to files]
- **Design Rationale**: [Why this approach was taken; adherence to integration hierarchy]
- **Commands Executed**: [Exact terminal commands executed]
- **Verification & Test Results**: [Build logs, unit tests, media playback/export verification]
- **Risks, Blockers & Assumptions**: [Potential edge cases, untested paths]
- **Human Approval Required?**: [YES / NO — specify if architecture, licensing, or core modification is involved]
- **Next Steps**: [Immediate next action item]
```

---

## 4. Required Repository Governance Files

The following files must be kept accurate and up to date at all times:
- `PRODUCT_BRIEF.md`: Product positioning, target user, and MVP scope.
- `ARCHITECTURE_DECISIONS.md`: Log of all architectural choices, trade-offs, and owners.
- `UPSTREAM_PATCHES.md`: Detailed ledger of any modifications made to upstream-owned code.
- `LICENSE_INVENTORY.md`: Comprehensive audit of crates, models, assets, and licenses.
- `TEST_PLAN.md`: Media test fixtures, platform matrix, and validation steps.
- `RELEASE_CHECKLIST.md`: Pre-release verification, signing, checksums, and notices.
- `CHANGELOG.md`: User-facing release notes.
