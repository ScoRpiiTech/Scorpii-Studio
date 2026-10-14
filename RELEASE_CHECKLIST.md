# SCORPII STUDIO — RELEASE CHECKLIST

Every release candidate must pass this checklist before publication.

---

## 1. Governance & Compliance
- [ ] Legal & licensing sign-off completed for commercial/distribution model.
- [ ] Third-party notices and license texts included in build bundle (`THIRD_PARTY_NOTICES.md`).
- [ ] Independent Scorpii Studio branding verified (no leftover upstream logos/names where prohibited).
- [ ] Privacy policy verified (local processing; no unauthorized telemetry/uploads).

---

## 2. Code & Build Verification
- [ ] Built on clean machine / clean working tree from tagged release commit.
- [ ] `cargo clippy --all-targets` and `cargo fmt --check` pass with zero warnings.
- [ ] Full automated test suite passes (`cargo test --all`).
- [ ] Media integrity tests (save, reopen, seek, export) pass with zero errors.

---

## 3. Packaging & Distribution
- [ ] Installer generated and tested on clean target OS.
- [ ] Binary code signing applied (where supported).
- [ ] SHA-256 checksums generated and recorded.
- [ ] Release notes and user-facing changelog updated in `CHANGELOG.md`.
