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

## 3. Dependency & Asset Ledger (Audited in Phase 0)

| Crate / Asset | Version | Stated License | Copyleft Obligations | Status / Notes |
| :--- | :--- | :--- | :--- | :--- |
| `concat-core` | `0.2.7` | AGPL-3.0-or-later | Strong copyleft | std-only, verified passing |
| `concat-project` | `0.2.7` | AGPL-3.0-or-later | Strong copyleft | Document model & undo |
| `concat-media` | `0.2.7` | AGPL-3.0-or-later | Strong copyleft | Links FFmpeg |
| `concat-render` | `0.2.7` | AGPL-3.0-or-later | Strong copyleft | GPU compositor, wgpu 30 |
| `concat-effects` | `0.2.7` | AGPL-3.0-or-later | Strong copyleft | WGSL shaders & look tables |
| `concat-text` | `0.2.7` | AGPL-3.0-or-later | Strong copyleft | Text rasterizer |
| `concat-export` | `0.2.7` | AGPL-3.0-or-later | Strong copyleft | Media encoding & export |
| `concat-speech` | `0.2.7` | AGPL-3.0-or-later | Strong copyleft | Whisper & Sherpa-onnx |
| `concat-api` | `0.2.7` | AGPL-3.0-or-later | Strong copyleft | JSON-RPC API dispatcher |
| `concat-server` | `0.2.7` | AGPL-3.0-or-later | Strong copyleft | Socket / TCP / gRPC server |
| `concat-cli` | `0.2.7` | AGPL-3.0-or-later | Strong copyleft | CLI driver (verified working) |
| `concat` (UI) | `0.2.7` | AGPL-3.0-or-later | Strong copyleft | Slint 1.18.1 with Skia |
| **FFmpeg** | `8.1` (BtbN) | GPLv3 (shared) | Copyleft on redistribution | Prebuilt DLLs under vendor |
| **ONNX Runtime** | `1.28.0` | MIT | Permissive | Microsoft prebuilt shared DLLs |
| **whisper.cpp** | Embedded | MIT | Permissive | Built via CMake in whisper-rs-sys |
| **sherpa-onnx** | `1.13.7` | Apache 2.0 | Permissive | Kokoro TTS backend |
| **Slint** | `1.18.1` | GPLv3 / Royalty-free | Strong copyleft (GPL) | Skia backend |

