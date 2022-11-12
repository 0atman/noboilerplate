%%
<style>
:root {--r-code-font: "FiraCode Nerd Font";}
</style>

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
use std::collections::BTreeMap;
```

# Setup

```rust
fn main() {
	type Dependency = String;
	println!("Rust is boring");

```

%%

![[rust-logo.png]]

# Rust is Boring

### Why Rust?

notes:

Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

Today I hope to show you that Rust is ready for use in your company, in production, NOW.

Not only that, but if you're not using Rust, you're missing out on a sea-change in how we build software.

---

# Some Stats

- Rust can legally drive a car
- crates.io has ~100k packages
- First-class support on AWS, Azure, GCP
- Most-used wasm language


notes:
First, some stats.

Rust is 16 years old, has more packages than npm did in 2012, has first-class support on all enterprise clouds, and has in-browser support with wasm for 5 years.

---

## Most Loved Languages '22


![[stackoverflow-survey-22-top 5.png]]

86.73%
notes:

It has been the most loved language according to the largest developer survey on the planet for 7 years, that's every single year since 1.0.

And it's not even close.

I don't think Rust is perfect, but I think that Rust can allow you to write perfect code.

While it isn't perfect -

---

<split even>

<ul>
<li>Correctness focus</li>
<li>Algebraic data types</li>
<li>Compile-time execution</li>
<li>Built-in transpiler metaprogramming</li>
<li>Monads (shhh!)</li>
</ul>

<ul>
<li>No runtime overhead</li>
<li>No classes</li>
<li>No runtime</li>
<li>No bytecode</li>
<li>No JIT</li>
<li>No Garbage Collector</li>

</ul>

</split>

notes:

Rust's compromises are EXCELLENT.

The Rust team seems to have chosen the exact features I wish I had in all the other languages I've used over the last 15 years.

Rust has all the features I want, and none of the features I don't.

---

> My biggest compliment to Rust is that it's boring, and this is an amazing compliment.

– Chris Dickinson, npm 

notes:

It's not even that interesting a list.
Rust isn't flashy, it's boring.

---

# Here Be Kraken

notes:

You know what isn't boring? seafaring in the ancient Mediterranean.

If you were a sailor or at merchant, you would understand that you would just lose ships to the sea now and then.

So it was, so it has always been.

Most of the sea was unexplored, and there were monsters, no doubt, taking the ships.

These were the risks that you had to accept to operate on the high seas.

---

# Here Be Overheads

notes:

You know what else isn't boring? SOFTWARE.

And I don't mean that in a good way.

In our field, in 2022, everyone accepts that there are some things you just have to deal with in order to write software.

---

# Lies Programmers Believe About Software

notes:

some lies programmers believe about software, include, but are not limited to:

- That software must be built with unknowable layers of abstraction
- Scaling horizontally is the best way to solve your problems
- Syntax is your enemy.
- Low-level speed and high-level ergonomics are mutually exclusive
- If you don't use yaml and javascript to solve your problems, you will have to use xml and java.

---

# I Am Tired of Accepting Things I Cannot Change

I am now changing the things I cannot accept

notes:

I have been a web developer for 15 years, writing backends in python, ruby and node, and frontends in various javascript frameworks.

I am tired of writing bullshit that I can't guarantee.

and I'm not alone.

---

![[linus-easy-going.png]]

(with apologies to Linus)

notes:

Linus Torvalds is not an easy man to please.

As you probably know, Linus created and oversees the development of the Linux kernel.

He is a detail-oriented man who cares about reliability over all else.

And yet when Rust support was recently merged in to the Linux kernel, many veteran developers decried this step as bowing to a flashy, fashionable new language.

The idea that Linus would approve of rust due to peer pressure is laughable to those who know his reputation.

---

![[network diagram.png]]

notes:

Linus is ABSOLUTELY RIGHT to be so conservative.

Nearly every computer used to show you this video, from your wifi router, to the network systems in your school, company and isp, to google's global video distribution network, is running this kernel.

there are islands of mac devices. But Linux is the OCEAN.

There are clouds of windows networks. But Linux is the SKY.

%% with apologies to Audioslave %%

---

![[linus-chill.png|400]]

(see, Linus is a normal chap)

notes:

We are not talking about your pet projects, or your company's production projects.

We're not even talking about real-time banking software and hospital systems that are shipped when they satisfy the business requirements.

We are talking about Linux.

Linux has to be perfect.

Rust is not the first language to be tried as an alternative to C, to write the Linux kernel.

C++ was proposed 25 years ago, but failed to meet Linus's and the community's extremely high standards.

C++ is not good enough for the Linux kernel, but Rust is.

---

# Rust isn't just for systems programming

notes:

And Rust, despite being a better language to write the Linux kernel in then C++ is also a fantastic web development language, on the backend AND frontend, with superpowers that javascript can only dream of.

---

![[jack-rust-perfect.png]]

<https://twitter.com/jack/status/1474263588651126788>

notes:

This is Jack Dorsey, who built twitter.

---

![[failwhale.png]]

notes:

Twitter started out using a Ruby on Rails monolith, then switched to a scala backend and microservices.

Jack didn't build twitter with C or C++, even though they are fast.
He's a web developer, he used a high level language.

Rust is a also very high level language, and the web development side of the community has delivered.

---

## 🚀 Rocket.rs

![[rocket-rs.png]]

notes:

Rocket, for example, is one of a handful of stable feature-rich backend web development frameworks the Rust community has built. 

If you write your web server in Rocket and your program compiles, many things are guaranteed, before you even test them, such as:
- All get params are handled
- All return values are valid
- No SQL injection
- There are no invalid data pathways in your code
- no memory errors,
- no nullpointerexceptions,
- no nulls anywhere, in rust as a matter of fact,
- Rust has no heavyweight, slow abstractions - code compiles down to for loops and if statements running on bare metal.
- Your app will have a predictable behaviour profile as you scale, unlike java, javascript and go.

This predictable runtime behaviour is something Discord recently adopted Rust to achieve.

---

![[discord-banner.png]]

[noboilerplate.org/discord](https://noboilerplate.org/discord)

notes:

By the way, did you know I run a discord server? Come chat about Rust, programming or my other projects, Lost Terminal & Modem Prometheus too! 

For a few years Discord's backend was mostly go and erlang, a functional concurrent language and system.

---

![[discord-go-graph.png]]

[Source: Why Discord is switching from Go to Rust](https://discord.com/blog/why-discord-is-switching-from-go-to-rust)

notes:

But Go was not fast enough for Discord. Or, it was, but every 2 minutes there would be these latency spikes.

Those who have worked on distributed systems already know what the problem is, because no matter what the language, this kind of spiky graph ALWAYS has the same culprit:

Garbage Collection Pauses.

Once the discord team exhausted all optimisations for Go, they had to switch to a language without a garbage collector, and they chose Rust.

---

![[discord-go-vs-rust.png]]

(Rust is in blue, go in purple)

notes:

- Even with just basic optimisation, Rust was able to outperform their hand-tuned Go version.
- After a bit of profiling and performance optimisation, Rust beat Go on every single performance metric. Latency, CPU, and memory were all better in the Rust version.

But YOU don't have to wait till you experience production issues to adopt a language without the compromises of a Garbage Collector, Rust is ready TODAY.

Lets look at another example:

---


## Cloudflare outgrew NGINX

![[pingora-cloudflare.png]]

<https://blog.cloudflare.com/how-we-built-pingora-the-proxy-that-connects-cloudflare-to-the-internet/>

notes:

Cloudflare recently rewrote their core proxy service because nginx was not fast enough.

I'll say that again: _nginx_ was not fast enough.
The fastest industry-standard web proxy, written in C, was not fast enough.

Pingora, their pure rust replacement, uses 70% less CPU and 67% less RAM, while serving 1 trillion requests a day.

---

> Pingora crashes are so rare we usually find unrelated issues when we do encounter one.
> 
> Recently we discovered [a kernel bug](https://lkml.org/lkml/2022/3/15/6) soon after our service started crashing.

&mdash; Cloudflare

notes:

What cloudflare did is move from a two-component system, nginx scripted with lua, to a single-component system, Pingora written in Rust.

This is an example of Rust's universality. 

You do not need a highly striated system, with slow applications written in high level languages, and fast plumbing written in C. 

Rust can do it all.

---

# What is Rust
# Good For?

notes:

Though Rust can be used in your entire stack, from bare-metal chips through backend web development to in-browser with wasm, there are some tasks that the Rust community can currently help you with more, and some they can help you with less.

You can still do everything, if you're very clever, but in one case you'll be able to rely upon more community code.

And it's here I recommend you start, just as Discord, and Cloudflare did:

With lightning-fast backend web development.

I keep saying 'fast', but what do I mean by that?

---

# How Fast 
# is Rust Code?

notes:

Rust is as fast as C, and in a world where most popular languages are many time (sometimes hundreds of times) slower, this is paradigm-changing.

but How much faster is it, then the other languages we use to write web code today?

Well, it's complicated.

---

## Why I'm No Longer Talking

_(to developers)_

###### <br/>

## About Benchmarks

notes:

Disclaimer: Benchmarks differ wildly, I'm sorry if the numbers in your favourite benchmark don't match mine. The only thing we can hope benchmarks agree on is the approximate order of fastish languages and slowish languages.

---

| Run time (vs C)       | Languages              |
| ------------------- | ---------------------- |
| 2<sup>0</sup> = 1x  | C++, Rust, Zig         |
| 2<sup>1</sup> = 2x  | Go                     |
| 2<sup>2</sup> = 4x  | Java                   |
| 2<sup>3</sup> = 8x  | Javascript             |
| 2<sup>4</sup> = 16x | Ruby                   |
| 2<sup>6</sup> = 64x | Python |
|                     |                        |

(Python is much slower in concurrent benchmarks)

notes:
This is my attempt at generalising benchmarks into categories of similar-speed languages.

The values here are deliberately approximate. 

But you get the idea 

each category has enormous error margins of the order of about 50%, and even then many languages swap around depending on what type of benchmark you do.

But whatever benchmarks you look at, Rust DOMINATES.


---

# Does Speed Matter?

notes:

The elephant in the room is that most of the time, pure language speed doesn't affect your app as much as these numbers suggest, because we're waiting for network, disk, or other IO.

Cloudflare were using lua until recently, for example. 

But a very important area that it DOES matter is in development ergonomics.

---

- Unit tests
- CI pipeline deploys
- Asset building
- Linting
- All tooling

notes:

Language Speed affects development time ENORMOUSLY.

- If your tests run in 1s,
- pipelines deploy instantly,
- Transpilation is all done inside the compiler,
- and the language is rich enough that linting itself is also handled inside the compiler

Then you can iterate faster, both your team, and all external contributors.

---

# Popularity Matters

notes:

You and I would not be here talking about Rust if it was another fringe language that people are excited about, but don't use.

Perhaps I'd be talking about Haskell or Clojure or Julia - three languages I love very much, but I would not build a team around today.

---

![[redmonk-graph-q322.png]]

notes:

There are more Rust projects on github than scala, swift and kotlin.

Another metric is to look at how many third-party packages rust has.
The answer is about 100k, about the same as npm had in 2012.

Rust's package manager is very clever, and worth a detour to talk about.

---

### Crate Definition From crates.io

```rust
struct Crate {
    name: String,
    vers: String,
    deps: Vec<Dependency>,
    cksum: String,
    features: BTreeMap<String, Vec<String>>,
    features2: Option<BTreeMap<String, Vec<String>>>,
    yanked: Option<bool>,
    links: Option<String>,
    v: Option<u32>
}
```

notes:

Rust's package registry is called Crates.io.
And the package tool is call Cargo

When running a package registry you have the problem of what to do with bad package versions that either have unacceptable bugs, or security vulnerabilities.

We want two incompatible things:
- We want packages to be idempotent and perennial, what we build today should build in a decade,
- but we also want to allow package authors to be able to remove bad versions that should not be used.
 
The way cargo solves this is clever.

Package versions can't be deleted, they can only be 'yanked'.
A yanked package can still be used if it is in your project's lock file, so existing projects will not break - but new projects cannot use this version, cargo will display an error and advise an upgrade.

This is an example of Rust's commitment to backwards compatibility, without compromising new features.

---

## No Rust 2.0

> If you almost never add anything, you get C.
> If you almost never remove anything, you get C++.

&mdash;Raffi Molero, yt comment 

notes:

Rust has learned from both C and C++, and has a sensible middle ground, as illustrated by this comment.

Code written today will be guaranteed to compile in all future versions of Rust, while still being able to be used with future code we write in new versions of Rust.

This system is called Rust Editions, as of time of recording, we are using Edition 2021.

This is what is meant when we say there will be no Rust 2.0, as that suggests a breaking change.

Rust will never have breaking changes.

---


%% buttoner %%

# A language for the next 40 years

Install here: https://rustup.rs

notes:


As I have said in previous videos: 

The last 40 years have been written in C, the next 40 years will be written in Rust.

It's not a question of if you will adopt Rust, but when.

Rust is ready now, are you?

---


![[rust-logo.png]]

# Why Rust?


notes:

# OUTTRO

If you'd like to see what you can write in rust, click the top video: I used it to make a fun retro computer visualisation for my scifi podcast, Lost Terminal.

Or if urban fantasy is more your bag, click the bottom video to listen to a strange and beautiful podcast I produce called Modem Prometheus.

If you would like to support my work, head to patreon.com/noboilerplate.

Transcripts and compile-checked markdown sourcecode are available on github, links in the description, and corrections are in the pinned ERRATA comment.

Thank you so much for watching, talk to you on Discord.

```rust
  println!("That's all folks!");
} 
```
