[![Rust](https://github.com/TelluricDeckay/telluricdeckay/workflows/Rust/badge.svg?branch=trunk)](https://github.com/TelluricDeckay/telluricdeckay/actions?query=workflow%3ARust)

# telluricdeckay

A multiplayer network poker game

## Status

In early development, not ready for use yet

## Build and Run

    cargo build
    cargo run

## Installation

No distribution packages are available. To install from source:

    autoconf (only needs to be run once, unless configure.ac is changed)
    ./configure --prefix=<dir> (or with no prefix given; defaults to /usr/local)
    cargo build --release
    make install

If '<dir>' is /usr, the binary will be installed to /usr/games, the data into
/usr/share/...

To install without root privileges, try one of these when running
`configure` above:

    ./configure --prefix=$HOME/.local
    ./configure --prefix=$PWD/pkg

The prefix(path) to the data directory will be built into the binary.

(Packagers: [DESTDIR](https://www.gnu.org/prep/standards/html_node/DESTDIR.html)
can be used with make, but the binary will still look for data using
the absolute path based on the prefix value given to configure.)

To uninstall:

    make uninstall

(If DESTDIR was used with 'install', it must be also be used with 'uninstall'.
