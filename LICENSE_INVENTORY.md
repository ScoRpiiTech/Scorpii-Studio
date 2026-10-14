# SCORPII STUDIO — LICENSE & SUPPLY CHAIN INVENTORY

This document provides a comprehensive accounting of licenses, dependencies, external models, assets, and redistribution obligations.

---

## 1. Upstream Core License Baseline
* **Candidate Repository**: [jub0t/Concat](https://github.com/jub0t/Concat)
* **Primary License**: **AGPL-3.0-or-later** (with plugin exception)
* **Status**: Under audit (Phase 0)

### Known Component Licenses:
* **Slint**: Slint GPLv3 option (or proprietary commercial license if acquired).
* **FFmpeg**: LGPLv2.1+ or GPLv2+ (depending on enabled codecs/libraries like libx264/libx265).
* **espeak-ng / TTS**: GPLv3.
* **Whisper / AI Models**: MIT / Apache 2.0 (weights and architectures subject to upstream inventory).

---

## 2. Redistribution Obligations & Commercial Model Gate
* **Source Code Disclosure**: AGPL-3.0 requires offering corresponding source code to network users and downstream recipients.
* **Proprietary Packaging**: **NOT APPROVED**. A formal compliance review is required before any closed-source packaging or commercial marketplace distribution can proceed.
* **Trademark & Branding**: Scorpii Studio must use independent branding, trademarks, and logos. Do not use upstream logos or suggest upstream endorsement.

---

## 3. Dependency & Asset Ledger (To be populated during Phase 0 audit)

| Crate / Asset | Version | Stated License | Copyleft Obligations | Status / Notes |
| :--- | :--- | :--- | :--- | :--- |
| *(Pending audit)* | — | — | — | Run `cargo tree` / `cargo deny` during Phase 0 |
