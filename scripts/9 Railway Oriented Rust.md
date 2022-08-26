%%
<style>
.reveal code.rust {
  font-size: 1.5em;
  line-height: 1.5em;
}
</style>

<style>
.reveal code.md {
  font-size: 1.5em;
  line-height: 1.5em;
}
.reveal code.sh {
  font-size: 0.7em;
  line-height: 1.2em;
}
</style>

# Cargo.toml 
```toml
[package]
name = "railway"
version = "0.1.0"
edition = "2021"

[build-dependencies]

[dev-dependencies]

[dependencies]
rand = "0.8.5"
rand_derive2 = "0.1.17"
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
	println!("testing");

```
%%

![[rust-logo.png]]

# RUST: TITLE
### SUBTITLE

notes:

Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.


---

# CONTENT HERE



---


# Ad spot for Ditto
- Today's video is sponsored by Ditto.
- Unlike other sponsors, ditto don't want your money, they actually want to pay you.
- This is because they've asked me to tell you about open Rust positions at their company.
- Here's what you'll want to know about them, their tech, and the open positions:
    - Ditto use rust to power their cross-platform data sync system
    - They're growing their team and looking for people passionate about Rust, if you're watching this video, that might be you.
    - The problems they are solving include mesh network, replication protocols, Conflict-free Replicated Data Types, and database design to name a few.
- They're looking for
    - demonstrable Rust experience or previous work with C/C++
    - Rust backend developers for their Big Peer cloud system
    - Low-level bare-metal coders working with FFI and C interop
    - Algorithm junkies to work on their data stores, and
    - Networking coders at either the low or high level to work on their actor-like frameworks in replication and multihop work.
- Links to learn more are
    - [https://www.ditto.live](https://www.ditto.live) and
    - [https://jobs.ashbyhq.com/ditto](https://jobs.ashbyhq.com/ditto),

---


![[rust-logo.png]]

# Subtitle 


notes:

# OUTTRO

If you'd like to see what you can write in rust, click the top video: I used it to make a fun retro computer visualisation for my hopepunk podcast, Lost Terminal.

Or if urban fantasy is more your bag, click the bottom video to listen to a strange and beautiful podcast I produce called Modem Prometheus.

Transcripts and compile-checked markdown sourcecode are available on github, links in the description, and corrections are in the pinned ERRATA comment.

Thank you so much for watching, talk to you on Discord.

```rust
} // That's all folks!
```


