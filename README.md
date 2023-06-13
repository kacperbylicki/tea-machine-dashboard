## Guide

1. `cargo install cross`
2. `rustup target add armv7-unknown-linux-gnueabihf`
3. `docker build -t ghcr.io/cross-rs/armv7-unknown-linux-gnueabihf:0.2.5 .`
4. `cd src-tauri`
5. `cross build --target armv7-unknown-linux-gnueabihf --release`
