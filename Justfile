set export

current_dir := `pwd`

# print help for Just targets
help:
    @just -l

# Install Rust toolchain dependencies
deps: 
    rustup install nightly
    rustup override set nightly
    rustup target add thumbv7em-none-eabihf
    echo "nightly" > rust-toolchain
    cargo install cargo-binutils
    rustup component add llvm-tools-preview
    cargo install cargo-generate

# Install udev rules for Teensy board types
udev-rules:
    sudo cp udev-rules.txt /etc/udev/rules.d/00-teensy.rules
    sudo udevadm control --reload-rules
    @echo "Make sure to physically unplug and then replug your teensy USB cable."

# Create new crate from teensy4-rs template
template *args:
    cargo generate --git https://github.com/mciantyre/teensy4-rs-template --name {{args}}

    
