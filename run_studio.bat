@echo off
title Scorpii Studio
set ORT_LIB_LOCATION=%~dp0vendor\onnxruntime-win-x64\lib
set FFMPEG_DIR=%~dp0vendor\ffmpeg-win64
set PATH=%~dp0src\target\debug;%~dp0vendor\onnxruntime-win-x64\lib;%~dp0vendor\ffmpeg-win64\bin;C:\Program Files\LLVM\bin;C:\Program Files\CMake\bin;%USERPROFILE%\.cargo\bin;%PATH%

cd /d "%~dp0src\target\debug"
start "" "%~dp0src\target\debug\concat.exe"
