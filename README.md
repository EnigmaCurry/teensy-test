# Teensy arduino test

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

## Install dependencies

```
just deps
```

## Install udev rules for Teensy boards

```
just udev-rules
```

## Create new crate from teensy4-rs template

See [teensy4-rs](https://github.com/mciantyre/teensy4-rs)

```
just template hello-world
```
