SCORPII STUDIO

CREATE WITHOUT LIMITS.

Agent Build Guide & Engineering Operating Rules

Version 1.0  |  Prepared 14 October 2026  |  Owner: Scorpii Tech

PurposeA shared operating document for coding agents and human reviewers. It defines the product direction, repository rules, upstream strategy, quality gates, and the decisions that must be resolved before a commercial release.

1. Product Identity and Mission

Product name: SCORPII STUDIO

Tagline: CREATE WITHOUT LIMITS.

Publisher / product family: Scorpii Tech. Related internal product: Scorpii Flow.

Mission: build a reliable, polished video-creation product by reusing a proven editing foundation where appropriate, then differentiating it through carefully selected creator workflows and automation—not by rewriting an entire editing engine unnecessarily.

Product principles

Reliability before feature count. A saved project must reopen, preview, and export predictably.

Reuse before rewrite. Inspect the existing engine and supported interfaces before changing core internals.

Modular by default. Keep product-specific logic in isolated modules, adapters, plugins, or services where technically appropriate.

No false promises. Do not claim conflict-free upstream updates, full platform support, or production readiness without evidence.

Privacy by design. Prefer local processing for media unless a clearly disclosed, user-authorized cloud feature requires upload.

Commercial and licensing compliance is a release gate, not a cleanup task.

Human approval for consequential changes. Agents must report findings and request approval for irreversible, licensing-sensitive, or architecture-changing actions.

2. Current Working Hypothesis

The initial technical candidate is the open-source Concat project: https://github.com/jub0t/Concat. Its current public documentation describes a cross-platform Rust video editor with a native editing engine, GPU compositor, local AI features, and automation interfaces. Treat all repository facts as provisional until agents inspect the exact commit checked out locally.

Critical gate — licensingConcat is published under AGPL-3.0-or-later with a plugin exception. Its published third-party notices also describe GPL-licensed components, including the Slint GPL option and certain FFmpeg/espeak-ng builds. A fork or combined application may carry source-disclosure and network-use obligations. Do not assume the product can be distributed as closed-source, or packaged for a marketplace under proprietary terms, without a documented legal/licensing review. The project’s trademark/name is separate from its code license.

References to inspect directly:

Repository: https://github.com/jub0t/Concat

Architecture: https://github.com/jub0t/Concat/blob/main/ARCHITECTURE.md

License: https://github.com/jub0t/Concat/blob/main/LICENSE

License exceptions: https://github.com/jub0t/Concat/blob/main/LICENSE-EXCEPTIONS.md

Third-party notices: https://github.com/jub0t/Concat/blob/main/THIRD_PARTY_NOTICES.md

Trademark policy: https://github.com/jub0t/Concat/blob/main/TRADEMARK.md

Contribution guide: https://github.com/jub0t/Concat/blob/main/CONTRIBUTING.md

3. Decisions: Confirmed vs. Not Yet Approved

Item

Status

Rule for agents

Brand and tagline

Confirmed

Use SCORPII STUDIO and CREATE WITHOUT LIMITS. in product planning and internal documentation.

Technical foundation

Candidate only

Evaluate Concat at a pinned commit; do not assume a fork is approved until feasibility and license gates pass.

Primary platform

Unconfirmed

First inspect upstream-supported targets and build status. Do not promise Windows/macOS/Linux/mobile parity without test evidence.

MVP feature set

Unconfirmed

Propose a small evidence-based MVP after a capability audit; do not implement every idea at once.

Commercial model

Unconfirmed

Could be marketplace distribution, paid desktop software, SaaS, or a compliant open-source model. Obtain owner decision before release architecture is locked.

Closed-source distribution

Not approved

Never represent the Concat-based product as proprietary-ready until the license review is complete.

Product domain, logo, trademark

Unverified

Check domain, package identifiers, repository names, and trademarks before public launch.

4. Agent Roles and Collaboration Rules

Agents may be assigned multiple roles, but every task must have one accountable owner and a review step.

Product / Research Agent: audits the repository, maps existing features, identifies target users and competitor differentiation, and separates verified facts from assumptions.

