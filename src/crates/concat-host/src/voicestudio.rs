// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Scorpii Studio contributors

//! Bridge and lifecycle manager for debpalash/VoiceStudio.
//!
//! Provides a hybrid architecture:
//! - Connects directly to any external VoiceStudio instance on `127.0.0.1:3900`
//!   (e.g. running in Docker or standalone development).
//! - If no instance is active, automatically provisions and manages an embedded
//!   background sidecar process using `uv` from `vendor/voicestudio`.
//! - Automatically terminates the sidecar on shutdown to release all GPU VRAM.

use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Default TCP port VoiceStudio listens on.
pub const DEFAULT_VOICESTUDIO_PORT: u16 = 3900;
/// Default HTTP base URL for local VoiceStudio.
pub const DEFAULT_VOICESTUDIO_URL: &str = "http://127.0.0.1:3900";

/// Request parameters for VoiceStudio speech generation.
#[derive(Clone, Debug)]
pub struct VoiceStudioGenerateRequest {
    /// Text to synthesize.
    pub text: String,
    /// Language code (e.g. "en", "es", "zh").
    pub language: Option<String>,
    /// Speed multiplier (default 1.0).
    pub speed: f32,
    /// Optional voice profile ID to speak in.
    pub profile_id: Option<String>,
    /// Optional natural-language expression or emotion prompt.
    pub instruct: Option<String>,
}

impl Default for VoiceStudioGenerateRequest {
    fn default() -> Self {
        Self {
            text: String::new(),
            language: Some("en".to_string()),
            speed: 1.0,
            profile_id: None,
            instruct: None,
        }
    }
}

/// Manages communication and lifecycle of the VoiceStudio backend.
pub struct VoiceStudioManager {
    base_url: String,
    sidecar: Mutex<Option<Child>>,
}

impl Default for VoiceStudioManager {
    fn default() -> Self {
        Self::new(None)
    }
}

impl Drop for VoiceStudioManager {
    fn drop(&mut self) {
        self.stop();
    }
}

impl VoiceStudioManager {
    /// Creates a new manager targeting `url` or `DEFAULT_VOICESTUDIO_URL`.
    pub fn new(url: Option<String>) -> Self {
        Self {
            base_url: url.unwrap_or_else(|| DEFAULT_VOICESTUDIO_URL.to_string()),
            sidecar: Mutex::new(None),
        }
    }

    /// Returns the active backend base URL.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Tests if the VoiceStudio FastAPI backend is responding.
    pub fn is_healthy(&self) -> bool {
        let probe_url = format!("{}/docs", self.base_url.trim_end_matches('/'));
        ureq::get(&probe_url)
            .timeout(Duration::from_millis(1500))
            .call()
            .map(|resp| resp.status() == 200)
            .unwrap_or(false)
    }

    /// Ensures VoiceStudio is available: connects to an existing instance if running,
    /// or starts the local sidecar from vendor/voicestudio if present.
    pub fn ensure_running(&self, vendor_dir: &Path) -> Result<(), String> {
        if self.is_healthy() {
            log::info!("VoiceStudio backend is already active at {}", self.base_url);
            return Ok(());
        }

        if !vendor_dir.exists() {
            return Err(format!(
                "VoiceStudio is not running and vendor directory not found at: {}",
                vendor_dir.display()
            ));
        }

        log::info!("Spawning VoiceStudio embedded sidecar from {}", vendor_dir.display());

        let mut cmd = Command::new("uv");
        cmd.args([
            "run",
            "uvicorn",
            "main:app",
            "--app-dir",
            "backend",
            "--host",
            "127.0.0.1",
            "--port",
            &DEFAULT_VOICESTUDIO_PORT.to_string(),
        ]);
        cmd.current_dir(vendor_dir);

        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x0800_0000;
            cmd.creation_flags(CREATE_NO_WINDOW);
        }

        let child = cmd
            .spawn()
            .map_err(|e| format!("Failed to spawn VoiceStudio via uv: {e}"))?;

        if let Ok(mut lock) = self.sidecar.lock() {
            *lock = Some(child);
        }

        // Wait up to 10 seconds for the backend to become ready
        let start = Instant::now();
        while start.elapsed() < Duration::from_secs(10) {
            std::thread::sleep(Duration::from_millis(500));
            if self.is_healthy() {
                log::info!("VoiceStudio sidecar successfully initialized and healthy");
                return Ok(());
            }
        }

        Err("VoiceStudio sidecar spawned but did not respond to health check within 10s".to_string())
    }

    /// Stops any spawned sidecar process to release GPU/VRAM resources.
    pub fn stop(&self) {
        if let Ok(mut lock) = self.sidecar.lock() {
            if let Some(mut child) = lock.take() {
                log::info!("Stopping VoiceStudio background sidecar");
                let _ = child.kill();
                let _ = child.wait();
            }
        }
    }

    /// Synthesizes speech using VoiceStudio and saves the audio to destination_path.
    pub fn generate(
        &self,
        request: &VoiceStudioGenerateRequest,
        destination_path: &Path,
    ) -> Result<PathBuf, String> {
        let endpoint = format!("{}/generate", self.base_url.trim_end_matches('/'));

        let form = ureq::post(&endpoint)
            .timeout(Duration::from_secs(120));

        let speed_str = request.speed.to_string();
        let mut form_data = vec![
            ("text", request.text.as_str()),
            ("speed", speed_str.as_str()),
        ];

        let lang_str;
        if let Some(ref lang) = request.language {
            lang_str = lang.clone();
            form_data.push(("language", lang_str.as_str()));
        }

        let profile_str;
        if let Some(ref prof) = request.profile_id {
            profile_str = prof.clone();
            form_data.push(("profile_id", profile_str.as_str()));
        }

        let instruct_str;
        if let Some(ref inst) = request.instruct {
            instruct_str = inst.clone();
            form_data.push(("instruct", instruct_str.as_str()));
        }

        let resp = form
            .send_form(&form_data)
            .map_err(|e| format!("VoiceStudio generation request failed: {e}"))?;

        if resp.status() != 200 {
            return Err(format!("VoiceStudio returned HTTP status: {}", resp.status()));
        }

        let mut reader = resp.into_reader();
        let mut file = std::fs::File::create(destination_path)
            .map_err(|e| format!("Failed to create destination audio file: {e}"))?;
        std::io::copy(&mut reader, &mut file)
            .map_err(|e| format!("Failed to write audio stream to disk: {e}"))?;

        Ok(destination_path.to_path_buf())
    }
}
