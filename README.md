# FreeNukum for Atari ST

<img src="./doc/atarist-splash.png" alt="Come get STome!" width="640" height="400"/>

Feature complete port of Duke Nukem, Episode 1 to Atari ST by
[Neil Rackett](https://x.com/neilrackett).

## Come get STome!

1991 saw the arrival of two 90s icons: Duke Nukem and the Atari Mega STE, but
sadly they never met... _Until now!_

To celebrate their 35th Birthday I've ported the full Episode 1 (shareware
version) of Duke Nukem to the Atari ST: it wants 2MB, has experimental support
for 1MB, and of course includes support for Mega STE features like Blitter and
16MHz mode.

You can download the latest version from the
[releases page](https://github.com/neilrackett/atarist-freenukum/releases).

## Controls

| Key     | Joystick                | Action                               |
| ------- | ----------------------- | ------------------------------------ |
| ← / →   | ← / →                   | Walk left / right                    |
| Ctrl    | ↑ (when nothing to use) | Jump                                 |
| Alt     | Fire                    | Fire                                 |
| ↑ / ↓   | ↑ / ↓                   | Use doors, elevators and other items |
| Return  |                         | Select menu item                     |
| Esc / Q |                         | Quit level / back to menu            |
| F1      |                         | Help                                 |

## System requirements

- Atari ST/STE, Mega ST/STE or TT
- 2MB RAM recommended: the whole level strip stays in memory, which makes
  horizontal scrolling cheaper
- 1MB is **experimental** - it fits by narrowing the level strip and
  re-anchoring it to the camera, and has only been tested on level 1
- ST low resolution
- A hard disk with the game files:
  - `NUKUM.TOS`
  - `NUKUM\DATA\*.DN1` (see below)

## Game data

The game requires the original game graphics and level data, extracted into
the `NUKUM\DATA` folder alongside `NUKUM.TOS`.

The easiest way to install the freely distributable shareware episode is the
bundled installer — run it from the folder containing `NUKUM.TOS`:

- **macOS / Linux:** `sh installer/install.sh`
- **Windows:** `installer\install.bat`

Alternatively, the game data can be obtained:

- Shareware episode from [ftp://ftp.3drealms.com/share/1duke.zip](https://ftp.zx.net.nz/pub/archive/ftp.3drealms.com/share/)
- Search on https://archive.org/ for it.
- The Duke Nukem 3D CD contains a copy of the full version.
- Buy it from an online store if you find it.

## Building

SDL is replaced by [STDL](https://github.com/neilrackett/atarist-stdl), a
planar-native subset of SDL 1.2 for the Atari ST. It is a submodule at
`lib/stdl`, pinned to a release tag, so clone with it:

```sh
git clone --recurse-submodules https://github.com/neilrackett/atarist-freenukum.git
```

In a clone that already exists, `git submodule update --init` fetches it.

Then build with
[atarist-toolkit-docker](https://github.com/sidecartridge/atarist-toolkit-docker)
(`m68k-atari-mint-gcc` via the `stcmd` wrapper):

```sh
stcmd make -f Makefile.atari
```

`libstdl.a` is built from the submodule as part of that, so there is nothing
to build first. The executable is built to `dist/NUKUM.TOS` and the loading
screen to `dist/SPLASH.PI1`, with no dependencies beyond the game data files.

Set `STDL=/path/to/atarist-stdl` to build against a checkout of the library
elsewhere; a sibling `../atarist-stdl` is picked up automatically when the
submodule is absent. Note that the toolkit container only mounts the folder
it starts in, so building against a checkout outside this one also needs
`ST_WORKING_FOLDER` set to a common parent.

## To do

The ST version already goes well beyond the original FreeNukum to implements all
of the shareware episode's enemies — including the flying bot, rabbitoid,
helicopter, snake bot, flame gnome and the Dr Proton encounter, which ends
level 11 when you defeat him — and shows the episode ending sequence when you
finish the game, but it would be great to go even further and implement:

- Save / restore game (the menu entry does nothing)
- High scores and cheat mode
- Demo playback ("Previews/Main Demo!" and "View user demo")
- Water mirror effect (water is solid but does not reflect)
- Episodes 2 and 3 (the episode change menu exists, but is untested and needs
  the `.DN2` / `.DN3` data files)
- The sound effects belonging to the features above

## Original FreeNukum

A massive thank you to the original [FreeNukum](https://gitlab.com/silwol/freenukum)
project, without which this port would not have been possible.

To find more about the original FreeNukum project, check out the
[project page](http://launchpad.net/freenukum) at or take a look at the
[INSTALL](./INSTALL) file.