License & Supply-Chain Agent: inventories licenses, models, fonts, media assets, FFmpeg builds, plugins, and redistribution obligations. Flags blockers; does not give legal conclusions beyond evidence.

Architecture Agent: maps crates/modules, APIs, extension points, build targets, and required core patches. Proposes the smallest maintainable integration.

Implementation Agent: changes only the scoped files, follows existing patterns, adds tests, and avoids unrelated refactors.

QA Agent: independently builds/tests changed paths, checks regressions and project/export integrity, and reports reproducible evidence.

Release Agent: maintains versioning, changelog, build artifacts, checksums, signing, third-party notices, and release checklist.

Agent handoff formatEvery agent handoff must include: objective; repository and commit; files inspected/changed; commands run; tests and exact results; risks/blockers; assumptions; next action; and whether human approval is required.

5. Mandatory Repository Discovery Before Coding

Record repository URL, current branch, upstream commit SHA, working-tree state, Rust/toolchain versions, OS, and build instructions.

Read README, ARCHITECTURE, CONTRIBUTING, ROADMAP/TODO, LICENSE, LICENSE-EXCEPTIONS, THIRD_PARTY_NOTICES, TRADEMARK, security policy, and relevant crate documentation.

Build and run the unmodified upstream baseline before introducing product changes. Capture build logs and known failures.

Create a feature inventory: already available, partially available, missing, unstable, and dependent on external models/services.

Map public API/CLI/MCP/gRPC/JSON-RPC capabilities and test whether each needed operation is supported by the checked-out commit.

Inspect all target platforms independently. Do not infer iOS/Android packaging quality from desktop success.

Produce an architecture map and a patch map identifying every proposed change to upstream-owned files.

Produce a license/asset/model inventory and a written go/no-go recommendation before product implementation.

6. Architecture Strategy

Preferred order of integration, from least to most coupled:

Use an existing supported API, CLI, MCP, or plugin interface if it can satisfy the requirement.

Create a separate product module/crate or companion service and connect through a documented adapter.

Add a small, clearly isolated integration point in the host/UI only when no supported interface exists.

Modify core timeline, media, render, or export internals only when a documented requirement cannot be met otherwise and the owner approves the maintenance cost.

Rules

Do not assume a new folder alone prevents conflicts. Shared manifest files, registration points, types, UI wiring, and engine interfaces can still conflict.

Do not duplicate core editing functionality that already exists and is stable.

Do not create a product module that depends on private internals when a stable interface can be used.

Keep product branding, onboarding, settings, templates, and workflow logic separate from low-level rendering where practical.

Document each unavoidable upstream patch with rationale, exact files, tests, and a rebase note.

Do not introduce a backend, user account, analytics, telemetry, cloud upload, or subscription system without a defined product requirement and owner approval.

7. Git and Upstream Maintenance Workflow

Use a clean upstream-tracking branch and separate product branches. The workflow reduces integration risk; it does not guarantee zero conflicts.

Suggested branch model

upstream/main: remote tracking reference for the original repository.

origin/main: clean mirror of the approved upstream baseline; no product-specific commits.

product/integration: branch that combines the pinned upstream baseline with reviewed product patches.

feat/<short-name>: small, independently reviewable feature branches.

release/<version>: tested release candidate branch; only release-approved changes.

Initial setup

git clone https://github.com/<your-org>/scorpii-studio.gitcd scorpii-studiogit remote add upstream https://github.com/jub0t/Concat.gitgit remote -vgit fetch upstream

If the fork has not been created yet, the repository owner must create it first. Verify the default branch name and use the actual upstream branch/commit rather than assuming it is always main.

Sync procedure

Fetch upstream and inspect the incoming commit range and release notes.

Update the clean tracking branch only with a fast-forward when possible; push the synchronized mirror.

Create a dedicated integration branch for the update. Rebase product feature commits or reapply the documented patch series; do not rewrite shared branches that other agents use.

Resolve conflicts deliberately. Never accept an automated conflict resolution without reviewing the resulting semantics.

Run formatting, linting, unit tests, baseline build, integration tests, and export regression checks.

