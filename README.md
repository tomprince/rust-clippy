# Rust Belt Rust Workshop

## Setup Guide

1. install `rustup` from https://rustup.rs
2. `rustup update nightly`
3. `git clone --branch rust_belt_rust git@github.com:Manishearth/rust-clippy.git`
4. `cd rust-clippy`
5. `rustup override set nightly`
6. `cargo test`

Step 6 is necessary so you don't have to download any crates at the conference itself. Also, in case the tests fail, please post an issue in the clippy repository.
