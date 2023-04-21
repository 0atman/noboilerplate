<style>
:root {--r-code-font: "FiraCode Nerd Font";}
.reveal .hljs {min-height: 50%;}
</style>
%%

f7f7f7 background slide colour

# Cargo.toml

```toml
[package]
name = "StartingRust"
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

# How to learn Rust

notes:
%%
- Tell them what you're going to tell them
- Tell them
- Tell them what you told them
%%
Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

This is my guide on how to learn Rust, and is the way I teach it to my patreons in our mentoring sessions.

I have lots of tips on how to get up to speed FAST, and we'll start with the primary sources.

---
## Public Domain Videos

[github.com/0atman/noboilerplate/](https://github.com/0atman/noboilerplate/)

notes:
Everything you see in this video from the script to the images are part of a markdown document available on github under a public domain license.

---

# Rust isn't difficult,
# it's unfamiliar

notes:

Rust isn't difficult, it's unfamiliar.
This is the first lesson.

You may have noticed that **My videos are mostly hype by design.**

Most comments on my videos are by total Rust newbies. Some are even new to programming and yet are so excited by what they see. If you don't yet understand Rust, and someone explains it clearly, as I try to, it seems like magic. 

---

![[redmonk-graph-q322.png|700]]

notes:

Rust is the [#19th most popular language](https://redmonk.com/sogrady/2022/03/28/language-rankings-1-22/), according to github projects and stackoverflow tags, but the scale is not linear. There is a lot of work to do if it is not to suffer the same fate as Haskell and common lisp.

**I want Rust to achieve escape velocity.**

---

# My Experience

- Multiple string types
- Lifetimes

notes:

Back in 2020 I crashed out of learning Rust twice. First time due to multiple string types (haskell's biggest mistake made again, I thought to myself) and the second due to lifetimes. At the time I had a great mentor (Shout-out to Alex!) who picked me up and helped me back on the path. Most people don't have this luxury. They need the excitement that I try to help with in my hype videos to break through.

I will eventually run out of hype topics and move on to slightly more detail. But first, there's a few million developers I want to send to [rustup.rs](http://rustup.rs).

Let's start there.

---

![[rustup-website.png|500]]
<https://rustup.rs>

notes:

If you're on mac or linux or WSL, run this command in a terminal.

If you're on windows and can't run WSL, then download and run the installer, probably the 64-bit version.

Linux is the native operating system of the internet, and I would recommend that windows users install Microsoft's Windows Subsystem for Linux, WSL, to get access to this incredible world.

Next you need the core triumvirate that is required learning for my students. 

---


<split even>

![[rust-book.jpg|200]]

&nbsp;
&nbsp;
&nbsp;
&nbsp;

![[rust-by-example-web.png|200]]

</split>

![[rustlings-badge.png|460]]


notes:

The three pillars of my recommended syllabus are:

1. [The Rust Programming language book](https://doc.rust-lang.org/stable/book/), which from now on we just call The Book
2. [Rust By Example](https://doc.rust-lang.org/stable/rust-by-example/), excellent supplementary reading, and
3. [Rustlings](https://github.com/rust-lang/rustlings), an interactive code kata system, one of the best I've ever used.

Though there are many other great ways to learn Rust, I recommend starting with these three because they are so tightly bound together, in a way that you might have missed.


---


| Rustlings Exercise | Book Chapter |
| ------------------ | ------------ |
| 1. variables       | §3.1         |
| 2. functions       | §3.3         |
| 3. if              | §3.5         |
| 4. primitive_types | §3.2, §4.3   |
| 5. vecs            | §8.1         |
| 6. move_semantics  | §4.1-2       |
| 7. structs         | §5.1, §5.3   |
| 8. enums           | §6, §18.3    |

notes:

Starting with Rustlings, it's written in almost the same order as The Book.
This is by design, they are made to be consumed together.

---

## Rust by Example

1. Hello World
2. Primitives
3. Custom Types
4. Variable Bindings
5. Types
6. Conversion
7. Expressions
8. Flow of Control
9. Functions
10. Modules

notes:

Rust By Example is less linearly linked to The Book.
Though it's partially in the same order, I read it as a supplement to The Book.
It includes high-level tips on idiomatic error handling, how to organise your code into modules, and testing strategies.
These are all covered in The Book, but Rust By Example is more opinionated on HOW to use them.

Keep all three of these tools together.

We'll get back to the main quest in a moment, but first, a quiz, from today's sponsor, Razor Secure:


---

<!-- slide bg="[[rs-train.jpg]]" -->

![[rs-logo.png|400]]
notes:
_(disclosure: The company's CTO is my brother!)_

What data-centre:
- travels at 100 miles-per-hour
- re-configures every IP when it attaches or detaches to other data-centres
- all without a reliable power source or internet connection, and must never fail?

The Answer: A train.

---

<!-- slide bg="[[rs-train.jpg]]" -->

![[rs-logo.png|200]]

1. Cross-platform Rust Agent
2. Cloud microservices
3. Embedded hardware platform

notes:

- RazorSecure is a 50-person startup bringing cutting-edge security tech to the rapidly-advancing world of rail
- They do this through:
   - A Rust intrusion detection and monitoring agent running on-board.
   - A cloud environment running K8s, Python micro-services, and event-based data processing, and
   - A Yocto hardware platform running custom embedded linux.
- Their team and customers span Europe and North America, so if you have taken a train journey here, then RazorSecure's security systems may have already kept you safe.

---

<!-- slide bg="[[rs-train2.jpg]]" -->
<split even>

&nbsp;
&nbsp;
![[Python_logo_icon.png|200]]
&nbsp;
&nbsp;
![[kubernetes-logo.png|200]]

&nbsp;
&nbsp;
![[rust-logo.png|200]]

</split>

notes:

- If you are excited by this challenge and tech, then they are VERY interested in speaking to you as they are hiring NOW.
- The company is fully remote, so wherever you are based they offer challenging work in an interesting field with some awesome technology in a highly collaborative team with flexible working practises.

---

![[rs-logo.png|200]]

[RazorSecure.com/careers](https://www.razorsecure.com/careers)

notes:

Find out more about jobs at RazorSecure at RazorSecure.com/careers.

My thanks to RazorSecure for their support of this channel.

Back to the side quest:

---

### SIDE QUEST #1

![[ultralearning.png|300]]

notes:

This is Ultralearning by Scott H Young.
(not sponsored)
A very good book on the meta process of learning to learn.

You should read it, or listen to it on audiobook as I did.

---

![[top-men-warehouse-indiana.png]]

(http://science.sciencemag.org/content/331/6818/772)

notes:

In Chapter 8 Young highlights a study where it seemed that students who took the exam and failed BEFORE studying did far better than those who only studied and then took the exam.

He calls this procedure "Test to Learn", and it makes a lot of sense.

It's like placing a number of boxes in your mind, labelled with the information you would LIKE to be there, facts, figures, how lifetime annotations work, etc. 
Then when you are studying later, you more readily absorb the information because you are already looking for it.

Keep this in mind when I tell you my next tip:

---

### Side quest #2

# Read the book twice

(second time: https://rust-book.cs.brown.edu)

notes:

I recommend reading the rust book from cover to cover as fast as possible, and without stopping to do the exercises.
If you come across something you don't understand, mentally note it, and move on to the next chapter.
The point is to pass the words through you eyeball compiler, and not worry too much about how many errors you get.

Then when you have finished, go back to the start and work through at your normal pace, but read the Brown University version with interactive quizzes.
- [x] re-record this take and use the new slide

You can now install Rustlings.

---

## [rust-lang/Rustlings](https://github.com/rust-lang/rustlings)

```sh
curl -L https://raw.githubusercontent.com
/rust-lang/rustlings/main/install.sh | bash
```

notes:

Rustlings will check that you've got everything set up correctly, git, rust, cargo, the installer is a great sanity check to avoid pain later.


---


![[rustlings-intro.png]]

notes:

rustling is a code kata style system that you can think of as a suite of failing unit tests that you slowly work through, fixing in a test driven development way. 

---

![[rustlings-failing.png]]

notes:

Here is the first exercise, fixing a hello world.

Rustlings starts out extremely simply, and ramps up slowly.
Even non-programmers could start this way, and veterans will speed through the early questions quickly.

Though starting simple, they follow the book all the way through lifetimes, smart pointers, and threads.

Note that the rustlings watcher, the output of which we are seeing here, has some extra functionality, not just auto-refreshing output from the compiler.

Each exercise has a hint, accessed by simply typing hint into the watcher, as I did here.

---

![[kata.png|700]]

notes:

Rustlings exercises are katas.

Katas, a term borrowed from Japanese martial arts, are something you practice over and over again, to build muscle memory.
Musicians do the same thing.

At time of writing there are 95 Rustlings katas, when you have done them all, it is time to choose your favourites to re-do regularly.
I recommend weekly.

Keep your muscle memory fresh.

---


### SIDE QUEST #3

# HASKELL

```haskell
divide :: Double -> Double -> Maybe Double
divide x 0 = Nothing
divide x y = Just (x / y)
```

notes:
- [x] re-take this screenshot
Rust is a hybrid imperative and functional language.
The compiler was originally written in OCaml
This means, in addition to the punched-card analogue of statements we can also use many features of functional languages, the most obvious of these is the pervasive use of the iterator pattern.

It is possible to write Rust mostly in the way you have been writing Python, or Javascript or Java. But you'll get most out of it if you adopt the functional principles of higher-order functions and immutability.

Rust is flexible, and doesn't force you to write in a functional style. Haskell, however is inflexible in the extreme, and you have no choice but to learn it, and learn it fast.

---

<split even>

![[lyah-haskell.png|300]]

&nbsp;
&nbsp;
&nbsp;
&nbsp;
&nbsp;
&nbsp;
&nbsp;
&nbsp;

![[real-world-haskell.jpg|300]]

</split>

notes:

Have a look at either Learn You A Haskell or Real World Haskell or both.
Again, you can mostly read through these quickly. Your package manger will have the haskell repl, which is called GHCI, and any version it installs will be fine for experimentation.

Learning Haskell will teach you about mapping, filtering, folding, currying, matching, and many other words to describe your algorithms in rust, which are abandoned in other languages that just use for loops and if statements.

---

_(One of Rust's smartest decisions is_

_to USE monads but not CALL them monads)_

notes:

You might even learn what a monad is.

---

# How to think about Rust syntax


notes:

A huge turn-off for me, coming from Python, was Rust's syntax.
There's SO MUCH MORE than I was used to!

in other languages the compiler can't help you very much, so moving fast and breaking things is the best way develop.
(any syntax gets in the way)

In rust the compiler is your best friend.
But you must be nice to it for it to be nice to you.


---


![[22-starting-rust 2023-03-20 20.38.27.excalidraw]]
notes:

Here's how I think the difficulty in learning, say Javascript and Rust compares to each other.
I've picked Javascript because I want to make as many friends as possible, but the general relationship is the same for Python and Java, and many others too

Rust up-fronts all the work that you would need to do in other languages later on to get scalability and performance and safety.

And in making you deal with them at the start, the language debt doesn't spiral as you increase complexity.

---

> In other languages simple things are easy and complex things are possible, in Rust simple things are possible and complex things are EASY.

---

![[22-starting-rust 2023-03-20 20.49.45.excalidraw]]

notes:
And this is as it should be.
I don't care how easy it is to do simple things, that doesn't impress me any more.
I want a language that can stay by my side as I make extremely complex distributed systems that run at C speed and don't wake me up at night.

Keep in mind through all your early learning, YOU only have to learn Rust once.
Javascript developers have to keep learning javascript every single day.

Edge case after edge case after edge case.
Framework after framework to try and manage these edge cases.

It's exhausting.
Rust offers another way. But you have to work for it.

For example

---

# Lifetime annotations

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

notes:

Lifetime annotations specify how long a reference should live.
It's a MINIMUM value that the variable must live for, before the borrow checker knows it is safe to clean up.

This example is from Chapter 10 of The Book, which states:

> the returned reference will be valid as long as both the parameters are valid. 
> We'll name the lifetime `'a` and then add it to each reference

