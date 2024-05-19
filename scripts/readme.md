#!/usr/bin/env -S sh -c "cargo install -q --all-features literate cargo-watch mask mprocs bacon && mask --maskfile readme.md mprocs"

## Literate Programming Video scripts

This markdown file is in [mask](https://lib.rs/crates/mask) format, all commands are defined here.

Usage:
1. Install the rust toolchain (probably from <https://rustup.rs> with `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
2. Execute this readme to get the deps and run bacon
   (yes literally `./readme.md`, trust me, unix is magic)

## mprocs

> Spawn bacon and the long-running watcher with mprocs.

```sh
mprocs "bacon clippy" "mask --maskfile readme.md watch"
```

## watch

> Rebuilds project when rs and toml files change

```sh
mask build
cargo watch -c -i "*.rs" -i "*.toml" -s "mask --maskfile readme.md build"
```

## build

> Pull out rust and toml code fences into their files

```sh
mkdir src -p
cat 40-compiler-driven-development.md | literate --language rust > src/main.rs
cat 40-compiler-driven-development.md | literate --language toml > Cargo.toml
echo "done"
```
