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
	println!("Rust userland");

```

%%

![[rust-logo.png]]

# Oxidise Your Life

notes:

%%
- Tell them what you're going to tell them
- Tell them
- Tell them what you told them
%%

Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

---

## Open Source Videos

```sh
$ git clone git@github.com:0atman/noboilerplate.git
```

notes:

As ever, all Rust code you see in this video is part of a literate programming document that can be extracted and compiled with native Rust tooling.

---

# Part 1:

# Shell and Userland

---

#### Part 0: Speed up Compilation

```sh
cargo install sccache
```

- [ ] add a demo benchmark of ripgrep

```sh
RUSTC_WRAPPER=sccache cargo install {package}
```

notes:

Actually before you install all this you should use sccache to dramatically reduce your compile times.

sccache re-uses already-compiled artifects to skip duplica compilation.

When you're installing system tools that very often are built by people that always use the stable version of a dependency, you might re-compile that dependency very often.

With sccache, you wont.

---

## `cargo Install nu`

notes:

Let's start with the shell.

Most shells are stuck in the 80s, bash, zsh certainly are, only giving slight improvements on sh in the name of backwards compatibility.

---

`fish screenshot`

notes:

Fish has the right idea, finally a shell for the 90s, and you can configure it in a web browser, even! But we are Rust developer we are not so timid.

Nu is a shell built around the language of the same name.

---

## Nu's Strucutured Pipelins

```sql
> ps | where cpu > 0
╭───┬───────┬───────────┬───────┬───────────┬───────────╮
│ # │  pid  │   name    │  cpu  │    mem    │  virtual  │
├───┼───────┼───────────┼───────┼───────────┼───────────┤
│ 0 │  2240 │ Slack.exe │ 16.40 │ 178.3 MiB │ 232.6 MiB │
│ 1 │ 16948 │ Slack.exe │ 16.32 │ 205.0 MiB │ 197.9 MiB │
│ 2 │ 17700 │ nu.exe    │  3.77 │  26.1 MiB │   8.8 MiB │
╰───┴───────┴───────────┴───────┴───────────┴───────────╯
```

(not just streams of strings)

notes:

> Nu draws inspiration from projects like PowerShell, functional programming languages, and modern CLI tools. Rather than thinking of files and data as raw streams of text, Nu looks at each input as something with structure.

One of the stealth benefits you get from breaking with traditional tooling is that Nu works everywhere cargo does.

Mac, Linux, ARM computers like raspberry pi and M1s running asahi linux, and even on vanilla windows, in addition to wsl.

The tools I'm going to tell you about compile across all these architectures

Your whole userland can be uniform wherever you work.

The chief reason that this all works nearly everywhere is that Rust developers tend to re-write the C libraries that other language have to rely upon.

The rewrite it in rust meme exists for a reason.

This makes our applications more portable, but comes with an interopability cost.

---

## Rust Doesn't Work with C++

# It Replaces it

notes:

Many people are upset that Rust doesn't play well with C++.

In this video, you'll see why that is: Rust is here to replace it.

Let's crack on.

---

## `cargo install starship`

<br/>

```sh
noboilerplate/scripts on  20-rust-userland via 🦀 v1.69
❯ echo "hi!"
hi!
```

notes:

---

Prompt

AWS

Azure

Battery

Buf

Bun

C

Character

CMake

Duration

Conda

Container

Crystal

Daml

Dart

Deno

Directory

Docker

Dotnet

Elixir

Elm

Env Vars

Erlang

Fill

gcloud

Git Branch

Git Commit

Git State

Git Metrics

Git Status

Go

Guix-shell

Haskell

Haxe

Helm

Hostname

Java

Jobs

Julia

Kotlin

Kubernetes

Line Break

Local IP

Lua

Memory Usage

Meson

Mercurial Branch

Nim

Nix-shell

Node.js

OCaml

Open Policy Agent

OpenStack

OS

Perl

PHP

PureScript

Python

R

Raku

Red

Ruby

**Rust**

Scala

Shell

SHLVL

Singularity

Spack

Status

Sudo

Swift

Terraform

Time

Vagrant

V

VCSH

Zig

Custom commands

_(Tag yourself)_

notes:

Starship has a lot of plugins.

Many enabled out of the box, such as version control and programming package versions, but some require some small config, and others such as the aws plugin, you might want to turn off for being too noisy.

But you can craft the exact prompt you want, with zero latency, as it's built with the fastest high-level language on the planet.

---

## `cargo install exa`

![[exa-demo-mac.png]]

notes:

Exa is a gorgeous replacement for ls.

The command we type all the time.

>  giving it more features and better defaults. It uses colours to distinguish file types and metadata. It knows about symlinks, extended attributes, and Git. it’s **small**, **fast**, and like everything I'm showing you today just **one single binary that you can compile anywhere**.

---

## `cargo install du-dust`

![[dust-dropbox-music.png]]

notes:

du + rust = dust. Like du but more intuitive.

A visual disk usage tool that graphs where your precious hard disk space has gone.

Here are the biggest music and podcast projects in my music directory, for instance.

Colourful, useful, keep it around, you never know when you might need it

---

## `cargo install bat`

![[bat-cargo-toml.png]]

notes:
while we're replacing standard unix tools we might as well keep going with a cat replacement.

Bat allows rich syntax highlighting of files when you don't want to open up a full editor.

if there is more than a screenful of text, bat will hand over to a pager such as less EXCEPT if terminal is not detected, then it does not.

Bat transparently acts like cat when it detects non-interactive use, so is safe to alias to cat.

---

## `cargo install zellij`

![[zellij-compiling-tokio.png]]

notes:
Zellij is everything you have to configure tmux to be.

Like tmux or screen, it is a terminal multiplexer with support for tabs, splits, and rich customisation.

Unlike either, zellij has glorious out-of-the-box defaults.

There is also a plugin system for writing text-based widgets, built-in there is a directory listing plugin and two others.

Zellij is ostentatious, colourful, wasteful of screen-space and I love it.

---

## `cargo install ripgrep`

![[ripgrep-tris.png]]

notes:

Ripgrep replaces find and grep with one tool. 

You might have used ripgrep already, it's the the fastest grepper in town.

This is because itt is built on top of [Rust's regex engine](https://github.com/rust-lang/regex). Rust's regex engine uses finite automata, SIMD and aggressive literal optimizations to make searching very fast.

If you use grep, ag, git grep, ucg, pt, sift
you would do well to upgrade.

---

# Development

---

## Bob-nvim

---

## Gitui

---

## Irust

---

## Bacon

---

## Mprocs

---

## evcxr_jupyter

---

## Cargo-info

---

# Applications

---

## Ncspot

---

## Porsmo

---

## Speedtest-rs

---

## Wiki-tui

---

## Rtx-cli

---

---

![[rust-logo.png]]

# With Rust!

notes:

# OUTTRO

If you would like to support my channel, get early ad-free and tracking-free videos and vip discord access head to patreon.com/noboilerplate.

If you're interested in transhumanism and hopepunk stories, please check out my sci-fi podcast, Lost Terminal.

Or if urban fantasy is more your bag, click the bottom video to listen to a strange and beautiful podcast I produce called Modem Prometheus.

Transcripts and compile-checked markdown sourcecode are available on github, links in the description, and corrections are in the pinned ERRATA comment.

Thank you so much for watching, talk to you on Discord.

```rust
  println!("That's all folks!");
} 
```
