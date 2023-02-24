# Tourmaline
Tourmaline is a WIP operating system written in Rust.

## Goals
Inspired by [nebulet](https://github.com/nebulet/nebulet), I wanted to try supporting WASM programs running in ring 0 as an alternative to using ring 3. I do not have much experience creating operating systems, and I'm not super experienced with WASM, but it seemed like a fun project.

## Cranelift support
Like mentioned in the README in the cranelift-no_std subdirectory, I have modified a no_std compatible version of cranelift made by jyn514. I could not get any regular version of cranelift compiling on no_std, even though it's marked as no_std on crates.io. Part of this is mostlikely my fault, but with jyn514's fork of an old version of Cranelift and my own small modifications to it and wasmparser, I got this version compiling on no_std for this project. Huge thanks for jyn514 and the Cranelift developers for their amazing work!
A lot of new code has also been taken from an old version of wasm-environ. README will be updated soon to reflect this properly.
