@echo off
rem Copyright (C) 2026 Neil Rackett
rem SPDX-License-Identifier: GPL-3.0-or-later
rem
rem Download and install the Duke Nukem 1 shareware data (Windows).
rem Run from the folder containing NUKUM.TOS.

where py >nul 2>nul
if %errorlevel%==0 (
    py -3 "%~dp0install.py" %*
    goto :eof
)

where python >nul 2>nul
if %errorlevel%==0 (
    python "%~dp0install.py" %*
    goto :eof
)

echo Python 3 is required but was not found.
echo Please install it from https://www.python.org/ and try again.
exit /b 1
