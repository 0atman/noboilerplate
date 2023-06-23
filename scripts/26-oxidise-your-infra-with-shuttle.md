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
axum = "0.6.10"
axum-extra = { version = "0.4.2", features = ["spa"] }
tower-http = { version = "0.4.0", features = ["fs"] }
serde = "1.0.148"
serde_json = "1.0.96"

shuttle-axum = { version = "0.16.0" }
shuttle-runtime = { version = "0.16.0" }
shuttle-static-folder = "0.16.0"

tokio = { version = "1.26.0" }
[dependencies.sqlx]
version = "0.6.2"
features = [
	"runtime-tokio-native-tls",
	"postgres", 
	"offline"] 
[dependencies.shuttle-shared-db]
version = "0.16.0"
features = ["postgres"]
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


# Setup

```rust
```



%%


![[shuttle-logo-white.png]]


# Infrastructure from code

notes:
%%
- Tell them what you're going to tell them
- Tell them
- Tell them what you told them
%%
Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

Rust is a very new language, existing CI and production pipelines are ill-suited to it, and don't take advantage of its superpowers.

Rust's type system is so powerful you can encode your infrastructure inside it.

The dream of infrastructure from code, No yaml, no terraform, just pure rust, validated by your local compiler is possible, and it's a technique you can use with shuttle.rs, who are both subject and sponsor of today's video.

---

## Public Domain Videos

