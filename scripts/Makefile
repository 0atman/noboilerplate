
watch:
	cargo watch -c -i "*.rs"  -s "make build"

build:
	cat "Build Your Rust lightsaber.md" | literate --language rust > src/main.rs
	cargo script src/main.rs
