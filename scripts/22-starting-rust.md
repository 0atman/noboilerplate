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

This video is a guide on how to learn Rust, and is the way I teach it in my patreon mentoring sessions.

I have lots of tips of how to get up to speed FAST, and we'll start with the primary sources.

---
## Open Source Videos

[github.com/0atman/noboilerplate/](https://github.com/0atman/noboilerplate/)

notes:
Everything you see in this video from the script to the images are part of a markdown document available on github under a public domain license.

---

# Rust isn't difficult,
# it's unfamiliar

notes:

You may have noticed that **My videos are mostly hype by design.**

Most comments on my videos are by total Rust newbies. Some are even new to programming and are so excited by what they see. If you don't understand Rust, and someone explains it clearly, as I try to, it seems like magic. 

Rust is the [#19th most popular language](https://redmonk.com/sogrady/2022/03/28/language-rankings-1-22/), but the scale is not linear. There is a lot of work to do if it is not to suffer the same fate as Haskell.

**I want Rust to achieve escape velocity.**

---

# My Experience

- Multiple string types
- lifetimes

notes:

Back in 2020 I crashed out of learning Rust twice. First time due to multiple string types (haskell's biggest mistake made again, I thought to myself) and second due to lifetimes. At the time I had a great mentor (Shout-out to Alex!) who picked me up and helped me back on the path. Most people don't have this. They need the excitement that I try to help with in my hype videos to break through.

I will eventually run out of hype topics and move on to slightly more detail. But first, there's a few million developers I want to send to [rustup.rs](http://rustup.rs).

Let's start there.

---

![[rustup-website.png]]
<https://rustup.rs>

notes:

If you're on mac or linux or WSL, run this command in a terminal.

If you're on windows and can't run WSL, then download and run the installer, probably the 64-bit version.

Linux is the native operating system of the internet, and I would recommend that windows users install Microsoft's Windows Subsystem for Linux, WSL, to get access to this incredible world.

Next you need the core triumverate that is required learning for my students. 

---


<split even>

![[rust-book.jpg|200]]

&nbsp;
&nbsp;
&nbsp;
&nbsp;

![[rust-by-example.png|200]]

</split>

![[rustlings-badge.png|460]]

notes:

The three pillars of my recommended sylabus are:

1. The Rust Programming language book, which we just call The Book
2. Rust By Example, excellent supplementary reading, and
3. Rustlings, an interactive code kata system, one of the best I've ever used.

Though there are many other great ways to learn Rust, I recommend starting with these three because they are so tightly bound together, in a way that you might have missed.

02:30

---


| Rustlings Exercise | Book Chapter |
| ------------------ | ------------ |
| variables          | §3.1         |
| functions          | §3.3         |
| if                 | §3.5         |
| primitive_types    | §3.2, §4.3   |
| vecs               | §8.1         |
| move_semantics     | §4.1-2       |
| structs            | §5.1, §5.3   |
| enums              | §6, §18.3    |
| strings            | §8.2         |

notes:

Starting with Rustlings, it's written almost the same order as The Book.
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
It includes tips on idomatic error handling, how to organise your code into modules, and testing strategies.
These are all covered by the book, but Rust By Example is more opinionated on HOW to use these tools.

Keep all three of these tools together.

We'll get back to the main quest in a moment, but I have a side quest for you:

---



![[ultralearning.png|400]]

notes:

This is Ultralearning by Scott H Young.
A very good book on the meta process of learning to learn.

You should read it, or listen to it on audiobook.

---

![[top-men-warehouse-indiana.png]]

(http://science.sciencemag.org/content/331/6818/772)

notes:

In Chapter 8 Young highlights a study where it seemed that students who took the test and failed BEFORE studying did far better than those who only studied and then took the exam.

He calls this procedure "Test to Learn".

It's like placing a number of boxes in your mind, labelled with the information you would LIKE to be there, facts, figures, how lifetime notations work, etc. 
Then when you are studying later, you more readily absorb the information because you are already looking for it.

Keep this in mind when I tell you my next tip:

---

# Read the book twice

notes:

Read the book twice.

I recommend reading the rust book from cover to cover as fast as possible, and without stopping to do the exercises.
If you come across something you don't understand, move on to the next chapter.
The point is to pass the words through the eyeball compiler, and not worry too much about how many errors you get.

---

# Rustlings

```sh
curl -L https://raw.githubusercontent.com
/rust-lang/rustlings/main/install.sh | bash
```

notes:

Rustlings will check that you've got everything set up correctly, git, rust, cargo, it's a great sanity check to avoid pain later.

---


![[rustlings-intro.png]]

---

- Object oriented design isn't easy or intuitive or optimal
	- it's easy to teach, and is taught at every university
	- structs are like tables in a database
	- structs are real

a lot of the literature i've seen on Rust's design systems very heavily emphasizes that the problems people attribute to inheritance are specifically caused by structural inheritance, and thus Rust only supports behavioral inheritance -- which has almost all of the value of structural inheritance and none of the problems.

---

- quizes are katas

---

lifetimes

 [@Mister Ace](https://www.youtube.com/channel/UCFqUeDo637zuKPUT3ShMXWA)  There's two ways to think about Lifetimes: As a liability and an asset.

I started off thinking about them as a liability - something annoying I HAD to do to satisfy the compiler. This is a totally fine way to start off with, and I have some suggestions as a beginner, in order:

1. Don't use references, use ownership and share nothing. Pass in variables into functions, and if you need them later, return them back out.

2. Copy and clone everything. Rust is 80x faster than python, you're probably not going to notice the performance hit!

3. Only use references when the compiler tells you to. This will eventually teach you how to use them.

  

When you've done (3) a few times you're ready to move on:  

  

The intermediate way to think about lifetimes are not as an annoying piece of syntax, but something to enrich the model of your data.


Other languages allow you to model WHAT your data is. Rust, through lifetimes, allows you to model WHEN.

3:30

---

# Razor Secure

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

- If you are a developer and are excited by this challenge and tech, then they are VERY interested in speaking to you as they are hiring NOW.
- The company is fully remote, so wherever you are based they offer challenging work in an interesting field with some awesome technology in a highly collaborative team with flexible working practises.

---

![[rs-logo.png|200]]

[RazorSecure.com/careers](https://www.razorsecure.com/careers)

[razorsecure.noboilerplate.org](http://razorsecure.noboilerplate.org)

notes:

Find out more about jobs at RazorSecure at RazorSecure.com/careers, and if you want to apply, use the link, razorsecure.noboilerplate.org, so they know I sent you.

My thanks to RazorSecure for their support of this channel.

---

![[rust-logo.png]]

# Subtitle

notes:

# OUTRO

If you would like to support my channel, get early ad-free and tracking-free videos and vip discord access head to patreon.com/noboilerplate.

If you're interested in transhumanism and hopepunk stories, please check out my sci-fi podcast, Lost Terminal.

Or if urban fantasy is more your bag, click the bottom video to listen to a strange and beautiful podcast I produce called Modem Prometheus.

Transcripts and compile-checked markdown sourcecode are available on github, links in the description, and corrections are in the pinned ERRATA comment.

Thank you so much for watching, talk to you on Discord.

```rust
  println!("That's all folks!");
} 
```
