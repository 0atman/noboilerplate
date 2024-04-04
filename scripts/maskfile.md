## Tasks For My Project

<!-- A heading defines the command's name -->

## watch

> Rebuilds project when rs and toml files change

```sh
mask build
cargo watch -c -i "*.rs" -i "*.toml" -s "mask build"
```

## build

> Pull out rust and toml code fences into their files

```sh
mkdir src -p
cat 40-compiler-driven-development.md | literate --language rust > src/main.rs
cat 40-compiler-driven-development.md | literate --language toml > Cargo.toml
echo "done"
```
