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

When running a package registry you have the problem 


---

# Some Stats

- Rust can legally drive a car
- crates.io has ~100k packages
- First-class support on AWS, Azure, GCP
- Most-used wasm language
- Most-loved language since 1.0 (7 years)

(stackoverflow developer survey)

notes:

Rust is 16 years old now, has more packages than npm did in 2012, has first-class support on all enterprise clouds, and in-browser with wasm for 5 years.

It has been the most loved language according to the largest developer survey on the planet for 7 years, that's every single year since 1.0.

---

![[jack-rust-perfect.png]]

<https://twitter.com/jack/status/1474263588651126788>

notes:

Why does everyone love Rust?

This is Jack Dorsey, who built twitter.

---

![[failwhale.png]]

notes:

Twitter started out using Ruby on Rails, then switched to a scala backend and microservices.

Jack didn't build twitter with C or C++. He's a web developer.

---

> If you almost never add anything, you get C.
> If you almost never remove anything, you get C++.

&mdash;Raffi Molero

notes:

I don't quite agree that rust is a PERFECT language, there are many features that I want in Rust, and every 6 weeks I get them, thanks to the incredible core team.

I don't think Rust is perfect, I think that Rust can allow you to write perfect code.

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

The language has all the features I want, and none of the ones I don't.

Rust can basically do EVERYTHING.


---

> My biggest compliment to Rust is that it's boring, and this is an amazing compliment.

– Chris Dickinson, on internal npm Rust adoption

notes:

It's not even that interesting.
Rust isn't trendy, it's boring.

---

# Here Be Kraken

notes:

You know what isn't boring? seafaring in the ancient Mediterranean.

If you were a sailor or at merchant, you would understand that you would lose ships to the sea now and then.

So it was, so it had always been.

Most of the sea was unexplored, and there were monsters, no doubt, taking the ships.

Sailors accepted that these were just the risks that you had to accept to operate on the high seas.

---

# Here Be Overheads

notes:

You know what else isn't boring? SOFTWARE.

And I don't mean in a good way.

In our field in 2022, everyone accepts that there are some things you just have to deal with in order to write software.

---

# Lies Programmers Believe About Software

notes:

some lies programmers believe about software, include, but are not limited to:

- That software is built on unknowable layers of abstraction
- The layers can only do what you want if you recite the correct inscrutable incantations
- Scaling horizontally is the way to solve your problems
- Bugs must be caught in production.
- If you don't use yaml and javascript to solve your problems, you will have to use xml and java.
- speed

---

## I Am Tired of Accepting Things I Cannot Change

I am now changing things I cannot accept

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

The idea that Linus would approve of rust due to POPULARITY is laughable to those who know his reputation.

---

![[network diagram.png]]

notes:

And Linus is ABSOLUTELY RIGHT to be so conservative.

Nearly every computer used show you this video, from your wifi router, to the network systems in your school, company and isp, to google's global video distribution network, is running this kernel.

There are islands of mac devices. But Linux is the OCEAN.

There are clouds of windows networks. But Linux is the SKY.

%% with apologies to Audioslave %%

---

![[linus-chill.png]]
notes:

We are not talking about your pet projects, or your companies production projects.

We're not even talking about real-time banking software and hospital systems.

We are talking about Linux.

Rust is not the first language to be tried as an alternative to C, to write the linux kernel.

C++ was proposed 25 years ago, but failed to meet Linus's extremely high standards

C++ is not good enough for the Linux kernel, but Rust is.

---

And Rust, despite being a better language to write the Linux kernel then C++ is also a fantastic web development language.

---

## 🚀 Rocket.rs

![[rocket-rs.png]]

notes:

Rocket is one of a handful of stable feature-rich backend web development frameworks rocket has

If you write your web server in Rocket, you can be sure of many things:

- The model of your data is correct,
- and there are no invalid data pathways in your code, no memory errors, no nullpointerexceptions, and a predictable behaviour profile, unlike java, javascript and go.

---

![[discord-banner.png]]

