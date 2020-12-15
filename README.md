# Freenukum

A clone of the 1991 DOS game *Duke Nukem 1*.

The game is still work in progress. Expect many things to not function yet,
such as opponents and some other items missing from the levels.

## How to install

For now, no compiled executable files are built or distributed by the
developers of this project, instead the source must be compiled by the
users.

The concrete building steps may vary depending on the environment. Here
is a list of a few systems and how to install it there. The generic
information can be found in [the Rust
book](https://doc.rust-lang.org/book/ch14-04-installing-binaries.html).

If your desired system is not listed and you'd like to contribute the guide
for it, please either submit
[an issue](https://gitlab.com/silwol/freenukum/-/issues) or
[a merge request](https://gitlab.com/silwol/freenukum/-/merge_requests).

### Debian GNU/Linux or derived distributions such as Ubuntu or Mint

* Install the required development libraries and the Rust compiler:
  `sudo apt install rustc libsdl2-dev libsdl2-ttf-dev`.
* Build and install the game with `cargo install freenukum`.
* The files are installed to `~/.cargo/bin`. You can add this to your
  `PATH` environment variable, or run the data tool and the game with
  `~/.cargo/bin/freenukum-data-tool` and `~/.cargo/bin/freenukum`.

## Game data

Currently the game requires the installation of the original game graphics
data. The project includes the `freenukum-data-tool` executable which can
be used for installation.

The game data can be obtained:
* Shareware episode from ftp://ftp.3drealms.com/share/1duke.zip (free of
  charge)
* Search on https://archive.org/ for it.
* The Duke Nukem 3D CD contains a copy of the full version.
* Buy it from an online store if you find it. It used to be available on
  [GOG.com](https://www.gog.com/news/release_duke_nukem_12), but that is no
  longer the case. Maybe it will be available some time in the future
  again.

The home of FreeNukum is <https://gitlab.com/silwol/freenukum>.
