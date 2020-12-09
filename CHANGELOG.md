# Freenukum Changelog

## [0.3.2] - 2020-12-09

### Fixed
- Fix loading of shootable wall
- Properly absorb shots when hitting actors
- Make score, particles and singleanimation act while invisible, so they
  don't reappear when they went out of view
- Make jumping mines absorb the shot

## [0.3.1] - 2020-12-08

### Added
- Show a disclaimer about the game version being under development
- Initial controller support (tested with 8BitDo SN30 Pro and Pro+)

### Fixed
- Fix fan range for pushing away the hero
- Fix a whitespace problem in the episode switch message
- Look for a TTF font on in the windows system font path when running on
  Windows

## [0.3.0] - 2020-12-07

### Added
- Extra `freenukum-data-tool` command-line application for managing the
  original data files
- Menu can now be controlled by cursor keys and mouse
- Improved collision detection system
- Implemented rocket (LP: #257550)
- Implemented rotating mill (LP: #242120)

### Changed
- Relicensed under AGPL-3.0
- Transitioned implementation to Rust
- Url is now https://gitlab.com/silwol/freenukum/

### Removed
- Built-in assistant for downloading the shareware was removed


## [0.2.10] - 2008-07-31

### Added
- Assistant which can download the shareware episode
- Fire wheel bot (LP: #242134)


## [0.2.9] - 2008-07-06

### Added
- Implemented fan wheels (LP: #242128)
- No more solid parts behind cameras (LP: #242133)
- Added fullscreen mode (LP: #245274)
- Implemented glove slot which can expand floor
- Soda can fly away when shot by hero (LP: #242118)
- Freenukum can now also load lowercase named data files (LP: #245269)


## [0.2.8] - 2008-07-03

### Fixed
- Compile bug on AMD64 (LP: #244763)

## [0.2.7] - 2008-07-01

### Added
- Implemented shots by enemies (LP: #243990)
- Implemented bombs (LP: #242116)
- Added background (LP: #240832)
- Added particle fireworks (LP: #242117)
- Added menu shortcut .desktop file
- Added tankbot (LP: #241105)

### Changed
- Removed dependency on glib (LP: #243956)


## [0.2.6] - 2008-06-28

### Changed
- Using BitsPerPixel and SDL-Flags from initial screen (LP: #243352)
- Calling SDL_Quit on exit of the program (LP: #243597)
- Distribution tarball includes manpage (LP: #243296)
- `fn_test_*` programs can now be compiled using
  `./configure --enable-testprograms`


## [0.2.5] - 2008-06-23

### Fixed
- Made freenukum compile on AMD64 (LP: #242228)

## [0.2.4] - 2008-06-22

### Added
- Implemented flamethrower
- Added ACME stone
- Added Manpage
- Implemented mines
- Implemented floor which crashes when hero steps upon it twice

### Fixed
- Shots can no longer go through solid parts


## [0.2.3] - 2008-06-18

### Added
- Implemented balloons
- Implemented wall-crawling green bots
- Made it possible to change to other episodes (if installed).

### Changed
- When hero gets hurt he has an immunity phase before he is hurt again


## [0.2.2] - 2008-06-15

### Added
- Implemented simple robot
- Made hero motion model more similar to original game.
- Elevators implemented


## [0.2.1] - 2008-06-14

### Added
- Hero can fetch guns for more firepower.
- Hero can fetch boots and then jump higher.
- Hero can use teleporters.
- Hero can shoot boxes.
- Hero can walk through opened doors immediately instead of having
  to wait until the animation finishes.
- Hero earns score for fetching keys.
- Keys can now be used.

### Fixed
- Keyhole stops blinking after key insert.
- After finishing a level, we go on to the next instead of returning
  to the title screen


## [0.2] - 2008-06-11

### Fixed
- Exit doors work now


## [0.1] - 2008-03-18

### Added
- Initial development release
