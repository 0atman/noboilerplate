
watch:
	make build
	cargo watch -c -i "*.rs" -i "*.toml" -s "make build"

build:
	mkdir src -p
	cat "15-rust-is-easy.md" | literate --language rust > src/main.rs
	cat "15-rust-is-easy.md" | literate --language toml > Cargo.toml

deps:
	cargo install --all-features literate bacon cargo-watch
