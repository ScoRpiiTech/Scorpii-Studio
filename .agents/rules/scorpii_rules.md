# Scorpii Studio Engineering Directives & Rules
> Applies to all agent tasks across `Scorpii Studio`.

## Core Guardrails
- **Product**: SCORPII STUDIO (Tagline: CREATE WITHOUT LIMITS. | Publisher: Scorpii Tech).
- **Foundation**: Upstream open-source Concat (Rust, AGPL-3.0-or-later + GPL components).
- **Closed-source distribution**: NOT approved. All licensing obligations must be respected.
- **Privacy & Local Processing**: Local-first processing only. No cloud uploads, external AI calls, telemetry, or user accounts without explicit owner sign-off and user consent.
- **Integration Order**:
  1. Supported Interfaces (CLI, MCP, plugins, RPC)
  2. Isolated modules/crates + documented adapters
  3. Small host/UI hooks
  4. Core engine edits (strictly gated, requires entry in UPSTREAM_PATCHES.md and human approval).
- **Git Discipline**: Never force push. Clean upstream baseline tracked on `upstream/main`. Default GitHub branch `main` showcases Scorpii Studio. Development on `product/integration` and `feat/*`.
- **Quality**: Compiling != done. Verify media import, seek, playback, export, and project save/load.
- **Reporting**: Always conclude tasks with the structured Task Report format defined in `AGENTS.md`.
