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

# Production RUST with shuttle.rs

notes:
%%
- Tell them what you're going to tell them
- Tell them
- Tell them what you told them
%%
Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

Rust is a very new language, existing CI and production pipelines are ill-suited to it, and don't take advantage of its superpowers.

Rust's type system is so powerful you can encode infrastructure inside it.

Statically typed infrastructure, validated by your local rust compiler is possible, and it's a technique you can use today with shuttle.rs, who are both subject and sponsor of today's video.

---

## Public Domain Videos

[github.com/0atman/noboilerplate/](https://github.com/0atman/noboilerplate/)

notes:
Everything you see in this video from the script to the images are part of a markdown document available on github under a public domain license.

---

![[shuttle-tutorial.png|700]]
https://docs.shuttle.rs/introduction/quick-start

notes:

We're going to follow the shuttle quickstart in the first part of this video, logging in to shuttle using github, generating an api key, and deploying a hello world app.

---

```bash
cargo install cargo-binstall
cargo binstall cargo-shuttle
cargo shuttle login
```

notes:

Assuming you've installed rust from rustup.rs, installing shuttle is easy, as there is a binary install for it.

If you're not familiar with cargo-binstall, it is a wrapper around cargo install that checks github releases and a few other sensible places for pre-built binaries for your version and architecture.

---

# Tutorial part 1

notes:

- 10 minutes of building a simple axum/postgres app and pushing it up to shuttle  

%%
Getting started with Shuttle: [https://docs.shuttle.rs/introduction/quick-start](https://docs.shuttle.rs/introduction/quick-start)  
Local run: [https://docs.shuttle.rs/introduction/local-run](https://docs.shuttle.rs/introduction/local-run)

Axum & Static files example: [https://github.com/shuttle-hq/examples/tree/54e3617a528dc32e5b9a1fe8514fc4f57bd0a4a9/axum/static-files](https://github.com/shuttle-hq/examples/tree/54e3617a528dc32e5b9a1fe8514fc4f57bd0a4a9/axum/static-files)

Shared database docs: [https://docs.shuttle.rs/resources/shuttle-shared-db](https://docs.shuttle.rs/resources/shuttle-shared-db) (Postgres)

Static folder docs: [https://docs.shuttle.rs/resources/shuttle-static-folder](https://docs.shuttle.rs/resources/shuttle-static-folder)  
%%




---

# Mid-roll 30s break

notes:

- A mid-roll breakaway, perhaps 30s, explaining shuttle's offering  
**What Shuttle has right now that's worth mentioning

1.  **create-shuttle-app** - 'create-shuttle-app' is an NPM package that, upon running the 'init' command, generates everything you need to deploy your first "full-stack" Next.js/Axum/Shuttle app (Diagram attached).
    
2.  **Shuttle Next** - [https://docs.shuttle.rs/examples/shuttle-next](https://docs.shuttle.rs/examples/shuttle-next) (experimental at this moment and limited)


---

# GH Stars

notes:

- if viewers would like to keep up-to-date with Shuttle & would like to show support, they can/should give us a star on GitHub.

---

# Discord 

notes:

our community has been thriving lately and we'd love if your viewers hopped over to our Discord to have a good time!

---

# Newsletter

notes:

by Wed, we should already have setup a landing page where users can subscribe but in short; we are doing a education newsletter called Shuttle Launchpad. The main idea behind it is to provide users with bite-sized tutorials/guides/resources on their path to learning Rust Web.


---


# Tutorial part 2

notes:

todo!()


---

# Future features


notes:
(some exclusive plans for the future)


1.  **​Pricing** - we are actively working on a pricing model in the upcoming weeks (aligned with Beta). So the current state of things is that we have a very generous free tier and a per-use-case pricing depending on what the need is (also flexible and generous)
2.  **Events** - we have a couple of events coming up such as a workshop that combines Next.js, Rust & interacting with GPT, a hackathon, etc.

What Shuttle will have soon that's worth mentioning**

1.  **​Console** - we will be releasing a console in the upcoming weeks which will allow users to visualize and manage their projects & resources (see which resources you have as part of your project, add additional resources via IfC, logs both project-based and resource-based, enhanced getting started experience, etc.). I've attached a showcase video below because it performed really well across all different channels where we dropped it (Reddit, HN, etc.)
2.  **Going Beta** - since Shuttle is currently still in Alpha, papercuts & stability issues, among other things, are somewhat expected. We are planning to hit beta soon (this summer) with new features and a step towards being able to support production-ready apps (production use cases)
3.  **Framework (Shuttle Next)** - further developments on the framework mentioned above
4.  **Other (tbd)**
5.  We have a "loose" roadmap for the rest of the year that covers things such as:
6.  Horizontal Scaling
7.  Backward Compatibility
8.  Advanced Workload isolation
9.  AWS Memcached Integration
10.  Multiple Resources per Crate & Multi-service Networking
11.  AWS Lambda Resource & Router Resource
12.  S3 & CloudFront Resources
13.  EBS Storage for Persistent Services
14.  SSR Backend Integration
15.  Event Producers & MSK Consumers
16.  AWS MSK & RDS HA
17.  Shuttle Own-Cloud Integration
18.  (potential) SurrealDB support
19.  (potential) Turso support
20.  Feel free to pick what you consider most appealing to your audience.


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
