<style>
:root {--r-code-font: "FiraCode Nerd Font";}
.reveal .hljs {min-height: 50%;}
</style>
%%

f7f7f7 background slide colour

# Cargo.toml

```toml
[package]
name = "rust-in-rust"
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
#![allow(clippy::items_after_statements)]
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

# Rust's Discoveries

_(Thanks to `@speykious` and `@laund` for their help)_

notes:
%%
- Tell them what you're going to tell them
- Tell them
- Tell them what you told them
%%
Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

---

![[xkcd-purity.png|1000]]

(don't learn about the Replication Crisis)

notes:

# INTRO

Maths is a WEIRD discipline.

What is maths based on?

Unlike the rest of the human world, where physicists are building on mathematics, chemists are building on physics, biologists are building on chemistry and so on, Maths is the floor of the abstraction.

---

# `1 = 1`

notes:

Maths is based on Axioms, fundamental self-evident rules.

The rules of our universe are pre-set, and mathematicians DISCOVER them, not INVENT them.

They're not something we've invented to explain the world, like quantum physics is, or string theory.

These rules exist, in perfect form, out there.
And what's true out there, is true here.

The machinery of the universe is built in applied mathematics, both natural and human-made.

Programming is the purest form of applied mathematics.

the machinery of the Rust language is built on the simple rules of functional programming and borrow checker.

Starting with good rules is a profound way to build a language.

You don't have to invent an error handling system in such a language.

You can discover it.

---

## Public Domain Videos

[github.com/0atman/noboilerplate/](https://github.com/0atman/noboilerplate/)

notes:
Everything you see in this video from the script to the images are part of a markdown document available on github under a public domain licence.

---

![[pexels-ali-arapoğlu-4432037.jpg|600]]

### Computers Exist

notes:

Computers exist.

This is a brave take, and I understand that many of you may disagree with me.

Rust acknowledges these truths:

- The CPU exists, and is fallible,
- memory and therefore pointers exists,
- and you can't always get what you want.

Purely high-level languages assume the underlying computer is a perfect machine that never fails and can be abstracted perfectly.

They then build on this assumption, and abstract the real machine out of the control of the developer.

The Rust developers understand that computers exist. But we're not C developers.

(Photo by Ali Arapoğlu: https://www.pexels.com/photo/desktop-motherboard-with-connectors-and-microchips-at-home-4432037/)

---

![[oppenheimer-blackboard.png|900]]

## Mathematics Exists

notes:

We understand that there is also MORE than the machine to help us.
We don't need to rely on pointer arithmetic, we can bring high-level functional mathematics to tackle our problems.

The CPU can only solve problems sequentially, one instruction at a time (perhaps with a few pipelined at a time, but my point stands).

The language of mathematics and higher-order functions are enormously powerful tools that we're still discovering uses for in our field.

Rust's built with real mathematics - both high-level and low level.

And the language developers hid none of this from you.

You may use it all, without compromising safety and ergonomics.

To show this, let me talk first about the compiler/userland split.

---

```toml[]
[package]
name = "rustc-main"
version = "0.0.0"
edition = "2021"

