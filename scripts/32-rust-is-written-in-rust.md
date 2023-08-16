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

![[Pasted image 20230814151822.png|600]]

# RUST is Rust

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
But if you want more control, perhaps you need to operate in an environment where a garbage collector is not possible, such as webassembly, you have no option but to bring all of the go runtime with you, because it is bound up in the compiler.

In this specific case, there is a fork of the compiler, tinygo, that addresses some of these problems with working with go and webassembly.

But this graph isn't actually to scale, let me fix it.

---

![[go-vs-rust-expanded.excalidraw]]

---


```rust
fn destroy_box(b: Box<i32>) {
    println!("Destroying a box that contains {b}");
    // `c` is destroyed and the memory freed
}

fn move_demo() {
    let a = Box::new(5i32);
    destroy_box(a);
}
```

https://doc.rust-lang.org/rust-by-example/scope/move.html

notes:

Here is part of the move semantics from the excellent Rust By Example


---

## [std::mem::drop](https://doc.rust-lang.org/std/mem/fn.drop.html)

```rust
pub fn drop<T>(_x: T) {}
```

_THAT'S IT_


notes:
This function is not magic:

Because `_x` is moved into the function, it is automatically dropped before the function returns.

---

https://doc.rust-lang.org/std/convert/fn.identity.html

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





<!-- slide bg="rgb(37, 34, 43)" -->

![[quadratic-website.png]]

notes:

Quadratic are building an Open Source spreadsheet for engineers and data scientists, built in Rust, Webassembly and WebGL

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

---

<!-- slide bg="rgb(37, 34, 43)" -->

![[Quadratic Logo.png]]

The infinite canvas spreadsheet with code

https://QuadraticHQ.com

notes:

-   Signup today!
-   Head to QuadraticHQ.com to try it out.

My thanks to quadratic for their support of this channel.

---

4:30

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
