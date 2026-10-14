@echo off
title Installing Visual Studio 2022 C++ Tools
echo ======================================================================
echo Installing Visual Studio 2022 C++ Build Tools for Scorpii Studio...
echo Please click 'Yes' if Windows displays a User Account Control prompt.
echo ======================================================================
"C:\Users\ScoRpii\AppData\Local\Temp\WinGet\Microsoft.VisualStudio.2022.BuildTools.17.14.41\vs_BuildTools.exe" --passive --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended
echo ======================================================================
echo Installation complete! You can now close this window.
echo ======================================================================
pause
