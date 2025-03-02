export CC_x86_64_unknown_linux_musl := "clang"

watch:
    browser-sync start --proxy localhost:8002 --files target/debug/nnnhl2 &
    find js css src Cargo.toml | entr -rsc 'cargo run'

build:
    cargo build --release

build-static:
     cargo build --release --target=x86_64-unknown-linux-musl
