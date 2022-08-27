
watch:
	make build
	cargo watch -c -i "*.rs" -i "*.toml" -s "make build"

build:
	mkdir src -p
	cat "9 Railway Oriented Rust.md" | literate --language rust > src/main.rs
	cat "9 Railway Oriented Rust.md" | literate --language toml > Cargo.toml
	cargo clippy

deps:
	cargo install literate bacon