Merge into product/integration only after QA approval; tag the exact upstream SHA and product commit used by the build.

Git safetyNever force-push shared branches, discard unknown local changes, reset a teammate’s work, or run destructive cleanup commands without explicit approval. A successful merge/rebase does not prove the product still works.

8. Product Discovery and MVP Guardrails

The product differentiator should be selected after the upstream capability audit. Candidate directions—not commitments—include:

AI-assisted editing: transcript-aware edits, silence detection/removal, captions, and voice-over.

Batch video creation: reusable templates and automated production of multiple variants.

Creator workflow: streamlined editing/export presets for Shorts, Reels, tutorials, and marketing videos.

Do not try to launch all directions in the first milestone. The Product Agent should rank features by customer value, implementation effort, upstream compatibility, offline/privacy impact, model cost, and licensing risk.

MVP acceptance baseline

The app launches on the selected target platform(s) and identifies itself as SCORPII STUDIO in the approved branding scope.

A user can create/open/save a project without losing media references or edit decisions.

Core edit, preview, and export workflows that are claimed as supported are tested with reproducible sample media.

Any AI feature has a clear model/download/runtime requirement, error state, privacy explanation, and fallback behavior where feasible.

No feature is marked complete merely because it compiles; it must be exercised in the running app.

Release notes clearly distinguish implemented, experimental, unsupported, and planned capabilities.

9. Engineering Quality Gates

Gate

Required evidence

Fail condition

Baseline

Clean build and launch on pinned upstream commit

Cannot reproduce or explain baseline failures

Code

Formatter, lint/static checks, no unexplained warnings

Unreviewed warnings or unrelated refactors

Tests

Unit tests plus tests for changed behavior

Tests skipped, failing, or not reported

Media integrity

Open/save/reopen; seek, audio, timeline, export sample checks

Corrupt project, broken media, sync drift, or failed export

Performance

Measured startup, preview, memory and export on documented hardware

Claims made without measurements

Security/privacy

Dependency review, safe file handling, no secret leakage, explicit upload consent

Unapproved telemetry/upload or embedded secrets

Licensing

Versioned dependency/model/asset inventory and approved compliance plan

Unknown license or unreviewed redistribution

Release

Version, changelog, checksums, signing plan, notices and smoke-test report

Missing release evidence

10. Security, Privacy, and Data Handling

Treat all imported media, project files, metadata, subtitles, model outputs, and external URLs as untrusted input.

Do not upload footage, audio, transcripts, project files, or user data to a cloud service unless the feature explicitly requires it and the user is informed and authorizes it.

Never place API keys, tokens, signing credentials, or private endpoints in source code, logs, screenshots, test fixtures, or release bundles.

If cloud AI is added, document data flow, retention, provider, cost controls, failure behavior, and deletion behavior before implementation.

Use least privilege for filesystem access and avoid executing shell commands built from untrusted filenames or user-provided text.

Voice cloning or voice imitation features must require appropriate consent and must not be marketed as permission to impersonate others.

11. Release and Distribution Rules

No public release before the licensing and commercial-model gate is signed off.

Use a distinct product name and visual identity; do not imply that SCORPII STUDIO is the official Concat project or that upstream maintainers endorse it.

Maintain required source offers, notices, corresponding-source materials, and license files if required by the final compliance analysis.

Inventory the exact FFmpeg build/configuration, codec licenses, model weights, fonts, images, icons, and other redistributed assets.

Test installers and upgrades on clean machines; record supported OS versions and known limitations.

Generate checksums and sign artifacts where supported. Keep release artifacts traceable to source commit and build environment.

If submitting to a marketplace, review that marketplace's source-code, license, dependency, and resale rules before preparing the package.

12. Required Documentation in the Repository

PRODUCT_BRIEF.md — positioning, target user, product promise, MVP and non-goals.

ARCHITECTURE_DECISIONS.md — dated decisions, alternatives, trade-offs, and approval owner.

UPSTREAM_PATCHES.md — every local patch to upstream-owned code and how to revalidate it.

LICENSE_INVENTORY.md — source licenses, transitive dependencies, model licenses, assets, and obligations.

