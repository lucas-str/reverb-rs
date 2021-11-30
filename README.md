# Reverb VST

A simple reverb VST plugin written in Rust.

## Build

The mingw-w64-gcc package is required to build for Windows (on Arch) :

    sudo pacman -S mingw-w64-gcc

Then build with :

    cargo build --target=x86_64-pc-windows-gnu --release

Then use `target/x86_64-pc-windows-gnu/release/reverb.dll` in your favorite
DAW.
