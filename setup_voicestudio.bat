@echo off
title Setting up Scorpii Voice Studio
echo ======================================================================
echo Setting up Scorpii Voice Studio dependencies (debpalash/VoiceStudio)
echo Pinned Python 3.11 + PyTorch + FastAPI backend via uv
echo ======================================================================

set "ROOT_DIR=%~dp0"
cd /d "%ROOT_DIR%vendor\voicestudio"

where uv >nul 2>nul
if %ERRORLEVEL% neq 0 (
    echo [1/3] Installing uv package manager...
    pip install uv
) else (
    echo [1/3] uv package manager detected.
)

echo [2/3] Provisioning virtual environment and dependencies...
uv sync

echo [3/3] Voice Studio setup complete!
echo ======================================================================
echo Voice Studio is ready to be launched as an embedded sidecar or standalone service.
echo ======================================================================
pause
