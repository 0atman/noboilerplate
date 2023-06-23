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

# How To Speak Basic Rust

notes:
%%
- Tell them what you're going to tell them
- Tell them
- Tell them what you told them
%%
Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

Welcome class to Rust 101, where you'll learn how to understand what the compiler is saying to you.

This is a critical skill that atrophies in other languages that give poor or no error output.

Open your books, here are my tips, and read the error again.

---

# Public Domain Videos

[github.com/0atman/noboilerplate/](https://github.com/0atman/noboilerplate/)

notes:
Everything you see in this video from the script to the images are part of a markdown document available on github under a public domain license.

---

## A Basic Language User

> Can communicate in simple and routine tasks requiring a simple and direct exchange of information on familiar and routine matters.  

notes:

Being a Basic User of the Rust language is easy, because there has been so much time invested by the incredible compiler team in making your early experience of Rust a wonderful one.

Forget the borrow checker, forget lifetimes and reference counting and all that other stuff you've heard of.

You don't need to understand them yet to build simple rust apps, in the same way as you can get the gender of a french noun wrong and still be understood fine by a native speaker.

Kind, generous people will always understand what you're talking about, and kind generous people wrote the rust compiler.

---

# Cargo clippy

```shell
cargo clippy -- \
-W clippy::pedantic \
-W clippy::nursery \
-W clippy::unwrap_used \
```

notes:

Here is the basic command I recommend beginners use to get the compiler involved with their Rust education.

Clippy is Rust's built-in linter (yes, named after the 90s ms office paperclip!) and as ever, you should let the linter teach you the language.

You can even get the very good crate bacon to auto-run clippy for you, while you code. it's a very nice complacent to your editor's linting!

---

```rust[]
function hello() {
	print("hello")
}
```

```js
function hello() {
-------- ^^^^^ expected one of 8 possible tokens

write `fn` instead of `function` to declare a function
```

_(I will lightly edit error messages to improve readability)_

notes:

you can even start by writing javascript and rust will tell you what to do.

Let's isolate some beginner mistakes and study how the compiler will respond.


---

```rust[]
fn typo() {
	println("hello!");
}
```

```js

println("hello!");
^^^^^^^ not a function

help: use `!` to invoke the macro

println!("hello!");
       +
```

notes:

A very common mistake is to forget the bang at the end of a macro invocation.
Macros, as i explained in detail in my 'witchcraft' video, are functions that execute at compile time, and are a very powerful metaprogramming technique borrowed from lisp.

Because they are so powerful, it's very good to know when they are being used. Rust flags this with the bang, on macro invocation.

As will will see throughout basic Rust, the compiler can almost always correct your errors for you.

---

```rust[]
fn test() {
	furmat!("{}", "hi");
}
```

```md
error: cannot find macro `furmat` in this scope
    --> src/main.rs:8:2

     furmat!("{}", "hi");
     ^^^^^^ a macro with a similar name exists: `format`
```

notes:

If the item is known to the compiler, it can suggest it for you, if you've spelled it wrong.


---

```rust[]
std::io::stdout().flush().unwrap();
```


```js
`std::io::stdout().flush().unwrap();`
                   ^^^^^ method not found in `Stdout`

= help: items from traits can only be used if the trait
is in scope [...] perhaps add a `use` for it

`use std::io::Write;`
```

notes:

It can also figure out if you're using an un-imported method, and suggest importing the trait that may provide that method.

This makes working with the unfamiliar trait system much easier for newbies.

---

# Mentoring Update

A 1h video call on:
- Rust, programming & your projects, 
- Computer science,
- Audio or video production,
- Team or personal productivity,
- You decide!

notes:

As I mentioned about a month ago, I've started 1:1 mentoring on patreon, and this is what has allowed me to quit my day job and focus on No Boilerplate full time!

If you would like to sign up for a monthly recurring 1-hour video meeting, head over to patreon.com/noboilerplate.

We can talk anything valuable, including Rust, video production or your choice of topics.

---

![[nb-patreon-3-tiers.png]]

https://www.patreon.com/noboilerplate

notes:

If you would like to support what I do here, I have multiple patreon options, and they all get early ad-free and tracking-free videos.
Thanks to everyone who has supported me on this wild journey so far!

Back to speaking Rust:

---

```rust[]
let 🦀 = "Ferris!";
```

```js
error: Ferris cannot be used as an identifier
  --> src/main.rs:7:5

let 🦀 = "Ferris!";
    ^^ help: try using their name instead: `ferris`
```

notes:

Here's a fun Easter egg, while rust supports emojis inside strings, they can't be used as identifiers.
But if you use a very special emoji, rust knows what it should be called!

---

```rust[]
10 – 5;
```


```js
 1  error: unknown start of token: \u{2013}
  --> src/main.rs:7:4
   |
 7 | 10 – 5;
   |    ^
   |
 help: Unicode character '–' (En Dash)
 looks like '-' (Minus/Hyphen), but it is not
   |
 7 | 10 - 5;
   |    ~
```

notes:
at first glance this simple line should compile.

But that is not a minus symbol up there, that's an emdash, and the developer has likely copied and pasted from a badly-formatted web page or a microsoft product.

I remember in my first programming class at university, the lecturer BEGGED us not to copy and paste from the pdf handouts they had given us.
Half the class, including me! did anyway, and our code didn't compile, and had a cryptic error message from the c compiler.

Rust understands these homogliphs.

---

```rust[]
let pi = 3.14;
```


```js
error: approximate value of `f{32, 64}::consts::PI` found
  --> src/main.rs:7:11

let pi = 3.14;
         ^^^^

= help: consider using the constant directly
```

notes:
even simple things can be made better by good error messages, like here where the compiler recommends constants be used.

If you're a purist, or vihart, this works with Tau, too.

---

```rust[]
let mut a = 1;
let mut b = 2;
a = b;
b = a;
```


```js
error: this looks like you are trying to swap `a` and `b`
   --> src/main.rs:9:1

 / a = b;
 | b = a;
 |_____^ help: try: `std::mem::swap(&mut a, &mut b)`

= or maybe you should use `std::mem::replace`?
```

notes:

This common variable swapping mistake can so easily slip into our code, especially when nested inside some dense inner block, rather than being easy to read as it is here.

I love this stuff.
You and I, as high-level application developers, can so easily use extremely performant bare-metal operations, giving our high-level applications speed and superpowers, and all without making me scared.

---

```rust[]
let s = [1, 2, 3];
_ = match s {
	[0, 0, 0] => { 0 }
};
```


```js
error[E0004]: non-exhaustive patterns:
  --> src/main.rs:8:11

_ = match s {
          ^ patterns `[i32::MIN..=-1_i32, _, _]` 
           and `[2_i32..=i32::MAX, _, _]` not covered

= the matched value is of type `[i32; 3]`
ensure that all possible cases are being handled by
adding a match arm with a wildcard pattern, a match arm
with multiple or-patterns, or multiple match arms

[2_i32..=i32::MAX, _, _] => todo!()

```

notes:

The more you use the match expression rather than if expression, the safer your code is, because if expressions aren't as type safe, they can do anything, after all!

If you want to do anything, they are there, just as for loops are available instead of iterators.

But most of the time, refactoring into a match expression will allow the compiler to help you enormously, principally by making sure you handle all input cases.

---
<split even>

![[shirt-read-the-error.png|400]]

&nbsp;
&nbsp;
&nbsp;
&nbsp;

![[nb-teepublic-error-obey.png|400]]

</split>

https://www.teepublic.com/user/no-boilerplate

notes:

In all your Rust coding, I encourage you to read the error again.
Read it twice, read it three times, the information is all there, with very few exceptions, if you only look.

I say this so often I put it on a stupid shirt in my stupid store.
Don't buy it whatever you do.

---

```rust[]
let not_true = ~true;
```

```js
error: `~` cannot be used as a unary operator
--> src/main.rs:8:10

let not_true = ~true;
               ^ help: use `!` to perform bitwise not
```

notes:

those of you coming from C might mix up bitwise not, and the compiler knows it.


---

```rust[]
let my_option: Option<()> = None;
if my_option.is_none() {
	my_option.unwrap();
}
```


```js
error: this call to `unwrap()` will always panic
 --> src/main.rs:5:9

     if my_option.is_none() {
       ------------------- because of this check
        my_option.unwrap();
        ^^^^^^^^^^^^^^^^^^

```

notes:

This example is a great case study of the benefits of a strongly typed language that pushes as much safety to compile time as possible.

You and I, as developers, live exclusively at compile time.
We sit in front of our editor and look at the same source code the compiler does.

Rust's compiler reads the same source we do, and because it is marked up with all this type metadata it could make the same decisions we can.

And if it doesn't yet, well there's work to be done, open a PR and pitch in.

Lets continue.

---

```rust[]
struct Node {
	parent: Node
}
```

```js
 1  error[E0072]: recursive type has infinite size
  --> src/main.rs:7:1

 struct Node {
 ^^^^^^^^^^^
     parent: Node
             ---- recursive without indirection

insert some indirection (e.g., a `Box`, `Rc`, or `&`)
to break the cycle

     parent: Box<Node>
             +++++++++
```

notes:

I first got this infinite error when I tried to make a little dungeon-crawler game where rooms were connected to other rooms and so on, in a tree.
Because structs must have a size that is known at compile time, recursive structs could contain themselves and therefore be linked forever, taking up infinite memory.

Fun!

Rust is telling you that you have to weaken the relationship slightly with a pointer, a reference-counting pointer, or a normal borrow, introducing you gently to these more complex smart pointers.

---


```rust[]
let mut users = vec!["Tris", "Lucy"];
let (tx, _) = std::sync::mpsc::channel();
tx.send(users);
users.push("Henrietta");
```

https://doc.rust-lang.org/std/sync/mpsc/

notes:

and finally, our most complex example for today, one of the core pillars of rust is 'fearless concurrency', which is due to the compiler keeping you safe right through complex parallel processing.

Here we have a multiple producer single consumer channel, MPSC, the most commonly used channel in rust and the one included in the standard library.

This channel is built in pure rust, by the way, it's not a feature provided by some underlying runtime, rust has almost no runtime, so you can peak at the channel's source code and write your own, if you would like, and many people have, check crates.io for them.

---

```rust[4]
let mut users = vec!["Tris", "Lucy"];
let (tx, _) = std::sync::mpsc::channel();
tx.send(users);
users.push("Henrietta");
```

```js
error[E0382]: borrow of moved value: `users`
   --> src/main.rs:12:1

let mut users = vec!["Tris", "Lucy"];
	----- move occurs because `users` has type Vec<&str>
          which does not implement the `Copy` trait

tx.send(users);
        ----- value moved here

users.push("Henrietta");
^^^^^^^^^^^^^^^^^^^^^^^^ value borrowed here after move
```

notes:


In this example we're sending a list of users down a channel and then trying to add another user onto that list after it's sent.

In other languages, such as Go, the simple type system can't keep you safe when using channels, you have to remember what the rules are for parallel processing.
One of these rules is, after you've sent a variable into a channel or thread, you can't safely reuse it - it could be modified or overwritten or used in unpredictable ways by unpredictable threads.

The rust compiler doesn't need to treat threaded code as a special case to maintain safety, the borrow checker's simple rules keep us safe, with the same rich errors we've seen all along.

---

# Rust is unfamiliar
## but can be understood 
## through thorough 
### thought though 

notes:

Read the erorr again, thank you!

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
