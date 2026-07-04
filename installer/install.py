#!/usr/bin/env python3
#
# Copyright (C) 2026 Neil Rackett
# SPDX-License-Identifier: GPL-3.0-or-later
#
# Download the Duke Nukem 1 shareware episode (1duke.zip) and extract
# its game data into NUKUM/DATA, ready for FreeNukum for Atari ST.
#
# Run from the folder containing NUKUM.TOS:
#
#   python3 install.py [target-folder]
#
# If a 1duke.zip is already present next to this script or in the
# target folder, it is used instead of downloading.
#
# The .SHR archive inside 1duke.zip uses PKWARE DCL "implode"
# compression; the decoder below is a Python port of zlib's
# contrib/blast by Mark Adler.

import io
import os
import sys
import urllib.request
import zipfile

DOWNLOAD_URLS = [
    # canonical source, offline more often than not these days
    "ftp://ftp.3drealms.com/share/1duke.zip",
    # mirror of the identical original archive
    "http://www.classicdosgames.com/files/games/apogee/1duke.zip",
]
ZIP_NAME = "1duke.zip"
SHR_NAME = "DN1SW20.SHR"
MAXBITS = 13


class BlastError(Exception):
    pass


class _Bits:
    """LSB-first bit reader."""

    def __init__(self, data):
        self.data = data
        self.pos = 0
        self.bitbuf = 0
        self.bitcnt = 0

    def bits(self, need):
        val = self.bitbuf
        while self.bitcnt < need:
            if self.pos >= len(self.data):
                raise BlastError("out of input")
            val |= self.data[self.pos] << self.bitcnt
            self.pos += 1
            self.bitcnt += 8
        self.bitbuf = val >> need
        self.bitcnt -= need
        return val & ((1 << need) - 1)


class _Huffman:
    def __init__(self, rep):
        length = []
        for byte in rep:
            length.extend([byte & 15] * ((byte >> 4) + 1))
        self.count = [0] * (MAXBITS + 1)
        for l in length:
            self.count[l] += 1
        offs = [0] * (MAXBITS + 1)
        for l in range(1, MAXBITS):
            offs[l + 1] = offs[l] + self.count[l]
        self.symbol = [0] * len(length)
        for sym, l in enumerate(length):
            if l != 0:
                self.symbol[offs[l]] = sym
                offs[l] += 1

    def decode(self, b):
        code = first = index = 0
        for length in range(1, MAXBITS + 1):
            code |= b.bits(1) ^ 1  # codes are stored complemented
            count = self.count[length]
            if code - first < count:
                return self.symbol[index + (code - first)]
            index += count
            first = (first + count) << 1
            code <<= 1
        raise BlastError("ran out of codes")


_LITCODE = _Huffman(bytes([
    11, 124, 8, 7, 28, 7, 188, 13, 76, 4, 10, 8, 12, 10, 12, 10, 8, 23, 8,
    9, 7, 6, 7, 8, 7, 6, 55, 8, 23, 24, 12, 11, 7, 9, 11, 12, 6, 7, 22, 5,
    7, 24, 6, 11, 9, 6, 7, 22, 7, 11, 38, 7, 9, 8, 25, 11, 8, 11, 9, 12,
    8, 12, 5, 38, 5, 38, 5, 11, 7, 5, 6, 21, 6, 10, 53, 8, 7, 24, 10, 27,
    44, 253, 253, 253, 252, 252, 252, 13, 12, 45, 12, 45, 12, 61, 12, 45,
    44, 173]))
_LENCODE = _Huffman(bytes([2, 35, 36, 53, 38, 23]))
_DISTCODE = _Huffman(bytes([2, 20, 53, 230, 247, 151, 248]))
_BASE = [3, 2, 4, 5, 6, 7, 8, 9, 10, 12, 16, 24, 40, 72, 136, 264]
_EXTRA = [0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8]


def blast(data):
    """Decompress a PKWARE DCL imploded buffer."""
    b = _Bits(data)
    lit = b.bits(8)
    if lit > 1:
        raise BlastError("bad literal flag")
    dict_ = b.bits(8)
    if dict_ < 4 or dict_ > 6:
        raise BlastError("bad dictionary size")
    out = bytearray()
    while True:
        if b.bits(1):
            symbol = _LENCODE.decode(b)
            length = _BASE[symbol] + b.bits(_EXTRA[symbol])
            if length == 519:
                return bytes(out)  # end code
            symbol = 2 if length == 2 else dict_
            dist = (_DISTCODE.decode(b) << symbol) + b.bits(symbol) + 1
            if dist > len(out):
                raise BlastError("distance too far back")
            for _ in range(length):  # overlapped copies must go bytewise
                out.append(out[-dist])
        else:
            out.append(_LITCODE.decode(b) if lit else b.bits(8))


def extract_shr(shr, datadir):
    """Extract the game files from a DN1SW20.SHR archive."""
    pos = 58  # self-extractor header
    count = 0
    while pos + 168 <= len(shr):
        name = shr[pos:pos + 16].split(b"\0")[0].decode("ascii")
        size = int.from_bytes(shr[pos + 136:pos + 140], "little")
        pos += 168
        blob = shr[pos:pos + size]
        pos += size
        if not name.upper().endswith(".DN1"):
            continue  # skip DOS extras (DN1.EXE, ORDER.FRM, ...)
        data = blast(blob)
        outname = os.path.join(datadir, name.upper())
        with open(outname, "wb") as f:
            f.write(data)
        print("  %-12s %6d bytes" % (name.upper(), len(data)))
        count += 1
    return count


def find_zip(target):
    here = os.path.dirname(os.path.abspath(__file__))
    for folder in (target, here, os.getcwd()):
        path = os.path.join(folder, ZIP_NAME)
        if os.path.isfile(path):
            return path
    return None


def main():
    target = os.path.abspath(sys.argv[1] if len(sys.argv) > 1 else ".")

    if not os.path.isfile(os.path.join(target, "NUKUM.TOS")):
        print("Note: no NUKUM.TOS found in %s" % target)
        print("      (installing the game data there anyway)")

    zippath = find_zip(target)
    if zippath:
        print("Using existing %s" % zippath)
        with open(zippath, "rb") as f:
            zipdata = f.read()
    else:
        zipdata = None
        for url in DOWNLOAD_URLS:
            print("Downloading %s ..." % url)
            try:
                with urllib.request.urlopen(url, timeout=30) as r:
                    zipdata = r.read()
                break
            except Exception as e:
                print("  failed: %s" % e)
        if zipdata is None:
            print()
            print("All downloads failed. Please download 1duke.zip")
            print("manually and place it next to this script, then")
            print("run it again.")
            return 1

    with zipfile.ZipFile(io.BytesIO(zipdata)) as z:
        shr = z.read(SHR_NAME)

    datadir = os.path.join(target, "NUKUM", "DATA")
    os.makedirs(datadir, exist_ok=True)
    print("Extracting to %s:" % datadir)
    count = extract_shr(shr, datadir)
    print("Done: %d files installed. Have fun!" % count)
    return 0


if __name__ == "__main__":
    sys.exit(main())
