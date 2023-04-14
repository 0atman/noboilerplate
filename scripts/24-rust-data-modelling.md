<style>
:root {--r-code-font: "FiraCode Nerd Font";}
.reveal .hljs {min-height: 50%;}
</style>
%%

f7f7f7 background slide colour

# Cargo.toml

```toml
[package]
name = "rust-data-modelling"
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
	println!("rust data modelling");

```

%%

![[rust-logo.png]]

# invalid states are invalid code

notes:
%%
- Tell them what you're going to tell them
- Tell them
- Tell them what you told them
%%
Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

---

## Public Domain Videos

[github.com/0atman/noboilerplate/](https://github.com/0atman/noboilerplate/)

notes:
Everything you see in this video from the script to the images are part of a markdown document available on github under a public domain license.

---

```rust
struct Point {
    x: u32,
    y: u32,
}
```

```rust
enum WebEvent {
    Click(Point),
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
}
```

(adapted from [_Rust By Example_](https://doc.rust-lang.org/stable/rust-by-example/custom_types/structs.html))

notes:

- one of my favourite features in rust is the type system
- it doesn't just check you've used the right type of a a variable but allows you to build your own complex type heirachy with deep, rich integration with the domain you are modelling

Most popular languages have product types, the first example here of a Point.
This is like a class or a c-style struct. It's a container for a number of attributes.

The second type here, an enum, is not typically a core part of popular languages.
Enums, when they exist, are typically an enumeration over a set of integers, assigning them value.
Bit flags or types of a message.

They're rubbish.

---


```rust
struct SchrodingersCat {
	alive: bool
}
```

```rust
enum RealCat {
	Dead,
	Alive
}
```

```rust[]
let fakecat = SchrodingersCat{alive: true};
if fakecat.alive {
	println!("good good")
}
let realcat = RealCat::Alive;
match realcat {
	RealCat::Alive => println!("good good")
}
```

notes:

The difference between these is simple.
To know if Schrodinger's cat is alive, we must look inside the metaphorical box at runtime.
With a RealCat, you know if it's alive at COMPILE time.

There's nothing to check with the real cat - the only two valid states are Alive and Dead, and because you've enforced this with the type system, you gain safety, guarentees, and superpowers.

Rust stops you making mistakes, this is not valid Rust, I have made a mistake.
Can you see it?

---

```js
$ cargo build
non-exhaustive patterns: `RealCat::Dead` not covered
  --> src/main.rs:32:7
   |
32 | match realcat {
   |       ^^^^^^^ pattern `RealCat::Dead` not covered
   |
   = note, the matched value is of type `RealCat`
help: ensure that all possible cases are being handled by
adding a match arm with a wildcard pattern or an
explicit pattern as shown
   |
33 ~     RealCat::Alive => println!("good good"),
34 ~     RealCat::Dead => todo!()
```

notes:
Unlike an if statement, which can do anything and is not bound to the type system, a match statement works with the type system.

We're going to use the match expression extensively today, so let's look at what it can do

---

```rust
let number = 13;

println!("Tell me about {}", number);
match number {
	1                  => println!("One!"),
	2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
	13..=19            => println!("A teen"),
	_                  => println!("Ain't special"),
}
```

(adapted from [_Rust By Example_](https://doc.rust-lang.org/stable/rust-by-example/custom_types/structs.html))

notes:

Rust provides pattern matching via the `match` keyword, which can be used like a C `switch`. The first matching arm is evaluated and all possible values must be covered.

Here we are matching on, in order:
- a value
- a set of values
- a range of values
- or wildcards

Note that we are using the runtime value inside the integer here.

To get the safety, you must use the rich type system, not just numbers and strings.



---

- Normalisation explanation
- State machine explanation
- bring them all together

---

## Part 1:
# Algebraic Type Systems

---

## Part 2: 
# Data normalisation 

---

## Part 3:
# State machines

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
