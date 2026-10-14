# SCORPII STUDIO — UPSTREAM PATCHES LEDGER

Every local patch or modification applied to upstream-owned code must be cataloged here.  
This ledger ensures smooth rebases, clear ownership, and transparent maintenance.

---

## Patch Ledger Format

| Patch ID | Files Affected | Purpose & Rationale | Integration Tier | Added Date | Revalidation Notes |
| :--- | :--- | :--- | :--- | :--- | :--- |
| *(None yet)* | — | Baseline is unmodified | — | — | Awaiting pinned commit audit |

---

## Patch Entry Details Template

```markdown
### [PATCH-XXXX] <Short Title>
- **Date**: YYYY-MM-DD
- **Target Files**:
  - `path/to/upstream/file.rs`
- **Rationale**: [Why this could not be accomplished via Tier 1-3]
- **Approval Owner**: [Name/Agent]
- **Upstream SHA Introduced**: [Commit SHA]
- **Test Instructions**: [How to verify this patch still works after rebase]
```
