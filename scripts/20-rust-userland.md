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

# Oxidise Your Life with Rust

notes:

%%
- Tell them what you're going to tell them
- Tell them
- Tell them what you told them
%%

Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

Today I'm going to show you how to oxidise your entire toolkit, from editors down to the shell.
Your entire userspace could be written in rust, and be a single cargo install line away.

I've been thinking about writing this video for some time, but the final push was when I read an article about the uutils project, which I will tell you about in a moment.

---

## Open Source Videos

```sh
$ git clone git@github.com:0atman/noboilerplate.git
```

notes:

As ever, everything you see in this video is part of a literate programming document that can be extracted and compiled with native Rust tooling.

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

sccache re-uses already-compiled artifices to skip redundant compilation.

When you're installing system tools that very often are built by people that always use the stable version of a dependency, you might unnecessarily re-compile that dependency very often.

With sccache, you wont.

---

## `cargo install nu`

`screens`

notes:

Let's start with the shell.

Most shells are stuck in the 80s, bash and zsh certainly are, only giving slight improvements on sh in the name of backwards compatibility.

---

`fish screenshot`

notes:

Fish has the right idea, finally a shell for the 90s, and you can configure it in a web browser, even! But we are Rust developer we are not so timid.

Nu is a shell built around the language of the same name.

---

## Nu's Strucutured Pipelins

```sql
$ ps | where cpu > 0
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

Mac, Linux, ARM computers like raspberry pi and (my setup) M1s running asahi linux, even on vanilla windows, in addition to wsl.

The tools I'm going to tell you today about compile across all these architectures

---

## `cargo install coreutils`

```sh
$ coreutils pwd
/home/oatman/projects/noboilerplate/scripts

$ coreutils cat readme.md | coreutils head -n1 
These are the scripts for my fast technical videos.

$ alias cat="coreutils cat"    # <-- aliasing is detected
$ alias head="coreutils head"  # and works transparently

$ cat readme.md | head -n1 
These are the scripts for my fast technical videos.
```

notes:

In fact, your whole userland could be uniform wherever you work, as the uutils project shows.

> uutils aims to work on as many platforms as possible, to be able to use the same utils on Linux, Mac, Windows and other platforms. This ensures, for example, that scripts can be easily transferred between platforms. Rust was chosen not only because it is fast and safe, but is also excellent for writing cross-platform code.

If anyone has used cygwin, it reminds me a bit of that project.

The chief reason that everything you see today works nearly everywhere is that Rust developers tend to re-write the platform-dependant C libraries that other language have to rely upon.

The rewrite it in rust meme exists for a reason.

This makes our applications more portable, coreutils has no dependencies other than rust, for example, but comes with an interoperability cost.

---

# Rust
# Will Replace
# C++

_(for new projects)_

notes:

Many people are upset that Rust doesn't play well with C++.
Not me.
I don't care about C++ compatibility at all, and every month it gets less and less relevant.

For legacy, sure keep using C++, but for new projects? We'll see.

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

Prompt toolkits are often written in their native shell and are SLOOOW.
Starship is as fast as it is possible to be and that gives us superpowers without slowing down our shell!

It runs as well on my 10 year old thinkpad as on this m1 linux machine.

---

Prompt AWS Azure Battery Buf Bun C Character CMake Duration Conda Container Crystal Daml Dart Deno Directory Docker Dotnet Elixir Elm Env Vars Erlang Fill gcloud Git Haskell Haxe Helm Hostname Java Jobs Julia Kotlin Kubernetes Line Break Local IP Lua Memory Usage Meson Mercurial Branch Nim Nix-shell Node.js OCaml Open Policy Agent OpenStack OS Perl PHP PureScript Python R Raku Red Ruby 🦀**Rust** Scala Shell SHLVL Singularity Spack Status Sudo Swift Terraform Time Vagrant V VCSH Zig Custom commands

_(Tag yourself)_

notes:

Starship has a lot of plugins.

Many enabled out of the box, such as version control and programming package versions, but some require some small config, and others such as the aws plugin, you might want to turn off for being too noisy.

But you can craft the exact prompt you want, with low latency, as it's built with the fastest high-level language on the planet.

---

## `cargo install exa`

![[exa-demo-mac.png]]

notes:

Exa is a gorgeous replacement for ls, the command we type all the time,

giving it more features and better defaults. It uses colours to distinguish file types and metadata. It knows about symlinks, extended attributes, and Git. it’s **small**, **fast**, and like everything I'm showing you today just **one single binary that you can compile anywhere**.

---

## `cargo install du-dust`

![[dust-dropbox-music.png]]

notes:

du + rust = dust. Like du but more intuitive.

A visual disk usage tool that graphs where your precious hard disk space has gone.

Here are the biggest music and podcast projects in my music directory, for instance.

Colourful, useful, keep it around, you never know when you might need it

---

## LostTerminal.com
## &
## ModemPrometheus.com

notes:
go on drop an ad here

---

## `cargo install bat`

![[bat-cargo-toml.png]]

notes:
while we're replacing standard unix tools we might as well keep going with a cat replacement.

Bat, the cat with wings, allows rich syntax highlighting of files when you don't want to open up a full editor.

if there is more than a screenful of text, bat will hand over to a pager such as less EXCEPT if a terminal is not detected, then it does not.

Bat transparently acts like cat when it detects non-interactive use, so is safe to alias to cat.

---

## `cargo install zellij`

![[zellij-compiling-tokio.png]]

notes:
%% pron: zell ee djz %%
Zellij is everything you have to configure tmux to be.

Like tmux or screen, it is a terminal multiplexer with support for tabs, splits, and rich customisation.

Unlike either, zellij has glorious out-of-the-box defaults.

There is also a webassembly plugin system for writing text-based widgets, built-in there is a directory listing plugin amongst two others.

Zellij is ostentatious, colourful, wasteful of screen-space and I love it.

---

## `cargo install mprocs`

![[mprocs-bacon.png]]
notes:

mprocs is similar to zellij, tmux, or screen, but it's optimised for long-running single processes, like dev servers, databases, or streaming service logs.

With a config or just by listing the commands on the command line, mprocs runs each of them in its own vertical tab, with a simple colour-coded UP or DOWN showing if the command is still running or not.

Vim bindings are used to switch between commands and they can be restarted by typing r, started and stopped with s and x, and new commands can be added interactively with a.

---

## `cargo install ripgrep`

![[ripgrep-tris.png]]

notes:

Ripgrep replaces find and grep with one unified tool. 

You might have used ripgrep already, it's the the fastest grepper in town and already quite popular.

This is because it is built on top of [Rust's regex engine](https://github.com/rust-lang/regex), which uses finite automata, SIMD and aggressive literal optimisations to make searching very fast.

If you use grep, ag, git grep, ucg, pt, or sift
you would do well to upgrade.

%% 04:36 %%

---

# Development

notes:
Now we've got a foundation and can use our system ergonomically, let's get some awesome development tools.

---

## `cargo install bob-nvim`

```sh
〉bob install 0.8.2
Downloaded to ~.local/share/bob/v0.8.2.tar.gz
  [00:00:00] [██████████████████████████████████] 9.98MiB
