
watch:
	make build
	cargo watch -c -i "*.rs" -i "*.toml" -s "make build"

build:
	mkdir src -p
	cat "9 Rust on Rails.md" | literate --language rust > src/main.rs
	cat "9 Rust on Rails.md" | literate --language toml > Cargo.toml

deps:
	cargo install literate bacon
