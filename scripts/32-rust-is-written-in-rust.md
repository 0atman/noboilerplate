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
# RUST is made of Rust

notes:
%%
- Tell them what you're going to tell them
- Tell them
- Tell them what you told them
%%
Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

As you probably know, the Rust compiler is built in Rust.
Same as the Go compiler is written in Go, and many other languages.

We call this feature self-hosting, when the compiler is written in the language it is compiling.

Rust of course is self-hosting, but unlike other languages, it goes much, much deeper.

---

## Public Domain Videos

[github.com/0atman/noboilerplate/](https://github.com/0atman/noboilerplate/)

notes:
Everything you see in this video from the script to the images are part of a markdown document available on github under a public domain licence.

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
Here are some self-hosted language you may have heard of.

The first version of a language can't be written in the language itself, there's by definition no compiler yet, so it must be bootstrapped in an existing language.

C or assembly are common bootstrapping languages, but with Rust it was OCaml.

---

> At "OCaml Chipotle" you can have whatever you like...But once a month you go for lunch, and they'll refuse to make you a bowl, refuse to tell you why, and refuse to let you leave. And when you try to get help...you realize there's nobody nearby who you can ask.

https://blog.darklang.com/first-thoughts-on-rust-vs-ocaml/

notes:

OCaml by the way is a really excellent ML family language, and the influences on Rust are clear. I talk a lot about Rust being similar to Haskell, but really, they're ALL ML family languages.

Rust is just hiding inside C's clothing to sneak into the popular kids party.

A reminder that there are more Rust projects on github than Kotlin, Scala, and Swift.

We're here because Rust isn't a fringe language, it's mainstream and is ready for use in your company TODAY.

Let's compare Rust to Go.

---

![[go-vs-rust.excalidraw]]

notes:

Go has an excellent garbage collector for automatically managing memory.
But if you want more control, perhaps you need to operate in an environment where a garbage collector is not desirable, such as webassembly, you have no option but to bring all of the go runtime with you, because it is bound up in the compiler.

In this specific case, there is a fork of the compiler, tinygo, that addresses some of these problems with working with go and webassembly.

But this graph isn't actually to scale, let me fix it.

---

![[go-vs-rust-expanded.excalidraw]]

notes:

Rust is both lower-level and higher-level than Go, and Java, and Python, and nearly all other popular language.

This is in large part thanks to the incredible lisp-grade macro system, and the unsafe system, that allows you to keep writing Rust when other languages have to reach for C.

Both these features deserve their own video, and I have made one already, look for my "turtles" video, pinned here.

Let's dig deeper and look at some actual examples of Rust's building blocks.

---

## Options

```rust
pub enum Option<T> {
    None,
    Some(T),
}
```

notes:

Rust's option type is not a primative, there are almost no such things in Rust, it's built in Rust's sum type: Enums.

Sum types are a core piece of algebraic 
- [ ] better name for sum types, algebraic aogorithm?

---

```rust
fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}
fn try_division(dividend: i32, divisor: i32) {
    match checked_division(dividend, divisor) {
        None => println!("{} / {} failed!", dividend, divisor),
        Some(quotient) => {
            println!("{} / {} = {}", dividend, divisor, quotient)
        },
    }jj
}
```

notes:

Rust does not have the concept of Null, instead using options ubiquitously throughout the language.

Nearly all other languages, oldstandards such as javascript, python, and java, and even recently-designed ones like Go and Kotlin, began with nulls, the billion dollar mistake, only to realise their error and attempt a retro-fit of options.

This never works, and you can feel it in the ecosystem and even standard libraries of these languages.

You must not allow nulls in from the start.

---

## std::result::Result

```rust
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

notes:

If you have sum types, rust enums, you can build enormous parts of your standard library with this simple structure.

Here is the result enum, rust's answer to error passing.

---


```rust
let mut file = match File::open(&path) {
    Err(why) => panic!("can't open {path.display()}: {why}"),
    Ok(file) => file,
};
```

or

```rust
let mut file = File::open(&path)?; // to return the Err
```

notes:
And it allows you, without an exception system, to handle errors safer and in a more obvious way than any language with exceptions, which often happen impicitly.

In Rust there are many ways to deal with the result type, should you call a falliable function.
You can match on the error hand handle it comprehensively, or use the idomatic question-mark operator to early-return the result containing the error to the calling function.

It's fantastic, yet entirely optional.

If you want a different error handling system for your library, perhaps some esoteric bare-metal custom hardware, you don't need to use this system, you can write your own, using the same algebraic type system that the standard library authors did.

We'll dig deeper after a word from today's sponsor, Quadratic.

---

<!-- slide bg="rgb(37, 34, 43)" -->

![[quadratic-website.png]]

notes:

Quadratic are building an Open Source spreadsheet for engineers and data scientists, built in Rust, Webassembly and WebGL.


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

---


<!-- slide bg="rgb(37, 34, 43)" -->

![[quadratic-github.png|800]]

https://github.com/quadratichq/quadratic

notes:

it's open source and free to use.
And I'm delighted to say they are hiring Rust developers.

---


<!-- slide bg="rgb(37, 34, 43)" -->
# Hiring for

<split even>

![[rust-logo.png|100]]

![[WebGL_Logo.png]]

![[arrow-inverse.png|100]]

![[scripts/attachments/WASM.png|90]]

</split>

notes:

Quadratic are looking for:
- Rust developers
- People with WebGL experience, even if that's only with JavaScript
- People with Apache Arrow experience for processing Quadratic's high-performance datasets, and,
- Senior engineers used to working at the pace of a startup.


---

<!-- slide bg="rgb(37, 34, 43)" -->

![[Quadratic Logo.png]]

The infinite canvas spreadsheet with code

https://QuadraticHQ.com

notes:

-   Signup today!
-   Head to QuadraticHQ.com to try it out.

My thanks to quadratic for their support of this channel.

Let's dive a little deeper into Rust's Rust.

---


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

https://doc.rust-lang.org/rust-by-example/scope/move.html

notes:

Here is part of the move semantics from the excellent Rust By Example

We create a 32-bit signed integer and allocate it on the heap.
and then move it into the destroy_box function.

In Rust we call this owned heap-allocated value a box.

As soon as the integer falls out of scope and the destroy_box function ends, the compiler inserts code to clean up the memory that was used.

This all happens deterministically at compile time, rather than at runtime, as a GC would do.

---

```rust
fn move_demo() {
    let a = Box::new(5i32);
    drop(a);
}
```

notes:

In production code, we can of course use std::mem::drop to do this, which has the same effect.

In fact, it's deeper than that, it does exactly the same thing.

Let's look at the definition of std::mem::drop:

---

## [std::mem::drop](https://doc.rust-lang.org/std/mem/fn.drop.html)

```rust
pub fn drop<T>(_x: T) {}
```

_THAT'S IT_


notes:
THAT'S IT. This isn't the function signature, this is the function. It has NO BODY.

Drop is a function that takes any type of input parameter and just does nothing with it, meaning that when it finishes, immediately, the compiler cleans up the memory of whatever was passed in.

This function is not magic:

Because `_x` is moved into the function, it is automatically dropped before the function returns.

---



## [std::convert::identity](https://doc.rust-lang.org/std/convert/fn.identity.html)

```rust
pub const fn identity<T>(x: T) -> T { x }
```

notes:

The identity function is almost the same as drop.
It simply takes ownership of the input for a moment, then gives it back.

Identity is a very useful function in functional code, map an iterator with identity and you get back the same iterator.
When functions are first class and can be passed around in lists and applied in sequence, you might need some of these to be "no-ops" depending on some logic. the identity function is that no-op.

---

![[32-rust-is-written-in-rust 2023-08-15 14.57.37.excalidraw]]

notes:

'rewrite it in rust' is so prevalent because in rust the whole stack is in the same language, and often in the same project

You don't have to learn a whole new language to tweak the backend, frontend, or even kernel code.

It's all here, ready for improvement.

Though there are projects to do all these layers in other languages, micropython for instance, or react native, these projects always have the feeling of hammering a square peg into a circular hole.

---

![[react-native-icon-12.jpg]]

notes:

React Native, especilaly is a great example of this.

Having built many production apps in React Native over the last few years of my career in industry, my lasting impression is that it's a horrible ecosystem full of nightmares saved by a single thing:

Web developers prefer to write javascript than java, or swift.

It's extremely unergonomic to do so, as the devices themselves require translation to native code, that's what react native does.

It's very clever, but hammering off the edges of the square peg to fit in this circular hole feels very inelegant to me.

Rust offers a more sane way.


---

---
- farm to table



- - Rust being a universal language
    - dont' have to learn C
    - or javascript
    - or even html
    - or funny build tools

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
