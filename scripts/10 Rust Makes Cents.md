<style>
:root {--r-code-font: "FiraCode Nerd Font";}
</style>

%%
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

# Lint tweaks
```rust
#![allow(dead_code)]
#![allow(unused_variables)]
```

# extern crates

```rust

```

# imports
```rust
```

# setup

```rust
fn main() {
	println!("Rust Makes Cents");

```
%%

![[rust-logo.png]]

# RUST Makes Cents
### Speed is a FEATURE

notes:

Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

Rust Makes Cents (cost is a feature)" - talking about the TCO of Rust, how it's cheap on CPU, RAM and indeed developer time, and the whole stack can be written in one language


---

- good fast cheap, pick three
- TCO of rust
- cheap on CPU
- cheap on RAM
- Cheap on developer time
	- not at first, but over the whole lifetime of the app
	- Fast tests
	- whole category of errors avoided
- The whole stack is one language
	- down at the hardware level in unsafe rust
	- great ergonomics like iter compile down to the same machine code as hand-crafted loops
	- build tools aren't separate, they're macros



---


![[rust-logo.png]]

# Speed is a FEATURE


notes:

# OUTTRO

If you'd like to see what you can write in rust, click the top video: I used it to make a fun retro computer visualisation for my hopepunk podcast, Lost Terminal.

Or if urban fantasy is more your bag, click the bottom video to listen to a strange and beautiful podcast I produce called Modem Prometheus.

If you would like to support my work, head to patreon.com/noboilerplate.
Transcripts and compile-checked markdown sourcecode are available on github, links in the description, and corrections are in the pinned ERRATA comment.

Thank you so much for watching, talk to you on Discord.

```rust
  println!("That's all folks!");
} 
```