It's very unfamiliar stuff, right?

There's two ways to think about Lifetimes 
1. As a liability and 
2. an asset.

I started off thinking about them as a liability - something annoying I HAD to do to satisfy the compiler. This is a totally fine way to start off with, and I have some suggestions on how to learn lifetimes gradually.

In order

---

## 1. Don't use references
## 2. Copy & Clone everything
## 3. Obey the compiler

notes:

1. Don't use references, use ownership and share nothing. Pass in variables into functions, and if you need them later, return them back out, giving away ownership.

2. Copy and clone everything. Rust is 80x faster than python, you're probably not going to notice the performance hit!

3. Only use references when the compiler tells you to. This will eventually teach you how to use them properly.

When you've done (3) a few times you're ready to move on  

The intermediate way to think about lifetimes are not as an annoying piece of syntax, but something to enrich the model of your data.

Other languages allow you to model WHAT your data is. Rust, through lifetimes, allows you to model WHEN.

That's worth learning!

---

# [rustup.rs](https://rustup.rs)

## The Book
## Rust By Example
## Rustlings

notes:

Rust is shaping up to be the only language we may need for systems development, frontend and backend web development, and bare metal too.

Please ask questions in the comments and I'll try to help, and do join my Discord.

Have fun! Talk to you next time.

---

![[rust-logo.png]]

# Maximum effort
# Maximum reward

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
