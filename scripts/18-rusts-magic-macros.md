<style>
:root {--r-code-font: "FiraCode Nerd Font";}
.reveal .hljs {min-height: 50%;}
</style>
%%

f7f7f7 background slide colour

# Cargo.toml

```toml
[package]
name = "magic-macros"
version = "0.1.0"
edition = "2021"

[build-dependencies]

[dev-dependencies]

[dependencies]
macro_lisp = "0.1.0"
```

# Lint Tweaks

```rust
#![allow(dead_code)]
#![allow(unused_variables)]
```

# Extern Crates

```rust
#[macro_use]
extern crate macro_lisp;
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

Compiler-checked markdown video scripts:
[github.com/0atman/noboilerplate](github.com/0atman/noboilerplate)

notes:

Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

You and I would not be here today if it weren't for my favourite feature of rust, and indeed of any language: Macros.

I've mentioned them in the past, but today I'm going to explain why they're SO POWERFUL.

---

## Open Source Videos

<https://github.com:0atman/noboilerplate/>


notes:

As ever, all Rust code you see in this video is part of a literate programming document that can be extracted and compiled with native Rust tooling.

---

# We Need to Talk about Macros

notes:

Most articles about macros minimise their most powerful feature, compile-time code execution, instead focusing on their ability to dry up your code, as if they were just a metaprogramming or way to save keystrokes.

---

```md[9]
Conditionals. 
A function type. 
Recursion.
Pass-by-reference.
Garbage-collection.
Programs composed of expressions, not statements.
A symbol type.
A notation for code using trees of symbols.
The whole language always available.
```

_"What Made Lisp Different"_

&mdash; Paul Graham

notes:

Let's be clear, macros are the most powerful technique way to decrease boilerplate we have.

We've known this for half a centuary, they were one of the key innovations in lisp.

Most languages have implemented about half of this list. Rust has nearly everything including macros.

---

# Examples of ~~Witchcraft~~

# Macros

notes:

Here are 4 examples of impossible things in other languages that Rust can do before breakfast.

---

Entirely new syntax

```rust
html! {
  <div id="my_div"></div>
};
```

---

Entirely new syntax

```rust
lisp!(defun factorial ((n i32)) i32
  (if (<= n 1)
    1
    (* n (factorial (- n 1)))));

lisp!(defun main () ()
    (defconstant num (factorial 10))
    (println "10! = {}" num));
```

<https://crates.io/crates/macro_lisp>

---

- simple syntax rewrite
- advanced: dsl lisp
- compile execution sqlx
- larger blocks

---

![[rust-logo.png]]

# Subtitle

notes:

# OUTTRO

If you would like to support my channel, get early ad-free and tracking-free videos and vip discord access head to patreon.com/noboilerplate.

If you're interested in transhumanism and hopepunk stories, please check out my sci-fi podcast, Lost Terminal.

Or if urban fantasy is more your bag, click the bottom video to listen to a strange and beautiful podcast I produce called Modem Prometheus.

Transcripts and compile-checked markdown sourcecode are available on github, links in the description, and corrections are in the pinned ERRATA comment.

Thank you so much for watching, talk to you on Discord.

```rust
  println!("That's all folks!");
} 
```
