<style>
.reveal code.md {
  font-size: 1.5em;
  line-height: 1.5em;
}
.reveal code.sh {
  font-size: 0.7em;
  line-height: 1.2em;
}
</style>

# dependencies
```rust
//! ```cargo
//! [package]
//! edition = "2021"
//! [dependencies]
//! serde = "1.0.140"
//! anyhow = "1.0.58"
//! tokio = { version = "1", features = ["full"] }
//! mini-redis = "0.4"
//! color-eyre = "0.6.2"
//! tracing = "0.1.35"
//! tracing-subscriber = "0.3.15"
//! reqwest = { version = "0.11", features = ["json"] }
//! rayon = "1.5.3"
//! aws-config = "0.46.0"
//! aws-sdk-dynamodb = "0.16.0"
//! clap = "3.2.15"
//! sqlx = { version = "0.6", features = [ "runtime-tokio-rustls", "postgres" ]}
//! chrono = "0.4"
//! egui = "0.18.1"
//! yew = "0.19.3"
//! ```
// The above Cargo.toml is used in this presentation
```

# Lint tweaks
```rust
#![allow(dead_code)]
```

# extern crates

```rust
extern crate serde;

#[macro_use]
extern crate clap;

```

# imports
```rust
```

# setup

```rust
fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::fmt().init();
```



---

![[rust-logo.png]]

## Build Your 
## Rust Lightsaber 

notes: 
Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

Today I'm going to help you build your own Rust toolchain by swapping out parts you don't need with parts you do, until you have an elegant weapon, for a more civilised age.


---

![[rust-lightsaber-lower.jpg|700]]

notes:
As with building any lightsaber we're going to start with the housing, batteries, and core

---

![[rust-lightsaber-upper.jpg|700]]

(design https://www.etsy.com/shop/TmaxProps)

notes:
Then move up to the focusing crystals and dials, where we can really make our mark.


---

![[inside a lightsaber.png]]

notes:
As you know, there are many kinds of Lightsabers from BASIC ones to swift or even groovy ones.

Today we are building a Rust Lightsaber.
Lets get started.


---

# Rustup.rs
```sh
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```sh
Welcome to Rust!

This will download and install the official compiler for the Rust
programming language, and its package manager, Cargo.

info: profile set to 'default'
info: default host triple is x86_64-unknown-linux-gnu
info: syncing channel updates for 'stable-x86_64-unknown-linux-gnu'
info: latest update on 2022-07-19, rust version 1.62.1 (e092d0b6b 2022-07-16)
info: downloading component 'cargo'
info: downloading component 'clippy'
info: downloading component 'rust-docs'
info: downloading component 'rust-std'
info: downloading component 'rustc'
info: downloading component 'rustfmt'
info: default toolchain set to 'stable-x86_64-unknown-linux-gnu'

  stable-x86_64-unknown-linux-gnu installed - rustc 1.62.1 (e092d0b6b 2022-07-16)

Rust is installed now. Great!
```


notes:
To build a lightsaber you start with the primary crystal. Many people like to use a Ruby, but we are going to use Rust. Install yours with rustup.rs.

If you're on mac, linux, or WSL, your method will be this script. For windows, there's an installer on the website.

This installs the entire toolchain:
- Rustc, the compiler
- Cargo, the build tool
- Clippy, the linter
- rustfmt, the autoformatter, and
- documentation generation tools 
Among other components

---

## VSCODE

![[vscode.png]]

notes:
Just having a language installed won't do much without a powerhouse of an Editor.
Many are available, if you don't know where to start, Visual Studio code is an excellent place to.
Emacs with spacemacs or doom is great too, as is vim or neovim.

The key thing is to make sure whatever editor you choose has first-class support for LSP, the language server protocol, which we'll talk about in a minute, this means that you will have access to all the features I do.

---


## NVIM
![[nvim.png]]


I am going to use neovim, the lightest LSP-powered editor.