[dependencies]
rustc_driver = { path = "../rustc_driver" }
rustc_driver_impl = { path = "../rustc_driver_impl" }
rustc_codegen_ssa = { path = "../rustc_codegen_ssa" }
rustc_smir = { path = "../rustc_smir" }
```

[rust-lang/rust/compiler/rustc/Cargo.toml](https://github.com/rust-lang/rust/blob/master/compiler/rustc/Cargo.toml)
notes:

As you probably know, the Rust compiler is built in Rust.
Same as the Go compiler is written in Go, and many other languages too.

We call this feature self-hosting, when the compiler is written in the language it is compiling.

---

|        |        |          |
| ------ | ------ | -------- |
| C      | C++    | Dart     |
| Elixir | Go     | Haskell  |
| Java   | Kotlin | Nim      |
| OCaml  | PyPy   | **Rust** |
| Scala  | TypeScript | Zig      |
|        |        |          |

notes:
Here are some self-hosted languages you may have heard of.

The first version of a language can't be written in the language itself though, there's by definition no compiler yet, so it must be bootstrapped in an existing language.

C or assembly are common bootstrapping languages, but with Rust it was OCaml.

---

> At "OCaml Chipotle" you can have whatever you like...But once a month you go for lunch, and they'll refuse to make you a bowl, refuse to tell you why, and refuse to let you leave. And when you try to get help...you realize there's nobody nearby who you can ask.

[blog.darklang.com/first-thoughts-on-rust-vs-ocaml/](https://blog.darklang.com/first-thoughts-on-rust-vs-ocaml/)

notes:

OCaml is a high level ML family language, and the influences on Rust are clear. I talk a lot about Rust being similar to Haskell, but really, they're BOTH ML family languages.

Rust is just hiding inside C's clothing to sneak into the popular kids party.

A reminder that there are more Rust projects on github than Kotlin, Scala, and Swift.

We're here today because Rust isn't a fringe language, despite it's very strange roots, it's mainstream and is ready for use in production TODAY.

Let's compare Rust to another popular language, Go.

---

![[go-vs-rust.excalidraw]]

notes:

The split between userland and compiler is important, as it represents what you can and can't change day-to-day.

For instance, Go has a splendid garbage collector for automatically managing memory.
But if you want more control, perhaps you need to operate in an environment where a garbage collector is not desirable, such as webassembly, you have no option but to bring all of the go runtime with you, because it is bound up in the compiler.

In this specific usecase, there is a fork of the compiler, tinygo, that addresses some of these problems with working with go and WebAssembly.

But this graph isn't actually to scale, let me fix it.

---

![[go-vs-rust-expanded.excalidraw]]

notes:

Rust is both lower-level and higher-level than Go, and Java, and Python, and nearly all other popular languages.

This is in large part thanks to the incredible macro system, which is as powerful as it is in lisp, coupled with the unsafe system, that allows you to keep writing Rust when other languages have to reach for C, or build tools.

Both these features deserve their own videos, and I have made two already, watch my "turtles" video, pinned here, or my deep-dive into macros, my 'Witchcraft' video.

---

## High-level Functional Programming

## &

## Low-level Control

notes:

Rust's unique blend of extremely high level functional programming and low-level hardware access through the unsafe system is a REALLY weird combination.

Typically, languages are either high-level, like JavaScript, OR they're low-level, like C.

A reasonable historic assumption, they previously have been used for very different use cases.

Having both features streamlined into a very coherent language is great for me, as a web developer, I can start writing low-level bare-metal code in the same language I write my webassembly in, but it goes much deeper.

---

<style>
.red {
	color: red;
}
</style>

### Functional Programming + Unsafe

# ` => `

### Functional Programming <span class="red">×</span> Unsafe

notes:

When you START OFF with low-level control in a high-level functional language, you appear to get far more than the sum of the parts.

You've not got Haskell with pointers, or C with monads.

You've got something very, very new.

And when you go digging around in this new language, this new universe, you can discover, or rediscover, many incredible useful building blocks for our applications, that you would have had to build for yourself otherwise.

Let's dig deeper and find some examples of Rust's discoveries, buried in the bedrock of the standard library.

after a word from today's sponsor, Quadratic.

---

<!-- slide bg="rgb(37, 34, 43)" -->

![[quadratic-website.png]]

notes:

# Quadratic Sponsor

Quadratic are building spreadsheet for engineers and data scientists, built in Rust, Webassembly and WebGL.

---

<!-- slide bg="rgb(37, 34, 43)" -->
![[cell.png]]
notes:

Quadratic combines the functional data visualisation of a spreadsheet with the power of full programming languages, starting with Python

---

<!-- slide bg="rgb(37, 34, 43)" -->

![[quadratic-dataframe.png|700]]

notes:
Standard Python data science libraries are built-in.

---

<!-- slide bg="rgb(37, 34, 43)" -->
![[quadratic-micropip-demo.png|700]]

<!-- slide bg="rgb(37, 34, 43)" -->
notes:
In fact, because quadratic are using Pyodide inside webassembly, any pure python dependency can be installed, like this example of the faker library.

---

<!-- slide bg="rgb(37, 34, 43)" -->

![[quadratic-api-demo.png|700]]

notes:
Because all of python is running locally inside webassembly, complex work, such as here pulling data from an api, is possible.

---

<!-- slide bg="rgb(37, 34, 43)" -->

![[quadratic-fps.png|700]]

notes:

This is all running at 60fps on the gpu using webgl, all inside your browser.

---

<!-- slide bg="rgb(37, 34, 43)" -->

![[quadratic-section-zoom-in-out.gif]]

notes:
Quadratic built their infinite canvas on webgl, allowing for smooth scrolling and pinch to zoom.

---

<!-- slide bg="rgb(37, 34, 43)" -->
![[quadratic-gpt.png|500]]

notes:

They also have GPT integration, giving you a copilot or pair programmer while you're writing.

It's a fantastic product made by some nice people, and I'm delighted to say they are hiring.

---

<!-- slide bg="rgb(37, 34, 43)" -->

## Founding Software Engineer

#### (Rust, WASM)

<br/>
More info:

### <https://careers.quadratichq.com>

notes:

Quadratic are looking for:
- Great Rust engineers who have experience at a Startup and leading software architecture and implementation.

---

<!-- slide bg="rgb(37, 34, 43)" -->

![[Quadratic Logo.png]]

The infinite canvas spreadsheet with code

#### https://QuadraticHQ.com

#### <https://careers.quadratichq.com>

notes:

-   Apply today, and
-   Head to QuadraticHQ.com to try it out.

My thanks to quadratic for their support of this channel.

Back to our rust discoveries.

---

## Options

```rust[]
pub enum Option<T> {
    None,
    Some(T),
}
```

notes:

Rust's option type is not an opaque language feature, it's built in Rust's sum type: Enums.

Enums are a core component of Rust's rich type system.

You or I could build our own, should we wish to.

for practical examples of using enums, you can watch my video "rust data modelling without classes"

---

```rust
fn div(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}
fn try_division(dividend: i32, divisor: i32) {
    match div(dividend, divisor) {
        None => println!("{dividend} / {divisor} fail"),
        Some(quotient) => {
            println!("{dividend}/{divisor}={quotient}");
        },
}}
```

notes:

Rust does not have the concept of Null, instead using options ubiquitously throughout the language.

Nearly all other languages, old standards such as javascript, python, and java, and even recently-designed ones like Go and Kotlin, began with nulls, the billion-dollar mistake, only to realise their error and attempt a retro-fit of options.

This never works, and you can feel it in the ecosystem and even standard libraries of these languages.

The Rust universe, just like the mathematical universe doesn't have the human concept of nulls, an exception to the type system.

---

## [std::result::Result](https://doc.rust-lang.org/std/result/enum.Result.html)

```rust
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

