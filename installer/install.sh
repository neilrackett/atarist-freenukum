#!/bin/sh
#
# Copyright (C) 2026 Neil Rackett
# SPDX-License-Identifier: GPL-3.0-or-later
#
# Download and install the Duke Nukem 1 shareware data (macOS/Linux).
# Run from the folder containing NUKUM.TOS.

DIR=$(dirname "$0")

if command -v python3 >/dev/null 2>&1; then
    exec python3 "$DIR/install.py" "$@"
elif command -v python >/dev/null 2>&1; then
    exec python "$DIR/install.py" "$@"
else
    echo "Python 3 is required but was not found."
    echo "Please install it (e.g. 'brew install python3' on macOS or"
    echo "'sudo apt install python3' on Debian/Ubuntu) and try again."
    exit 1
fi