[github.com/0atman/noboilerplate/](https://github.com/0atman/noboilerplate/)

notes:
Everything you see in this video from the script to the images are part of a markdown document available on github under a public domain license.

---


# What is

![[shuttle-logo-white.png|500]]

<h1>?</h1>

notes:
Shuttle is a y-combinator-backed company that I first heard of last year on hackernews.
I was impressed by what I saw: Think Heroku, but with an actual free tier, or like Vercel for backends.
No yaml, no terraform, just pure rust, if you need some infra, you can just have it.

It's quite a impressive claim, let's try it out.

---

![[shuttle-tutorial.png|700]]
https://docs.shuttle.rs/introduction/quick-start

notes:

To get up and running fast, we're going to follow the 5-part shuttle quickstart in the first half of this video, logging in to shuttle using github, generating an api key, and deploying a hello world app.

---

```bash
$ cargo install cargo-binstall  # first get binstall
```
```bash
$ cargo binstall cargo-shuttle  # then binstall shuttle

 INFO resolve: Resolving package: 'cargo-shuttle'
 WARN The package cargo-shuttle v0.16.0 
      will be downloaded from github.com
 INFO This will install the following binaries:
 INFO   - cargo-shuttle 
 INFO Installing binaries...
 INFO Done in 4.5s
```
```bash
$ cargo shuttle login
```

notes:

## Installing shuttle

Assuming you've installed rust from rustup.rs, installing shuttle is easy, as there is a binary install for it.

If you're not familiar with cargo-binstall, it is a wrapper around cargo install that checks github releases and a few other sensible places for pre-built binaries for your version and architecture.
This cuts down install time to seconds.

If there isn't a matching binary for your system, it falls back to compiling as normal.
There will be for shuttle, as shuttle even build binaries for my weird setup: asahi linux running on apple silicon.

---

# Tutorial part 1

notes:

Let's get a hello world working quickly.

---


- axum
- poise
- poem
- rocket
- salvo
- serenity
- tide
- thruster
- tower
- warp

notes:

At time of recording shuttle support
{}
frameworks.

---

```md
❯ cargo shuttle init
How do you want to name your project?
It will be hosted at ${project_name}.shuttleapp.rs.
✔ Project name · shuttest3
Where should we create this project?
✔ Directory · /home/oatman/projects
Shuttle works with a range of web frameworks.
Which one do you want to use?
· axum
    Creating project "shuttest3" in "~/projects"
✔ Do you want to create the project on Shuttle?
· yes

project 'shuttest3' is ready
```

notes:

## Creating a new shuttle project


We'll make a simple Axum project today.

Shuttle has a setup wizard to start a new template project so you can get to hello world with no code.

Initial compilation, as always with Rust is slower, but subsequent builds are fast.

2.5 minutes for initial and 40 seconds for subsequent deploys I saw in my tests while making this video.

---

![[shuttle-test-axum-helloworld.png]]

notes: 
well that was easy.
We've got a hello world with a lets encrypt ssl certificate up on the internet without any code so far.

What ELSE can we do with shuttle?

---


![[infra-from-code.gif]]

> "Get a database by just asking for one in your Rust code."

notes:

Over at shuttle.rs they are not shy about their dedication to Rust which I love to see.
Rust is different, that's the point.
We need different CI tools.

They say you can get a database by just by asking for one, let's see what that looks like.

---

# Ask for a DB

```rust[]
#[shuttle_runtime::main]
async fn axum() -> ShuttleAxum
```

# Get a DB

```rust[]
#[shuttle_runtime::main]
async fn axum(#[Postgres] pool: PgPool) -> ShuttleAxum
```

notes:

(signature-valid pseudocode)


## Introduction to "infrastructure from code"

Here is the signature of our main axum entrypoint that shuttle init created.
The only change to vanilla axum is the return value, which uses the wrapped shuttle axum value for the axum router.

I'll demo you all the code in a moment, but I want to highlight how easy it is to request a database from shuttle.

In the second codeblock, I've added a new param to the main function which is a sqlx pgPool struct.
Normally we'd have to create this postgres pool manually ourselves, with a database uri or similar.

The shuttle-provided postgres annotation here builds this pool for us with sensible defaults. 

Now, either locally or when built on shuttle's servers, your code will be passed a valid connection pool, connected transparently in dev to a local shuttle-managed docker database.
you only need docker installed

If you would like to manage your own dev database, you can configure it to do that too, by passing in a `local_uri`

---


![[shuttle-run-db.png]]

notes:

Here's the output from `cargo shuttle run`, you can see the databases connected, 

---

![[shuttle-run-static.png]]

notes:

extra plugins like the static plugin here, 

---

![[shuttle-run-log.png]]

notes:

and then the output from the server log.

---

# Shuttle features

notes:
## Mid-roll 30s break

Let's talk about some of shuttle's impressive features.

---


| AWS      | SHARED DB         | PERSIST         |
| -------- | ----------------- | --------------- |
| Postgres | Postgres          | shuttle-persist |
| MySql    | Postgres (RusTLS) |                 |
| MariaDB  | Mongo             |                 |

(Upcoming: Memcache, DynamoDB, Turso)

notes:

## Supported and future databases

There are three main databases Shuttle supports at time of recording.
AWS integration, and two native shuttle methods, shared db, and persist.
Shared db uses a large database that shuttle provides for you, shared with other users.
You get your own private section of the database, but the server is managed by shuttle.

Persist is interesting, and deserves a demo

---

```rust[]
struct MyState {persist: PersistInstance}

#[shuttle_runtime::main]
async fn rocket(
    #[Persist] persist: PersistInstance, // <--
) -> ShuttleRocket {
    let state = MyState { persist };
    let rocket = rocket::build()
        .mount("/", routes![retrieve, add])
        .manage(state); // state is now available
    Ok(rocket.into())
}
```

```rust[]
state.persist.save(data_to_be_saved)
let restored_data = state.persist.load()
```

notes:

## shuttle-persist demo

Here's a simple rocket.rs demo of shuttle persist.

The persist struct that is passed in inside our MyState wrapper can load and save ANY serde serialisable struct trasparently. 

Behind the scenes, shuttle have told me that this is implemented with a persistant docker volume attached to your project.
This is a genius simple persistance option for when managing a database is overkill.


---

```bash
$ npx create-shuttle-app

✔ What is your project named? … npx-shuttle-app
✔ Use TypeScript with this project? … No / Yes
✔ Use ESLint with this project? … No / Yes
✔ Use `src/` directory with this project? … No / Yes
✔ What import alias would you like configured? … @/*
Creating a new Next.js app in ./npx-shuttle-app.

$ npm run start  # start dev server
$ npm run deploy # deploy to shuttle
```

notes:
Shuttle have created an quickstart template, requiring just node and rust to be installed for a full-stack app.

You Run the `npx create-shuttle-app` command, and
A new project gets initialized which contains:
Shuttle, Axum and other dependencies installed, 
a static folder with a Next.js app inside
and all relevant code required to instantly deploy the full-stack app with `npm run deploy`, which uses cargo shuttle deploy behind the scenes.


Let's talk more about shuttle the service.

---

![[shuttle-github-org.png]]
https://github.com/shuttle-hq/shuttle

notes:

## Github CTA

As you would hope, Shuttle manage their development on github, and based on their response to an issue I filed while writing this video, they're very responsive to contribution.
Do star the project on github, it really helps them out.

---

![[shuttle-discord.png]]

https://discord.gg/shuttle

notes:

## Discord CTA

Support and community organising is over on their Discord, where I immediately found some old friends, which was a delight.

---

# Shuttle Launchpad

A Rust course written by Stefan Baumgartner
(organiser of [Rust Linz](https://rust-linz.at/))

https://www.shuttle.rs/launchpad

notes:
They are also just starting a education newsletter called Shuttle Launchpad. The main idea behind it is to provide users with bite-sized tutorials/guides/resources on their path to learning Rust Web.

This is extremely new, so sign up if you'd like to learn more.

---


# Tutorial part 2

Axum + DB + Static files demo

notes:

A hello world is all very well, but let's build something with persistance showing how to use the shuttle infrastructure from code principles.

---

```rust
use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
struct Todo {
    pub id: i32,
    pub text: String,
}
```


notes:
## 'todo' Data model

Step one as always is our data model.
The full demo in shuttle's example projects supports both saving and loading todos, but we're just going to read from the db in this short example today.

Imagine we have a database full of these objects already.

Deriving from Serialise and FromRow provide our ORM mapping using sqlx, powered by the great serde.

---

%%
```rust
use axum::{routing::get, Router};
use tower_http::services::{ServeFile, ServeDir};
use std::path::PathBuf;
use shuttle_shared_db::Postgres;
use shuttle_static_folder::StaticFolder;
```
%%

```rust
#[shuttle_runtime::main]
async fn axum(
    #[Postgres] pool: PgPool,
    #[StaticFolder] static_folder: PathBuf,
) -> shuttle_axum::ShuttleAxum {
```

notes:

## Shuttle main annotations deep-dive

Now lets look at the main function here.

you can learn a lot about a rust function from it's signature.
let's focus on that.

---

```rust[1]
#[shuttle_runtime::main]
async fn axum(
    #[Postgres] pool: PgPool,
    #[StaticFolder] static_folder: PathBuf,
) -> shuttle_axum::ShuttleAxum {
```

notes:
This first line annotates the shuttle entrypoint, this macro expands differently depending on if it is built on your dev machine or shuttle's build system, handling the differences in environment transparently.


---

```rust[3]
#[shuttle_runtime::main]
async fn axum(
    #[Postgres] pool: PgPool,
    #[StaticFolder] static_folder: PathBuf,
) -> shuttle_axum::ShuttleAxum {
```

notes:
Next is shuttle's postgres pool, generated at compile-time by shuttle's Postgres annotation here.

This sets up everything you need to supply the pool, using docker on your local machine, or connecting to a real database if built on shuttle.

---

```rust[4]
#[shuttle_runtime::main]
async fn axum(
    #[Postgres] pool: PgPool,
    #[StaticFolder] static_folder: PathBuf,
) -> shuttle_axum::ShuttleAxum {
```

notes:
Next is a clever workaround for deployment differences.

Serving static files is very different from how you might have experienced in other languages, where we might have them served by a frontend like nginx or another proxy.

This is less important in Rust, as it is not only possible but easy to get the same or better performance than these traditional frontends with native rust.

Shuttle's StaticFolder annotation makes it just as easy to serve files in local development as on their server (where the folder structure may be very different)

---

```rust[5]
#[shuttle_runtime::main]
async fn axum(
    #[Postgres] pool: PgPool,
    #[StaticFolder] static_folder: PathBuf,
) -> shuttle_axum::ShuttleAxum {
```

notes:

Finally the Axum router is converted into a shuttle-compatible struct ready for serving. 

---

```rust

    let router = Router::new()
		.route("/todos", get(todos))
		.nest_service("/", ServeDir::new("dist")
			.not_found_service(
				ServeFile::new("dist/index.html")),)
        .with_state(pool);

    Ok(router.into())
}
```

notes:

Here is the body of the main function, setting up the axum router with two endpoints, our todos list, and static files inside a dist directory served from the root. 

This could be where our frontend app, written in a javascript or webassembly framework could live.

Note the router.into() line, hooking into the From trait for the ShuttleAxum return value.

---


```rust
use axum::extract::State;
use sqlx::{PgPool, query_as};

async fn todos(State(pool): State<PgPool>) -> String {
    let todos = query_as!(Todo, "SELECT * FROM todos")
        .fetch_all(&pool)
        .await
        .unwrap();
    serde_json::to_string(&todos).unwrap()
}
```


notes:
## `todos` route

Finally, here's our single controller function, this is the handler mounted at `/todos` in our little demo app.

Because we've wired up shuttle's postgres pool into our app, every handler function is giving a state struct with our postgres pool inside.
This is how state works in functional languages, the state is passed from trunk to leaf, passing down the function tree.

The handler returns a simple string representation of the todos in our database, if we wanted we could build a richer html representation or even json, to be consumed by a frontend.

---



```rust[5]
use axum::extract::State;
use sqlx::{PgPool, query_as};

async fn todos(State(pool): State<PgPool>) -> String {
    let todos = query_as!(Todo, "SELECT * FROM todos")
        .fetch_all(&pool)
        .await
        .unwrap();
    serde_json::to_string(&todos).unwrap()
}
```

notes:

Note we are using sqlx's fantastic `query_as!()` macro to validate that sql query on our actual database AT COMPILE TIME.

I've raved about how you can't get this real-world schema and query validation anywhere else in other videos, check them out for more details.

but What if you don't want to run a local dev database, or you're running in an environment, like ci or github actions, where the database isn't available?

sqlx has an offline feature, which is LOVELY, let me show you how to use it:

---

```bash
$ cargo sqlx prepare
```

```json
{ // sql-data.json
    "db": "PostgreSQL",
    "describe": {
	  "query": "SELECT * FROM todos",
      "columns": [
        {
          "name": "id",
          "type_info": "Int4"
        },
        {
          "name": "note",
          "type_info": "Text"
        }
...
```

(some keys missing)

notes:
## sqlx offline demo

Running `cargo sqlx prepare` builds this little json file that you check in to your project that contains the valid schema and queries of your database.
If the file exists, sqlx transparently uses it instead of connecting to the db.

You should always try to use a real db where possible, but this is a smart escape hatch!

---

# Shuttle.rs Feature Roadmap


notes:
I'll finish up with an exclusive look at the future of shuttle.

Shuttle's pace of development is very fast, and they have shared with me some extremely promising upcoming features.

---

# The road to Beta

notes:
Shuttle is currently still in Alpha, papercuts & stability issues, among other things, are somewhat expected. 
They are planning to hit beta this summer with new features and a step towards being able to support production-ready apps.

---

![[infra-from-code.gif]]

(mockup of Shuttle Console)

notes:

## Shuttle Console

In a few weeks Shuttle plan on releasing a web console to allow you to visualise and manage projects, resources, logs, and so forth.

This will expose the details that are already available on the command line, but in an alternative way.

You know I love the command line, but I also love options.


---

```rust[]
shuttle_next::app! {
    #[shuttle_next::endpoint(method = get, route = "/")]
    async fn hello() -> &'static str {
        "Hello, World!"
    }
}
```

https://docs.shuttle.rs/examples/shuttle-next

notes:
Another extremely new project is Shuttle Next.

Next is a batteries-included, WASM-based backend web-framework.
Based on Axum and Hyper, but with the isolation and built-in containerisation of webassembly.

Wasm is the lightest container format we have, and we're starting to see it used more and more on the server, as shuttle are doing here, as a replacement for Docker and k8s.

Shuttle Next is available for very pre-alpha testing and feedback, at the moment the only resource available is an http stream to and from your project.

---

## The Rest!


notes:

shuttle's roadmap for the rest of the year contains:
-  Horizontal Scaling
-  Additional dbs, like Memcache, DynamoDB, Turso
-  Multi-service Networking
-  AWS Lambda 
-  S3 & CloudFront 
-  EBS Storage for Persistent Services


---

# Events


Next workshop 14th of June, sign up at https://workshop.shuttle.rs

notes:

Shuttle have a couple of events coming up such as a workshop on the 14th of June that combines Next.js, Rust & interacting with GPT!

---

![[shuttle-logo-black.png]]


notes:
Go check them out at shuttle.rs!
My thanks to shuttle for their support of this channel.

---

# Want a db?

```rust[]
async fn axum(#[Postgres] pool: PgPool) -> ShuttleAxum
```

# Just ask

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

