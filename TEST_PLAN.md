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

## 3. Test Fixtures (To be established in `tests/fixtures/`)
- `sample_1080p60.mp4` (standard high-framerate video)
- `sample_4k30.mp4` (high resolution test)
- `sample_audio_48khz.wav` (clean uncompressed audio)
- `sample_variable_framerate.mp4` (edge case: phone recordings)
