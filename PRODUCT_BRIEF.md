# SCORPII STUDIO — PRODUCT BRIEF

**Product Name**: SCORPII STUDIO  
**Tagline**: CREATE WITHOUT LIMITS.  
**Publisher**: Scorpii Tech  
**Related Products**: Scorpii Flow  
**Status**: Phase 1 Approved — Active Phase 2 (Thin Vertical Slice)  

---

## 1. Product Mission & Value Proposition
Build a dependable, high-efficiency, visually polished desktop video-creation application by leveraging the proven foundation of [Concat](https://github.com/jub0t/Concat) (a cross-platform Rust video editor) and differentiating it through creator-first automation, workflow acceleration, and intuitive tooling—without reinventing the fundamental timeline and GPU rendering engines.

---

## 2. Target Audience & Primary Use Case
- **Primary Audience**: Solo creators, influencers, and marketing teams producing short-form video content (YouTube Shorts, Instagram Reels, TikTok).
- **Core Use Case**: Taking raw horizontal or long-form clips, converting them instantly to engaging 9:16 vertical videos with auto-framing, adding stylized dynamic captions, and exporting in 1-click without subscription walls or cloud privacy risks.

---

## 3. Product Principles
1. **Reliability Before Feature Count**: A saved project must reopen, seek, preview, and export reliably every single time.
2. **Reuse Before Rewrite**: Leverage upstream engine stability and official interfaces rather than replacing core subsystems.
3. **Local-First & Private**: Media stays on the user's machine. Zero background tracking or unprompted cloud processing.
4. **License Compliance**: Commercial feasibility and open-source license adherence are release gates, not afterthoughts.

---

## 4. Confirmed MVP Differentiator: Creator Fast-Cut & Social Suite
- **Smart 9:16 Format Conversion**: One-click aspect ratio conversion (1080x1920) with automatic intelligent cropping and framing.
- **Dynamic Stylized Captions**: Pre-styled, high-retention creator caption presets (karaoke highlight, bold sans, vibrant outlines) integrated with local Whisper transcription.
- **1-Click Social Export Presets**: Pre-configured render settings tailored for Shorts, Reels, and TikTok (1080x1920 60fps / 30fps H.264/AAC with optimal bitrates).
- **Native UI Integration**: Seamlessly integrated into the desktop Slint UI via dedicated Scorpii Studio toolbar and pane hooks.

---

## 5. Phase 2 Thin Vertical Slice Acceptance Criteria
The minimal vertical slice must demonstrate the complete end-to-end user loop:
1. **Launch**: Application opens identifying as **SCORPII STUDIO** (`CREATE WITHOUT LIMITS.`).
2. **Import Media**: User imports a standard 16:9 video into the project.
3. **Automate / Reframe**: User activates the Scorpii Creator Fast-Cut tool; the timeline adjusts to 9:16 vertical orientation with centered subject framing.
4. **Preview**: Smooth, artifact-free GPU preview in the monitor viewport with synchronized audio.
5. **Export**: 1-click export generates a valid `output_short.mp4` file.
6. **Persistence**: Save project, exit application, reopen project, and verify all edits, media references, and frame settings remain 100% intact.

---

## 6. Non-Goals (Initial Scope)
- Cloud-hosted video processing or mandatory online accounts.
- Proprietary closed-source release prior to thorough AGPL/GPL licensing sign-off.
- Multi-user real-time collaborative cloud editing.

