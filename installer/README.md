# Shareware data installer

These scripts download the freely distributable Duke Nukem 1 shareware
episode (`1duke.zip`) and extract its game data into `NUKUM\DATA`,
ready to play.

Run from the folder containing `NUKUM.TOS`:

* **macOS / Linux:** `sh installer/install.sh`
* **Windows:** `installer\install.bat`

Python 3 is required (preinstalled on macOS and most Linux systems).

If the download fails — the 3D Realms FTP server comes and goes —
download `1duke.zip` manually (e.g. search for `1duke` on
[archive.org](https://archive.org/)), put it next to `install.py`,
and run the script again.
