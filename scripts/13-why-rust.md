%%
<style>
:root {--r-code-font: "FiraCode Nerd Font";}
</style>

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
	println!("Rust talk");

```
%%

![[rust-logo.png]]

# Rust is boring
### Why Rust?

notes:

Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

---

thread 
- Rust isn't trendy, it's boring.

You know what isn't boring? seafaring in the ancient Mediterranean - everyone knew that you'd just lose ships to the sea now and then.

So it was, so it has always been.

At the edge of the map was labelled "here be dragons".

Rust is the opposite of "here be dragons"


---
# linus toravlds is not an easy man to please.

C++ is not good enough for this man.

Rust is
![[linus-easy-going.png]]

---



# Jack loves rust 

https://twitter.com/jack/status/1474263588651126788
![[jack-rust-perfect.png]]

---

GC killed Discord's hot path 
https://discord.com/blog/why-discord-is-switching-from-go-to-rust
- **Even with just basic optimization, Rust was able to outperform the hyper hand-tuned Go version.**
- After a bit of profiling and performance optimizations, **we were able to beat Go on every single performance metric**. Latency, CPU, and memory were all better in the Rust version.

![[discord-go-vs-rust.png]]

---

Cloudflare outgrew NGINX
https://blog.cloudflare.com/how-we-built-pingora-the-proxy-that-connects-cloudflare-to-the-internet/

> Pingora crashes are so rare we usually find unrelated issues when we do encounter one. Recently we discovered [a kernel bug](https://lkml.org/lkml/2022/3/15/6) soon after our service started crashing.
---

![[rust-logo.png]]

# Subtitle 


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