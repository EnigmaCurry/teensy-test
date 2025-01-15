# Teensy arduino test in Rust

This repository contains experimental arduino programs for the Teensy
4.1.

## Install Rust toolchain

 * Install Rust with [rustup](https://rustup.rs/).

## Install "Just"

```
cargo install just
```

The following `just` commands should be run in your terminal in the
directory containing this repository (it also works from any
subdirectory).

## Install Rust toolchain dependencies

```
just deps
```

## Install system dependencies

To be able to read logs over USB serial connection, install
[picocom](https://github.com/npat-efault/picocom):

```
sudo dnf install picocom
```

Install
[teensy_loader_cli](https://github.com/PaulStoffregen/teensy_loader_cli),
which you need to compile from source code.

```
## You'll need libusb (0.1) development libraries:
sudo dnf install libusb-compat-0.1-devel
```

```
## Build it:
git clone https://github.com/PaulStoffregen/teensy_loader_cli \
  ~/git/vendor/PaulStoffregen/teensy_loader_cli
cd ~/git/vendor/PaulStoffregen/teensy_loader_cli
make
```

```
## Install it:
sudo install teensy_loader_cli /usr/local/bin/teensy_loader_cli
```

## Install udev rules for Teensy boards

```
just udev-rules
```

## Create a new sub-crate using the teensy4-rs template

See [teensy4-rs](https://github.com/mciantyre/teensy4-rs)

```
just template hello-world
```

> **Note**: [hello-world](hello-world) has already been created in
> this repository, you must choose a new name for your new sub-crate.

Make sure the new module (e.g., `hello-world`) is added the root
[Cargo.toml](Cargo.toml) `members` section.

## Build sub-crate

```
# Only build it:
just build hello-world
```

## Upload program to Teensy board

To upload, you must set the board into programming mode:

 * Press the button on the Teensy board and a small red LED should illuminate.

```
# Build and install it:
just upload hello-world
```

## Read logs over USB serial connection

```
just logs
```

> **Note**: To quit picocom, press `Ctrl-q`. In Emacs vterm you need
> to press `C-a C-q`.

## Open documentation in your browser

```
just doc
```