[noboilerplate.org/discord](https://noboilerplate.org/discord)

notes:

For a few Discord's backend was mostly go and erlang, a functional concurrent language and system.

---

![[discord-go-graph.png]]

[Source: Why Discord is switching from Go to Rust](https://discord.com/blog/why-discord-is-switching-from-go-to-rust)

notes:

Go was not fast enough for Discord. Or, it was, but every 2 minutes there would be latency spikes.

Those who have worked on distributed systems already know what the problem is, because no matter what the language, this kind of spikey graph ALWAYS has the same culprit:

Garbage Collection Pauses.

Once the team exhausted any optimisations, they had to switch to a language without a garbage collector, they chose Rust.

---

![[discord-go-vs-rust.png]]

(Rust is in blue, go in purple)

notes:

- Even with just basic optimisation, Rust was able to outperform the hand-tuned Go version.
- After a bit of profiling and performance optimisation, Rust beat Go on every single performance metric. Latency, CPU, and memory were all better in the Rust version.

---

Cloudflare outgrew NGINX

<https://blog.cloudflare.com/how-we-built-pingora-the-proxy-that-connects-cloudflare-to-the-internet/>

> Pingora crashes are so rare we usually find unrelated issues when we do encounter one. Recently we discovered [a kernel bug](https://lkml.org/lkml/2022/3/15/6) soon after our service started crashing.

---

# What is Rust Good For?

notes:

Though Rust can be used in your entire stack, from bare-metal chips through backend web development to in-browser with wasm, there are some tasks that the Rust community can help you with more, and some they can help you with less.

You can still do both, but in one case you'll be able to rely upon more community code.

---

# How Fast is Rust Code?

notes:

Rust is as fast as C, and in a world where most popular languages are many time (sometimes hundreds of times) slower, this is paradigm changing.

How much faster is it, then the other languages we use to write web code today?

Well, it's complicated.

---

## Why I'm No Longer Talking

_(to developers)_

<br/>

## About Benchmarks

notes:

Disclaimer: Benchmarks differ wildly, I'm sorry if the numbers in your favourite benchmark don't match mine. The only thing we can hope benchmarks can agree on is the approximate order of fastish languages and slowish languages.

---

| Relative to C       | Languages              |
| ------------------- | ---------------------- |
| 2<sup>0</sup> = 1x  | C++, Rust, Zig         |
| 2<sup>1</sup> = 2x  | Go                     |
| 2<sup>2</sup> = 4x  | Java                   |
| 2<sup>3</sup> = 8x  | Javascript             |
| 2<sup>4</sup> = 16x | Ruby                   |
| 2<sup>6</sup> = 64x | Python (single-thread) |
|                     |                        |

notes:

The values here are deliberately approximate.

each category has enormous error margins of the order of the previous category, and even then many languages swap around depending on what type of benchmark you do.

%% 6 minutes %%

---

# Does Speed Matter?

notes:

The elephant in the room is that most of the time, pure language speed doesn't matter, because we're waiting for network, disk, or other IO.

---

- Unit tests
- CI pipeline deploys
- Asset building
- Linting
- All tooling

notes:

Language Speed affects development time.

- If your tests run in 1s,
- pipelines deploy instantly,
- Transpilation is all done inside the compiler,
- and the language itself is rich enough that linting itself is handled inside the compiler

You can iterate faster, both your team, and all external contributors.

---

---

![[rust-logo.png]]

# Why Rust?

# Why Now?

notes:

# OUTTRO

If you'd like to see what you can write in rust, click the top video: I used it to make a fun retro computer visualisation for my hopepunk podcast, Lost Terminal.

Or if urban fantasy is more your bag, click the bottom video to listen to a strange and beautiful podcast I produce called Modem Prometheus.

If you would like to support my work, head to patreon.com/noboilerplate.

Transcripts and compile-checked markdown sourcecode are available on github, links in the description, and corrections are in the pinned ERRATA comment.

Thank you so much for watching, talk to you on Discord.

```rust
  println!("That's all folks!");
} 
```