notes:

If you have sum types, rust enums, you can build enormous parts of your standard library with this simple structure instead of building it in to the plumbing of the compiler.

You can use the rules of your universe to make what you want.

Here is the result enum, rust's answer to error passing.

---

```rust[]
let mut file = match File::open(&path) {
    Err(why) => panic!("can't open {path.display()}"),
    Ok(file) => file,
};
```

or

```rust[]
let mut file = File::open(&path)?; // to return the Err
```

notes:
And it allows you, without an exception system, to handle errors safer and in a more obvious way than any language with exceptions, which often happen implicitly.

In Rust there are many ways to deal with the result type, should you call a fallible function.
You can match on the error and handle it comprehensively, or use the idiomatic question-mark operator to early-return the result containing the error to the calling function.

The rust authors didn't build an exception system in the compiler, they were able to discover that by using normal rust sum types they didn't have to.

And this discoverability doesn't just apply to the high-level parts of the language.

---

# Move Semantics

```rust
fn destroy_box(b: Box<i32>) {
    println!("Destroying a box that contains {b}");
    // `b` is destroyed and the memory freed
}

fn move_demo() {
    let a = Box::new(5i32);
    destroy_box(a);
}
```

[doc.rust-lang.org/rust-by-example/scope/move.html](https://doc.rust-lang.org/rust-by-example/scope/move.html)

notes:

Here is part of the move semantics example from the excellent book Rust By Example

We create a 32-bit signed integer and allocate it on the heap.
and then move it into the destroy_box function.

In Rust we call this owned heap-allocated value a box.

As soon as the integer falls out of scope and the destroy_box function ends, the compiler inserts code to clean up the memory that was used.

This all happens deterministically at compile time, rather than at runtime, as a GC would do.

---

```rust
fn move_demo2() {
    let a = Box::new(5i32);
    drop(a);
}
```

notes:

# Drop Introduction

In normal code, we can of course use std::mem::drop to do this, which has the same effect.

In fact, it's deeper than that, it does exactly the same thing.

Let's look at the source of std::mem::drop:

---

## [std::mem::drop](https://doc.rust-lang.org/std/mem/fn.drop.html)

```rust
pub fn drop<T>(_x: T) {}
```

_THAT'S IT_

notes:

# Drop Code

THAT'S IT. This isn't the function signature, this is the function. It has NO BODY.

Drop is a function that takes any type of input parameter and just does nothing with it, meaning that when it finishes, immediately, rust will clean up the memory of whatever was passed in.

This function is not magic:

Because `_x` is moved into the function, it is automatically dropped before the function returns.

Using rust's normal move semantics.

---

## [std::convert::identity](https://doc.rust-lang.org/std/convert/fn.identity.html)

```rust
pub const fn identity<T>(x: T) -> T { x }
```

notes:

# Identity Code

The identity function is almost the same as drop.
It simply takes ownership of the input for a moment, then gives it back.

Identity is a very useful function in functional code, map an iterator with identity, and you get back the same iterator.
When functions are first class and can be passed around in lists and applied in sequence, you might need some of these to be "no-ops" depending on some logic. the identity function is that no-op.

High level Functional programming coupled with low-level control.

---

## [std::mem::copy](https://doc.rust-lang.org/std/mem/fn.copy.html#)

```rust
pub const fn copy<T: Copy>(x: &T) -> T {
    *x 
}
```

notes:

# Copy Code

Copy is similar.
The copy function just dereferences a copy type, creating a copy.

Note that this is not a how the Copy Trait is implemented, this is a helper function in the standard library to explicitly name what is often implicitly used.

We're discovering that by using the rules set out in the language, there are almost no special cases.

---

## Structs

```rust
struct Color {
    r: bool,
    g: bool,
    b: bool,
}

let black   = Color { r: false, g: false, b: false }; 
let blue    = Color { r: false, g: false, b: true  }; 
let green   = Color { r: false, g: true,  b: false }; 
let cyan    = Color { r: false, g: true,  b: true  }; 
let red     = Color { r: true,  g: false, b: false }; 
let magenta = Color { r: true,  g: false, b: true  }; 
let yellow  = Color { r: true,  g: true,  b: false };
let white   = Color { r: true,  g: true,  b: true  }; 
```

#### A Color Has 2 * 2 * 2 = 8 States

notes:

Structs and tuples are product types. The possible combinations you can make out of them is the product of the possible combinations of each individual field.

a bool has 2 possible states: true and false
a Color has 2 * 2 * 2 = 8 possible states, the product of the possible states of all its fields

---

## Enums

```rust
enum Cup {
    Plastic(Color),
    CompostablePlastic(Color),
    Glass, // no color, transparent glass
}
```

#### A Cup Has 8 + 8 + 1 = 17 States

notes:

Enums are sum types. The possible combinations you can make out of them is the sum of the possible combinations of each individual variant.

a reminder that a Color has 8 possible states

a Cup has 8 + 8 + 1 possible states: either it's one of the possible states of Cup::Plastic, or
one of the possible states of Cup::CompostablePlastic, or it's Cup::Glass which is only one state

---

### Struct (product type)

```rust
struct Unit;

let my_unit = Unit;
```

### Enum (sum type)

```rust[]
enum Infallible {}

// compile error, no variants = can't instantiate
let impossible = Infallible::no_varients_found;
```

notes:

A struct with no fields, like the unit struct, is therefore the no-op of products, which is 1. There is exactly 1 way to instantiate a struct with no fields, which makes sense.

An enum with no variants is therefore the no-op of sums, which is 0. There are exactly 0 ways to instantiate an enum with no variants.

Unit is a struct with only one way to instantiate it, and Infallible is an enum with NO ways to instantiate it.

Credit: Speykious on discord

---

## [std::convert::Infallible](https://doc.rust-lang.org/std/convert/enum.Infallible.html)

```rust[]
pub enum Infallible {}
```

```rust[]
fn t_1000() -> Infallible {
	loop {
		seek_humans();
		kill_all_humans();
	}
}
```

notes:

You might have seen Infallible used as a return value of a function that can never return.

Useful for infinite loops or branches of your code that can never finish.

---

```rust[]
fn t_1000() -> ! {
	loop {
		seek_humans();
		kill_all_humans();
	}
}
```

```ts
// Current std library
pub enum Infallible {}

// Future plans
pub type Infallible = !;
```

notes:

At time of recording the never type, notated with a bang can be used for this in unstable rust.

The future plans are for these two types to be unified, by the way, with infallible being a type alias for never.

By making a function that returns Infallible, you guarantee that your function will never return. Rust gets this feature for free simply because it has algebraic data types.

Rust being designed so well means that these features simply emerge out of its type system.

Credit: Speykious on discord

---

# Conclusion

## What Does This MEAN IRL?

![[rir.png|200]]

notes:

What does this mean for our actual programming in real life?

'rewrite it in rust' is so prevalent because rust's choices of low-level control and high-level ergonomics are wildly useful to any application.

---

# Rust is a Universal Language

notes:
Building Rust on real mathematics AND real hardware constraints makes the language extremely uniform.

I think this uniformity helps with rust's universality.

This universality means that with one language, you can do what previously you'd have to learn multiple languages to do:

- you dont' have to learn C to write robotics firmware or linux kernel drivers
- Or learn javascript to write interactive web apps
- Or learn the latest hot html preprocessors
- Or the flavour of the week for build tools

Rust can do it all.

And there's so much more left for us to discover.

---

![[tri-hex-moon-white-transparent.png|300]]

# Thank You

## [Patreon.com/NoBoilerplate](http://www.patreon.com/noboilerplate)

notes:

# OUTRO

If you would like to support my channel, get early ad-free and tracking-free videos, vip discord access or 1:1 mentoring, head to patreon.com/noboilerplate.

If you're interested in transhumanism and hopepunk stories, please check out my sci-fi podcast, Lost Terminal.

Or if urban fantasy is more your bag, do listen to a strange and beautiful podcast I produce called Modem Prometheus.

Transcripts and compile-checked markdown sourcecode are available on github, links in the description, and corrections are in the pinned ERRATA comment.

Thank you so much for watching, talk to you on Discord.

```rust
  println!("That's all folks!");
} 
```

---

![[compass-logo-transparent-2.png|500]]
# Rust's Discoveries