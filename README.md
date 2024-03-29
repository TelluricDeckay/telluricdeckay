[![Rust](https://github.com/TelluricDeckay/telluricdeckay/workflows/Rust/badge.svg?branch=trunk)](https://github.com/TelluricDeckay/telluricdeckay/actions?query=workflow%3ARust)

# telluricdeckay

A multiplayer network poker game

## Status

In early development, not ready for use yet

## Build and Run

If you only want to build and run, but not install, just copy
'config_h.rs.in' to config_h.rs, open with an editor, and replace
"@DATADIR@" with "/usr/local/share" (or some other arbitrary directory). You
can then skip the first two steps listed below.

    autoconf
    ./configure
    cargo build
    cargo run

## Installation

No distribution packages are available. Installation is not required to
run the game and is primarily intended for packagers or anyone who
wants to install a release.

    autoconf (only needs to be run once, unless configure.ac is changed)
    ./configure --prefix=<dir> (or with no prefix given; defaults to /usr/local)
    cargo build --release
    make install

If `<dir>` is /usr, the binary will be installed to /usr/games, the data into
/usr/share/...

Example for installing without root privileges:

    ./configure --prefix=$HOME/.local

for testing changes, try

    ./configure --prefix=$PWD/install_test

The prefix(path) to the data directory will be built into the binary.

(Packagers: [DESTDIR](https://www.gnu.org/prep/standards/html_node/DESTDIR.html)
can be used with make, but the binary will still look for data using
the absolute path based on the prefix value given to configure.)

To uninstall:

    make uninstall

(If DESTDIR was used with 'install', it must be also be used with 'uninstall'.

To remove all files generated by './configure'

    make distclean

## Contributing

See
[CONTRIBUTING.md](https://github.com/TelluricDeckay/telluricdeckay/blob/trunk/CONTRIBUTING.md)

## Help and Support

* [Discussions](https://github.com/TelluricDeckay/telluricdeckay/discussions)
* [Issues](https://github.com/TelluricDeckay/telluricdeckay/issues)
* Emails listed in [Cargo.toml](https://github.com/TelluricDeckay/telluricdeckay/blob/trunk/Cargo.toml)
