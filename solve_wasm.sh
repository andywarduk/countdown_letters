#!/bin/sh

# relies on cargo-wasi being installed: cargo install cargo-wasi [--force]
# target installed with: rustup target add wasm32-wasi

CARGO_TARGET_WASM32_WASI_RUNNER="wasmtime --dir=." cargo wasi run --quiet --release --bin solve -- $@ 2>&1 | less -R