(more details https://sharksforarms.dev/posts/neovim-rust/)

---

![[astronvim.png]]

```sh
$ git clone https://github.com/AstroNvim/AstroNvim ~/.config/nvim
$ nvim +PackerSync
```

notes:

If you want a fantastic out-of-the-box experience, with no configuration I recommend Astro Nvim, which has terrific defaults, and incredible rust support. Once you've installed it, which is a simple git clone, there's a graphical LSP selection process which handles everything you'll need to get language support running.

---

## Astronvim

![[nvim-helloworld.png]]

notes:
After installation you'll have a minimal rust IDE with all the power of LSP.
This represents the most lightweight setup I can think of, and is what we will build upon today.
If you have VSCode or another editor, you'll have the same functionality, but a bit more chrome.

---

## Neovide

```bash
$ git clone https://github.com/neovide/neovide
$ cd neovide
$ cargo build --release
```

https://github.com/neovide/neovide

notes:

We're now going to tweak the focusing crystal of our lightsaber, which is to say, the theme.

Though we've got very far with astronvim, we're going to take it to the next level with neovide

---

## Neovide

![[neovide.png]]

notes:
Neovide, a portmanteau of Neovim and IDE, provides ligatures and font shaping, an animated cursor, smooth scrolling, animated windows, blurred floating windows, and most importantly, emoji support.

It also can connect over a socket to a remote instance of nvim, handy for cloud use

All of this is rendered on the GPU with opengl and is the fastest editor I've ever used.
Neovide is what I have built MY lightsaber around, and I do hope you will too.


---

## LSP

![[rust-lsp.png]]

notes:
But whatever editor you choose, I recommend that you get one that supports LSP.
Though there are heavyweight IDEs such as Eclipse and Intelij that use their own propitiatory static analysis tools, the one that seems to be most used in the open source and rust worlds is LSP.
Neovim has built-in LSP support, and the Rust toolchain that you just installed with rustup came with an LSP server.

---

#### Rust Analyzer

![[rust_analyzer.png]]




```bash
$ rustup +nightly component add rust-analyzer-preview
```

notes:
The LSP plugin we will be using is the new de-facto standard, which is called Rust Analyzer. It started off as a fork of the official LSP server, and has now become the officially recognised one. Your install of rustup will very likely come with it by the time you watch this. If it doesn't, then you'll have to use the nightly toolchain for a few weeks.


---

## EVCXR 

```bash
$ cargo install evcxr_repl
```

```perl
$ evcxr     
Welcome to evcxr. For help, type :help
>> println!("hello repl!")
hello repl!
()
>> 

```

https://github.com/google/evcxr

notes:
Let's plug in a repl.
While rust has almost no runtime, certainly compared to dynamic languages, YOU can have a repl because google were so nice as to build one for us.
EVCXR, a badly named tool that is impossible to remember the name of, is a repl kernel that can be used at the command line, or in a graphical jupyter session in your browser.
Behind the scenes it's just wrapping your code up in a main function, compiling that file and printing the output.

I am including it here because many of my fellow recovering dynamic language addicts will feel relief knowing there is a repl to fall back on.
By all means install it, I did, but you won't need it - Rust's compiler feedback, available in-editor using LSP, is so rich, you'll forget all about the repl.

---

## EVCXR

```perl
$ cargo install evcxr_repl
```

Installs to:

```bash
~/.cargo/bin/evcxr_repl
```

notes:

It's worth pausing for a moment here.
Something special is happening.

Not only does cargo let you manage your rust projects, run tests, linting, formatters, and upload packages to crates.io (the official but not exclusive package registry) you can also install standalone rust programs.

You'll be pleased to learn that these programs are not installed system wide.
Wouldn't that be terrible.
Cargo-installed binaries are added to a sensible location in your home directory.

Let's press on.

---

#### Bacon

![[bacon-rust.png]]

https://crates.io/crates/bacon

notes:
In addition to LSP, I'm going to recommend a terminal-based test runner. You're looking at one called Bacon.

I don't know if it's so many years working in the terminal mines, but I like to see what's what, in plain text.
And despite all the incredible richness that LSP brings to our editors, there's something about the messages the rust compiler sends me that warms my heart.
I like to see both.

Bacon runs your tests, or build, or clippy, or any of the cargo commands (including documentation building) and you can switch between them by pressing t, or r, or c etc.

the real magic is that it picks up changes, seemingly INSTANTLY, with no flickering, it's the smoothest example of this I've ever seen.

An alternative I've used is called cargo-watch, which provides a similar, though non-overlapping featureset. Try bacon first.

---

#### Lazygit

![[lazygit.png]]

https://github.com/jesseduffield/lazygit

notes:
next is lazygit, or it's rust equivalent, gitui, which I discovered though its integration with astronvim. This lovely git ui is just a `SPC g g away`.
Do note neovide's transparencies here.
However they are both normal terminal programs, and can run in any git repo.

It almost makes me forget my time with emacs's magit.
Almost.


///

Now we have set up our editor we can talk about actual rust development libraries. But before I do, I'd like to thank Obsidian, the markdown knowledge base without which my videos, and much of my life, would not be possible.

---


<!-- slide bg="rgb(77, 60, 166)" -->
## OBSIDIAN

![[obsidian-screenshot.png]]

notes:
Obsidian is a powerful knowledge base built on top of a local folder of plain-text Markdown files.

If you've ever used Notion or Roam or Evernote, it fills a similar purpose, it's:

---

<!-- slide bg="rgb(77, 60, 166)" -->
# A second brain
# for you, forever.

---

<!-- slide bg="rgb(77, 60, 166)" -->

> Obsidian is my current choice for writing and organizing notes on any complicated subject. Feels like the closest anyone has come yet to realizing the promise of hypertext as a tool of thought.

&mdash; [Graydon Hoare](https://github.com/graydon)

notes:
Graydon Hoare, whom you might recognise as the creator of Rust, has this to say on the topic of Obsidian.
I couldn't have put it better.
Hypertext was supposed to link our thinking together, and obsidian realises this.


---

<!-- slide bg="rgb(77, 60, 166)" -->
## My SETUP

![[my-obsidian-lightsaber.png]]

notes:
You can think of obsidian as a markdown IDE, with plugins galore.
Here is my exact setup I'm using to edit this script right now.
- At the top is a kanban board, which is simple markdown behind the scenes,
- The reveal.js-powered Advanced Slides plugin's preview is in the upper-left quadrant, with the markdown sourcecode on the right.
- I have this open twice, again in the bottom left as I was noodling with font sizes quite a lot, as they're in the frontmatter.
- Finally the outline is on the right, along with a checklist plugin pulling out markdown checklist items from the current document

---

<!-- slide bg="rgb(77, 60, 166)" -->
![[obsidian-mermaid.jpeg]]

notes:
Though if you don't want to use plugins there's plenty of native features, such as these mermaid diagrams

---

<!-- slide bg="rgb(77, 60, 166)" -->
![[obsidian-graph.jpeg]]

notes: 
and a REALLY COOL graph view of all your notes and the links between them.


---


<!-- slide bg="rgb(77, 60, 166)" -->
![[obsidian-mobile.jpg|275]]

notes:
All native functionality is available on their free mobile app, too, along with many of the community-made plugins.

Obsidian is freeware, available on linux, mac and windows, and Android and iphone too.

My thanks again to the obsidian team for building this incredible app.

---


# FINALLY SOME RUST

notes:
My recommendations come in two parts, the first are built-in to your lightsaber, ie you should use them in nearly all your projects, and the second are tools for your toolbelt. 

Let's talk about some essential libraries.

---

#### TOKIO

```rust
use mini_redis::{client, Result as redis_Result};

#[tokio::main]
async fn main() -> redis_Result<()> {
    println!("in tokio");
    let mut client =
        client::connect("127.0.0.1:6379").await?;
    client.set("hello", "world".into()).await?;
    let result = client.get("hello").await?;
    println!("got value result={:?}", result);
    Ok(())
}
```
https://crates.io/crates/tokio

(_All code examples in this video are compiler-checked_)

notes:
Tokio is the most popular async runtime. If the framework you're using gives your a choice, probably your life will be easier if you choose tokio.
It's used by all my subsequent async recommendations here.

---

#### Eyre
```rust
use color_eyre::eyre::Result;

fn get_cluster_info() -> Result<String> {
    let config = std::fs::read_to_string("cluster.json")?;
    Ok(config)
}
```


notes:
Rust's functional roots mean there are no exceptions that would take you out of the flow of functions, rather, errors are passed by value, using our old friend the Result enum.
As I have explained in previous videos, Results either contain the value you want, or an error, with more information.

Errors can be multiple types from different frameworks, so type signatures can get a little cluttered, as they list all the kinds of errors you might return from a function.

Eyre unifies this under one Result type, in addition to other useful properties, such as...

---

#### Color-Eyre

```sh
❯ cargo run --example custom_section
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running target/debug/examples/custom_section
Error:
   0: Unable to read config
   1: cmd exited with non-zero status code

Stderr:
   cat: fake_file: No such file or directory

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ SPANTRACE ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

   0: custom_section::output2 with self="cat" "fake_file"
      at examples/custom_section.rs:14
   1: custom_section::read_file with path="fake_file"
      at examples/custom_section.rs:58
   2: custom_section::read_config
      at examples/custom_section.rs:63

Suggestion: try using a file that exists next time
```

https://crates.io/crates/color-eyre

notes:
Color-eyre, which is an error report handler for colorful, consistent, and well-formatted error reports for all kinds of errors

For me, the reason all of my programs use this is that it brings Rust's runtime exceptions up to the same high standard as the compiler exceptions 

---

#### tracing

```rust
use tracing::info;

info!("TPS report filed and sent to corporate accounts.");
```

```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running target/debug/lightsabre
Jul 28 12:34:56 INFO lightsaber: TPS report filed and sent to corporate accounts.
```

https://crates.io/crates/tracing

notes:
Sensible logging is a must for complex applications, and because Rust allows us to fearlessly make concurrent programs, we can't just print out stuff and expect all the threads to line up nicely for us.
Tracing is a framework for instrumenting Rust programs to collect structured, event-based diagnostic information.

---


![[jedi-belt-1-inch-loaded.png]]

notes:

Now you have built your main weapon, your lightsaber, it's time to accessorise.

These are tools you should have on your toolbelt ready for use at a moments notice.

we'll start with reqwest:

---

#### Reqwest

```rust
use std::collections::HashMap;

async fn get_ip() -> Result<()> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
```

https://crates.io/crates/reqwest

notes:
Reqwest is a batteries-included http requests library.
It's spelled a little weirdly because if they named it the correct way, our lives would painful trying to google anything.
Every language seems to have a Request library, doesn't it?
And this one is at least spelled uniquely.

Reqwest has both synch and async methods, the default and one that we're using here is async.
Make sure you copy the right examples.


---

#### Rayon

```rust[3]
use rayon::prelude::*;
fn sum_of_squares(input: &[i32]) -> i32 {
    input.par_iter()
         .map(|&i| i * i)
         .sum()
}
```

https://crates.io/crates/rayon

notes:

Welcome to fearless concurrency.

Rayon is a data-parallelism library for Rust which makes it trivial to convert sequential iterators into parallel ones: For example here we have just changed the `input.iter()` call into `input.par_iter()`, and Rayon does the rest.

Rayon taps into Rust's powerful type system to guarantee data-race freedom.

---

#### Aws-sdk-rust

```rust
use aws_sdk_dynamodb::{Client, Error};

async fn print_tables() -> Result<(), Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let req = client.list_tables().limit(10);
    let names = req.send().await?.table_names;
    println!("Current DynamoDB tables: {:?}", names);
    Ok(())
}
```


https://crates.io/crates/aws-sdk-rust

notes:

Amazon are one of the many companies taking Rust seriously.
Here, we're querying our dynamodb table names.

Amazon auto-generate this SDK from [Smithy models](https://awslabs.github.io/smithy/) that represent each AWS service.
I didn't learn what that means, but it sounds good doesn't it?
Links in the sourcecode for the curious.

---

#### Clap

```rust
fn clap_main() {
    let _matches = clap_app!(myapp =>
        (version: "1.0")
        (author: "Alice A. <alice@example.com>")
        (about: "Does awesome things")
        (@arg CONFIG: -c --config +takes_value "config")
        (@arg debug: -d ... "Sets debugging level")
        (@subcommand test =>
            (about: "controls testing features")
            (@arg verbose: -v --verbose "Verbose test")
        )
    ).get_matches();
    // Handle cli args from here
}
```

https://crates.io/crates/clap

notes:
Clap is the most popular command line args-parsing library.
It seems to get more and more featureful each time I look at it.
This example here uses the `clap_app!` macro to make your config extremely small. However, you can use the normal functional builder pattern (which this Macro unpacks to) or even define it in a yaml file, unpacked and checked at compile-time by clap's `load_yaml!` macro.

Personally, I like to keep things pure rust.

---

#### SQLX

```rust
async fn sqlx() {
    use sqlx::postgres::PgPool;
    let pool = PgPool::connect("localhost").await.unwrap();

    #[derive(sqlx::FromRow)]
    struct User { name: String, id: i64 }
    
    let _stream = sqlx::query_as::<_, User>(
    "SELECT * FROM users WHERE email = ? OR name = ?")
        .bind("test@example.com")
        .bind("test")
        .fetch_all(&pool);
}
```

https://crates.io/crates/sqlx


notes:

SQLX is my favourite SQL framework.
No ORMs, no layers in your way. Just compiler-checked SQL, as I've demoed multiple times in previous videos.

Here, as a change, is the non-macro way to query your db using sqlx. Fully async, pure rust, even down to the database drivers and rust version of openssl.
Lovely.

---

#### Chrono

```rust
use chrono::prelude::{DateTime, Local};

let _local: DateTime<Local> = Local::now();
// =~ `2014-11-28T21:45:59.324310806+09:00`
```

https://crates.io/crates/chrono

notes:

Chrono is a strict ISO 8601 timezone-aware datetime library.

Nothing fancy, but you're gonna need it.


---

#### EGUI
```rust
fn ui_counter(ui: &mut egui::Ui, counter: &mut i32) {
    ui.horizontal(|ui| {
        if ui.button("-").clicked() {
            *counter -= 1;
        }
        ui.label(counter.to_string());
        if ui.button("+").clicked() {
            *counter += 1;
        }
    });
}
```

https://crates.io/crates/egui

notes:
There's a few gui libraries available in Rust at the moment. Tauri is a huge project, but you have to write your app in javascript, and that doesn't spark joy for me.

EGUI, here, is pure rust, runs at 60fps, and is not reliant on native c libraries, which means it's portable and even runs in webassembly. 

If you want to write native HTML, however, you want. YEW:

---

#### Yew.rs

```sh
impl Component for Model {
    fn create(ctx: &Context<Self>) -> Self {
        Self {value: 0,}
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {self.value += 1; true}
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div>
            <button onclick={ctx.link().callback(|_| Msg::AddOne)}>{ "+1" }</button>
            <p>{ self.value }</p>
        </div>
        }
    }
}
```

https://crates.io/crates/yew

notes:
Yew is a reactive webassembly-powered ui toolkit that is faster than Reactjs, type-checked, and has native html support thanks not to a pre-processor like babel, but rust macros.

%%
this example doesn't compile, apologies I ran out of space for all the imports etc
%%

---

#### Clippy
```shell
cargo clippy --fix -- \
-W clippy::pedantic \
-W clippy::nursery \
-W clippy::unwrap_used \
-W clippy::expect_used
```

https://github.com/rust-lang/rust-clippy

notes:

One final thing is you should set up Clippy for your project with some sensible defaults.

If you're coding for yourself, I recommend enabling as much as possible. like I've done here.

At MINIMUM I want you to turn on `unwrap_used` and `expect_used` warnings.

I consider use of .unwrap() and .expect() as shortcuts to get your code working quickly, but I never commit them.
Think of them as TODOs in your code.

---


![[rust-lightsaber-components.jpg]]

notes:
You now have everything you need to go out into the world and solve problems.
In the highly likely event you need to do something I've not mentioned, just search on crates.io. There's something for everyone there!

From bare-metal chips, through backend code, aws lambdas, and frontend and mobile and app development, Rust can be your constant companion, now and for the next 40 years.

---


![[rust-logo.png]]

## Build Your 
## Rust Lightsaber 

notes:

# OUTTRO


If you'd like to see what you can write in rust, click the top video: I used it to make a fun retro computer visualisation for my hopepunk podcast, Lost Terminal.

Or if urban fantasy is more your bag, click the bottom video to listen to a strange and beautiful podcast I produce called Modem Prometheus.

Transcripts and compile-checked markdown sourcecode are available on github, links in the description, and corrections are in the pinned ERRATA comment.

Thank you so much for watching, see you next time.


(642 packages, hotbuilding in < 1s)

---

%% Thumbnail %%


![[rust-logo.png]]

# For a more civilised age


---

```rust
Ok(())
} // That's all folks!
```
