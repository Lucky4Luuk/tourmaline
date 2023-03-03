# Tourmaline
Tourmaline is a WIP operating system written in Rust.

## Goals
Inspired by [nebulet](https://github.com/nebulet/nebulet), I wanted to try supporting WASM programs running in ring 0 as an alternative to using ring 3. I do not have much experience creating operating systems, and I'm not super experienced with WASM, but it seemed like a fun project.

## How to compile
Use either Linux or WSL. By default, the project is set up for WSL, but to run it directly from Linux, simply set `RUN_AS_CMD` to `false` in `kernel/.cargo/runner.sh`.

### Required toolchain + targets
At the moment, using `nightly` is required. Also, make sure to have the `x86_64-unknown-none` and `wasm32-unknown-unknown` targets installed!

### Required tools
- git
- build-essential
- make
- xorisso
