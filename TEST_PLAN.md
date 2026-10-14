# SCORPII STUDIO — TEST & QUALITY VERIFICATION PLAN

A feature is not considered complete simply because it compiles without errors. This plan defines the media fixtures, test procedures, and quality gates required for validation.

---

## 1. Supported Platform Matrix
- [ ] **Windows 11 / 10 (x86_64)**: Primary target for initial development & validation.
- [ ] **macOS (Apple Silicon / Intel)**: Secondary target.
- [ ] **Linux (Ubuntu / Fedora / Arch)**: Secondary target.

---

## 2. Core Media Integrity Gates (Must Pass for Every Release/PR)
1. **Project File Lifecycle**:
   - Create project $\rightarrow$ Add video/audio/image tracks $\rightarrow$ Save project $\rightarrow$ Close app $\rightarrow$ Reopen project $\rightarrow$ Verify all asset links and timeline offsets remain intact.
2. **Timeline Playback & Seeking**:
   - Scrub playhead across multiple video clips, cuts, and transitions without crash or memory leak.
   - Verify audio-video sync does not drift during continuous 60-second preview.
3. **Export Integrity**:
   - Render project to MP4 (H.264/AAC).
   - Verify exported file has correct resolution, frame rate, duration, and synced audio.
4. **Performance Baselines**:
   - Record cold startup time, peak memory usage during timeline scrub, and export render multiplier (e.g. 1.2x real-time).

---

## 3. Phase 2 Thin Vertical Slice Checklist
- [ ] **Branding Display**: Window title and header identify as **SCORPII STUDIO** (*CREATE WITHOUT LIMITS.*).
- [ ] **Media Ingestion**: Import standard 1080p / 4K video clip into media bin and place on timeline.
- [ ] **1-Click 9:16 Reframe**: Activate Creator Fast-Cut $\rightarrow$ timeline dimensions reconfigure cleanly to $1080 \times 1920$ with centered composition.
- [ ] **Live Monitor Preview**: Play and scrub at 60fps in the GPU monitor; verify frame bounds match 9:16 portrait.
- [ ] **Undo / Redo Safety**: Press Ctrl+Z $\rightarrow$ reverts to 16:9; Ctrl+Y $\rightarrow$ returns to 9:16.
- [ ] **Export Verification**: Render to `output_short.mp4`; verify video streams at $1080 \times 1920$ with AAC audio.
- [ ] **Persistence**: Save project, exit app, reopen project; verify 9:16 timeline is retained.


---

## 3. Test Fixtures (To be established in `tests/fixtures/`)
- `sample_1080p60.mp4` (standard high-framerate video)
- `sample_4k30.mp4` (high resolution test)
- `sample_audio_48khz.wav` (clean uncompressed audio)
- `sample_variable_framerate.mp4` (edge case: phone recordings)
