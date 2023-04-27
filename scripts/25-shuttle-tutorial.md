<style>
:root {--r-code-font: "FiraCode Nerd Font";}
.reveal .hljs {min-height: 50%;}
</style>
%%

f7f7f7 background slide colour

# Cargo.toml

```toml
[package]
name = "template"
version = "0.1.0"
edition = "2021"

[build-dependencies]

[dev-dependencies]

[dependencies]
```

# Lint Tweaks

```rust
#![allow(dead_code)]
#![allow(unused_variables)]
```

# Extern Crates

```rust

```

# Imports

```rust
```

# Setup

```rust
fn main() {
	println!("Rust talk");

```

%%

![[rust-logo.png]]

# RUST: TITLE

notes:
%%
- Tell them what you're going to tell them
- Tell them
- Tell them what you told them
%%
Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

---

%%
Getting started with Shuttle: [https://docs.shuttle.rs/introduction/quick-start](https://docs.shuttle.rs/introduction/quick-start)  
Local run: [https://docs.shuttle.rs/introduction/local-run](https://docs.shuttle.rs/introduction/local-run)

Axum & Static files example: [https://github.com/shuttle-hq/examples/tree/54e3617a528dc32e5b9a1fe8514fc4f57bd0a4a9/axum/static-files](https://github.com/shuttle-hq/examples/tree/54e3617a528dc32e5b9a1fe8514fc4f57bd0a4a9/axum/static-files)

Shared database docs: [https://docs.shuttle.rs/resources/shuttle-shared-db](https://docs.shuttle.rs/resources/shuttle-shared-db) (Postgres)

Static folder docs: [https://docs.shuttle.rs/resources/shuttle-static-folder](https://docs.shuttle.rs/resources/shuttle-static-folder)  
%%


# setup
- rustup
- bacon

## shuttle
cargo install cargo-binstall
(also get cargo install cargo-update)
cargo binstall cargo-shuttle
cargo shuttle login



---

## Public Domain Videos

[github.com/0atman/noboilerplate/](https://github.com/0atman/noboilerplate/)

notes:
Everything you see in this video from the script to the images are part of a markdown document available on github under a public domain license.

---

# CONTENT HERE

---

![[tri-hex-moon-white-transparent.png|300]]

# Thank you
## [Patreon.com/NoBoilerplate](http://www.patreon.com/noboilerplate)

notes:

# OUTRO

If you would like to support my channel, get early ad-free and tracking-free videos and vip discord access head to patreon.com/noboilerplate.

If you're interested in transhumanism and hopepunk stories, please check out my sci-fi podcast, Lost Terminal.

Or if urban fantasy is more your bag, do listen to a strange and beautiful podcast I produce called Modem Prometheus.

Transcripts and compile-checked markdown sourcecode are available on github, links in the description, and corrections are in the pinned ERRATA comment.

Thank you so much for watching, talk to you on Discord.

```rust
  println!("That's all folks!");
} 
```
