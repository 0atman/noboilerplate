## Tasks For My Project

<!-- A heading defines the command's name -->

## Watch

> Rebuilds project when rs and toml files change

```sh
mask Build
cargo watch -c -i "*.rs" -i "*.toml" -s "mask Build"
```

## Build

> Pull out rust and toml code fences into their files

```sh
mkdir src -p
cat 37-functional-rust.md | literate --language rust > src/main.rs
cat 37-functional-rust.md | literate --language toml > Cargo.toml
echo "done"
```
