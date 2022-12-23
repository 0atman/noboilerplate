
watch:
	make build
	cargo watch -c -i "*.rs" -i "*.toml" -s "make build"

build:
	mkdir src -p
	cat "16-rust-testing.md" | literate --language rust > src/main.rs
	cat "16-rust-testing.md" | literate --language toml > Cargo.toml

deps:
	cargo install --all-features literate bacon cargo-watch