Finished expanding to ~.local/share/bob/v0.8.2
  [00:00:00] [██████████████████████████████████] 
Feb 22 16:05:15.512  INFO v0.8.2 has been successfully
installed in /home/oatman/.local/share/bob
```

notes:

> Bob is a cross-platform and easy-to-use Neovim version manager, allowing for easy switching between versions right from the command line.

I found it after discovering that the ubuntu repos did not have neovim verion 0.8, which is the minimum version that my preferred distribution, astronvim, supports.

Though the neovim team build comprehsnive packages and installers for every system, I try not to do in a web browser what I can do on the command line.

When I read that bob is cargo installable, my life was complete.

Or it will be once my ARM64 patch has been accepted for M1 linux.

---

## `cargo install gitui`

![[gitui-demo.png]]

notes:

Though I have used the excellent terminal ui lazygit for many years, it started to slow down for me on extremely large repositories.

While gitui is still under development and has not yet reached parity with lazygit, it's got everything I need for day to day use, and is FASTER and pure rust.

---

## `cargo install irust`

![[irust-demo.png]]

notes:
iRust is smorgasbord of rust tools in one easy to use command.

In addition to interactive experimentation, like you might be used to in python, ruby, and javascript, you can debug rust code, expand macros and even use irust from an editor, for repl-driven development.

If you want all this in a browser, I recommend using the evcxr kernel inside a jupyter notebook.

---

### `cargo install evcxr_jupyter`

![[evcxr-jupyter-error-demo.png]]

notes:
This tool is not actually pure rust, and requires the Jupyter framework to be installed, which you probably can get from your package manager, but evcxr is a rust kernel for jupyter and is SO GOOD I simply must include it in this list.

useful for data scientists, teaching, or anywhere a graphical representation of data is useful, jupyter has been the standard notebook for over a decade at this point, most languages have kernels that work with it already, and here's the Rust one, built by google, the champions of the _"right hand doesn't know what the left hand is doing"_ competition.

---

## `cargo install bacon`

![[bacon-type-error-demo.png]]

notes:

Bacon is your constant companion while learning and writing Rust at all levels.
On the surface, it just re-runs cargo clippy, build, test or run, and you get those errors from your LSP-capable IDE, right?

Not as nicely as they are in bacon, it refreshes instantly, is flicker-free, and I find it a complament to an LSP editor, not a replacement.

A bonus is that in the run mode, it acts as an auto-reloading server, when your program is a long-running service. Change the code, and bacon will kill it and run it again. 

Hot reloading! Almost.

---

### `cargo install cargo-info`

![[cargo-info-serde.png]]

notes:

In my quest to never open a web browser, a challenge I sometimes face is that I need to do a search on crates.io.

cargo info does is exactly what I want, and has options to show the features when you need to remind yourself what to cargo add.

---

# Applications

notes:
And finally some FUN.
%% 10:00 %%

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
