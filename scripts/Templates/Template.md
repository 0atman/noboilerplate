<style>
:root {--r-code-font: "FiraCode Nerd Font";}
.reveal .hljs {min-height: 50%;}
</style>
%%

f7f7f7 background slide colour
or maybe 191919

# Cargo.toml

```toml
[package]
name = "template"
version = "0.1.0"
edition = "2021"

[build-dependencies]

[dev-dependencies]

[lints.rust]
unsafe_code = "forbid"

# come at me, clippy
[lints.clippy]
enum_glob_use = "deny"
pedantic = { level = "deny", priority = -1 }
nursery = { level = "deny", priority = -1 }
unwrap_used = "deny"

[dependencies]
# defined later, during the video
```

# Lint Tweaks

These lints make clippy less noisy when I'm building the video

```rust
#![allow(dead_code, unused_variables)]
#![allow(clippy::items_after_statements, clippy::no_effect, unused_must_use, clippy::must_use_candidate, clippy::unused_self, clippy::missing_const_for_fn)]
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

![logo|400](attachments/tri-hex-moon-white-transparent.png)

# TITLE

notes:
Hi friends my name is Tris and this is No Boilerplate, where I make fast, technical videos.

---

![300|400](attachments/cc-zero-2k.png)

## Public Domain Videos

[https://github.com/0atman/noboilerplate/](https://github.com/0atman/noboilerplate/)

(for all [blue links]() read my scripts here ⬆)

notes:
My video scripts are dedicated to the public domain.

Everything you see here: script, links, and images are part of a markdown document available freely on GitHub at the above address.

---

# CONTENT HERE

---

# Thank You

Especially to my Producer and Sponsor patrons:

%%

```rust
fn credits() {
```

%%

```rust
let producers: [&str; 0] = [];
let sponsors = [
	"Jaycee", "Gregory Taylor", "Ything LLC", "Taylor Dolezal"
];
```

%%

```rust
}
```

%%

### [Patreon.com/NoBoilerplate](http://www.patreon.com/noboilerplate)

### [ko-fi.com/noboilerplate](https://ko-fi.com/noboilerplate)

notes:

# OUTRO

Thank you.

If you would like to support my channel, get early ad-free and tracking-free videos, your name in the credits or 1:1 mentoring, head to my patreon or ko-fi.

If you're interested in transhumanism and hopepunk, please check out my weekly sci-fi audiofiction podcast, Lost Terminal.

Or if urban fantasy is more your bag, do listen to a strange and beautiful podcast I produce every full moon called Modem Prometheus.

Transcripts and compile-checked markdown sourcecode are available on github, links in the description, and corrections are in the pinned ERRATA comment.

Thank you so much for watching, talk to you on Discord.

```rust
  println!("That's all folks!");
} 
```