TEST_PLAN.md — supported platform matrix, sample media, test procedures, expected results.

RELEASE_CHECKLIST.md — reproducible release process, artifacts, checksums, notices, signing and smoke tests.

CHANGELOG.md — user-visible changes, fixes, limitations, and migration notes.

13. Definition of Done for Every Agent Task

The requested outcome is implemented and scoped; no unrelated cleanup is mixed in.

The agent names the exact files changed and explains the design decision.

Formatting, tests, and builds appropriate to the change were run; results are reported honestly.

A human reviewer can reproduce the behavior from the instructions supplied.

Risks, untested paths, assumptions, and remaining work are listed explicitly.

Any license, security, data migration, architecture, or release-impacting issue is escalated before proceeding.

Documentation and tests are updated when behavior, configuration, or operating procedure changes.

14. First Execution Plan

Phase 0 — Feasibility and compliance (must pass)

Pin and build an unmodified Concat commit.

Audit architecture, supported platforms, APIs, feature inventory, build dependencies, and current known issues.

Complete a license/third-party inventory and determine whether the desired commercial model is viable.

Deliver a written go/no-go recommendation. Do not start a large product fork before approval.

Phase 1 — Product definition

Choose initial platform(s), audience, distribution model, and one primary differentiator.

Create product requirements, user flows, acceptance criteria, and a feature priority list.

Approve product architecture and the allowed upstream patch budget.

Phase 2 — Thin vertical slice

Implement one end-to-end workflow using existing capabilities and the least-coupled integration point.

Demonstrate launch → import media → edit/automate → preview → export → reopen project.

Measure build, reliability, and performance; obtain QA approval.

Phase 3 — Harden and differentiate

Add only the next highest-value feature after the vertical slice is stable.

Add regression fixtures, platform-specific tests, crash/error reporting policy, and accessibility/localization as needed.

Validate upstream update procedure on a test branch.

Phase 4 — Release readiness

Finalize licensing, branding, packaging, signing, documentation, and support policy.

Run clean-machine smoke tests and publish only with owner approval.

15. Copy/Paste Master Instruction for Coding Agents

You are an engineering agent working on SCORPII STUDIO — CREATE WITHOUT LIMITS., a Scorpii Tech video-creation product. Follow SCORPII_STUDIO_Agent_Build_Guide.docx and the repository's local equivalent as governing project instructions. Start by inspecting the actual repository state and pinned commit; do not assume prior summaries are correct. Do not implement features until the task scope and required acceptance criteria are clear. Prefer existing supported APIs, plugins, CLI, MCP, JSON-RPC, or gRPC interfaces over core modifications. Keep changes small, modular, tested, and documented. Upstream sync can still cause conflicts and semantic regressions; never promise zero conflicts. Treat licensing as a blocking gate: Concat is AGPL-3.0-or-later with a plugin exception and has GPL-licensed components. Do not assume proprietary distribution or marketplace suitability. Do not upload user media or add telemetry/cloud services without explicit product approval and clear disclosure. Never force-push, discard unknown work, expose secrets, or make a release without approval. For every task, report: goal, repository/commit, files inspected and changed, implementation summary, commands run, exact test/build results, risks, assumptions, and next steps. If the requirement conflicts with upstream architecture, license obligations, security, or product scope, stop and present options instead of silently taking a risky path.

16. Reference Links

Concat repository: https://github.com/jub0t/Concat

Architecture: https://github.com/jub0t/Concat/blob/main/ARCHITECTURE.md

README: https://github.com/jub0t/Concat/blob/main/README.md

License: https://github.com/jub0t/Concat/blob/main/LICENSE

License exceptions: https://github.com/jub0t/Concat/blob/main/LICENSE-EXCEPTIONS.md

Third-party notices: https://github.com/jub0t/Concat/blob/main/THIRD_PARTY_NOTICES.md

Trademark policy: https://github.com/jub0t/Concat/blob/main/TRADEMARK.md

Contributing guide: https://github.com/jub0t/Concat/blob/main/CONTRIBUTING.md

Releases: https://github.com/jub0t/Concat/releases