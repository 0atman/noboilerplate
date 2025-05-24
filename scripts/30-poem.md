<style>
:root {--r-code-font: "FiraCode Nerd Font";}
.reveal .hljs {min-height: 50%;}
</style>

f7f7f7 background slide colour

# Cargo.toml

```toml
[package]
name = "poem-demo"
version = "0.1.0"
edition = "2021"

[build-dependencies]

[dev-dependencies]

[dependencies]
color-eyre = "0.6.2"
tracing = "0.1.37"
poem-openapi = { version = "2", features = ["swagger-ui", "openapi-explorer"]}
poem = "1"

sqlx = { version = "0.6.0", features = ["runtime-tokio-rustls", "sqlite"] }
tokio = { version = "1", features = ["full"] }

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
use poem::{
    error::InternalServerError, listener::TcpListener, web::Data, EndpointExt,
    Result, Route, Server,
};
use poem_openapi::{
    payload::{Json, PlainText},
    Object, OpenApi, OpenApiService,
};

use sqlx::SqlitePool;

```

# Setup

---

![[rust-logo.png]]

# A Good Rust Stack

notes:
%%
- Tell them what you're going to tell them
- Tell them
- Tell them what you told them
%%
Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

Today I'm going to make some web service recommendations, mixing some of my favourite rust crates together into something that I use across most of my Rust projects.

---

# Naming IS HARD

- ~~FASTERTHANTHINE~~
- ~~RTTM~~
- ~~SAFe~~
- ~~PRIMESTACK~~
- ~~RSTY~~
- ~~GJENGSTACK~~
- _THE STACK_

notes:

We'll start with the hardest problem in computer science, specifically that acronyms don't always make the words you want them to.

I'll just call what I'm using "The Stack" and let you come up with your own name if you like.

The stack I'm going to talk about today isn't the best stack, nor most fully featured or fanciest.
It's my favourite stack, it showcases what rust is GREAT at, and it focusses on correctness above all else.

---

# Public Domain Videos

[github.com/0atman/noboilerplate/](https://github.com/0atman/noboilerplate/)

notes:
Everything you see in this video from the script to the images are part of a markdown document available on github under a public domain licence.

---

# web.py

```python
import web

urls = (
    '/', 'index'
)

class index:
    def GET(self):
        return "Hello, world!"
```


_“Think about the ideal way to write a web app. Write the code to make it happen.”_
&mdash; Aaron Swartz

notes:

So let's talk about what we're NOT going to be doing, with a counter example.
web.py is one of the many gifts the late, great Aaron Swartz gave us.
It's elegant in its simplicity, but we are not building simple tools, today we are building guaranteed correct tools.

Python, and by virtue, web.py can't give you these guarantees, lots of things are just strings, and the language can't help you with compile time checking very much.

Luckily we have Rust.

---

If you want to get mad,

Look up what happened to Aaron Swartz.

Aaron helped create RSS, Creative Commons, Markdown, and Reddit.

He was born only 3 months after me, and if we lived in a kinder world would still be here. 

Thank you Aaron

---

| CRATE        | DESCRIPTION                                           |
| ------------ | ----------------------------------------------------- |
| Color-eyre   | Ergonomic Results and colourful errors. `anyhow` fork |
| iRust        | Fully-featured REPL, debug, asm inspection.           |
| Bacon        | Build, clippy, test, run watcher.                     |
| Tracing      | Async-native logging.                                 |
| SQLx         | Rust-native correct sql                               |
| Poem-openapi | Fast, correct, and ergonomic REST builder             |

notes:

I have some crates that I use in nearly all my Rust projects, this feels like my own personal standard library.

There are options for all of these, if you would prefer anyhow to eyre, or evcxr to irust, you can't go wrong.

I'll explain some of them briefly.

---

## color_eyre

```rust[]
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    
    None::<i32>.unwrap(); // <- panic at the disco
	
	Ok(())
}
```

```js
$ cargo run
thread 'main' panicked at 
'called Option::unwrap() on a None value', main.rs:8:17
run with RUST_BACKTRACE=1 environment variable
to display a backtrace
```

notes:
Eyre, pronounced like the book "Jane Eyre", or the Result variant, `Result::Err`, is a fork of the ubiquitous `anyhow` crate, a trait object based error type for easy idiomatic error handling, but Eyre allows customisable error reports, which is exactly what color_eyre hooks into.

Install color_eyre as shown, and enable the RUST_BACKTRACE environment variable, and it turns this normal looking Rust error, which is functional, but boring into

---

![[color-eyre-backtrace.png]]

<https://lib.rs/crates/eyre>

notes:
A full colour traceback!

Runtime errors are dreadful in any language, and with Rust they are nearly always avoidable. However, there are cases where unrecoverable errors happen, and when they do you will want color-eyre.

The whole readme of eyre is WONDERFUL, btw, and you should read the entirety of it.

ONWARDS

---

## Bacon

![[bacon-type-error-demo.png]]

notes:

I LOVE BACON.

If I'm coding Rust, bacon is always open, running clippy (by pressing c after launch) even when I'm editing in an LSP editor.
There's nothing clearer than rustc's output, and when I'm coding, I need my conversation with the compiler to be as clear as possible.

---

![[tracing-logo-splash.svg]]

```rust[]
fn myfunc() {
	debug!(feeling = "yay!", "I'm gonna shave a yak.");
	info!("yak size: challenging.");
	warn!("could not locate yak!");
	error!("yak ESCAPED!");
}
```

notes:
tracing is Tokio-native logging.
Everything's in tokio, web servers, http request crates, parallel processing libraries - you might as well START off with a logging handler that is great at async logging.

---

# Things not in my standard library:

- Serde
- Tokio
- A computer
- A keyboard

notes:

My stack doesn't explicitly include serde or tokio for the same reason I won't be telling you to type with a keyboard - or execute code with a CPU, it's a prerequisite of using the other crates so we don't need to go over them.

---

# SQLx

```rust[]
struct Country { country: String, count: i64 }

let countries = sqlx::query_as!(
	Country,
	"SELECT country, COUNT(*) as count
	FROM users
	GROUP BY country
	WHERE organization = ?",
	organization
)
    .fetch_all(&pool) // -> Vec<Country>
    .await?;
```

notes:

I'll show you sqlx actually working when we build our api in a moment, but it is is so alien you'll need to start processing this now. here's the gist

SQLx allows the same structs you build the state of your program with to validate your sql queries at compile time.

got it?

I'll pin my deep-dive testing video if you would like to watch that too.

---

# Poem-openapi

> A program is like a poem, you cannot write a poem without writing it.

&mdash; Dijkstra

notes:

FINALLY the meat of the stack.

Poem came to my attention in 2022 as I evaluated the rust web framework landscape.
There's a lot of really great options, and so I tried them all.

Now I'm going to have to talk about benchmarks in this section, so I need everybody to be chill OK?

OK.

---

![[poem-benchmarks.svg]]

[web-frameworks-benchmark.netlify.app](web-frameworks-benchmark.netlify.app)

notes:
I have included all rust frameworks alongside some fast and slow other frameworks.

I discounted unpopular languages and frameworks, this included nim, cpp and a load of others that aren't relevant, you'll never persuade your team to use them, so you won't be using them.
Popularity matters, that's why Rust is so exciting.

I used these synthetic benchmarks to get a feel for if any rust frameworks were going to hold me back when scaling, and the answer is no, not really.

As we all know, speed often isn't very important, but if you can have it, you might as well get it.

To illustrate this, look at the bottom of this graph, at our old friends Express, Sinatra, and Flask.

If this were a popularity graph instead of requests per second, EACH of these mature frameworks would eclipse all above them!
Our industry has decided that they are fast enough, and who am I to disagree.

Looking at the top of the graph, to be fair (AND BALANCED) I have included the two fastest java and javascript frameworks in the WORLD.
These are obscure, rely mostly on native code tricks, and are still only just edging out the Rust pack, despite both Java and Javascript's literal trillions of dollars over decades in optimisations.

Very interesting!

---

![[poem-benchmarks -100k.svg]]

[web-frameworks-benchmark.netlify.app](web-frameworks-benchmark.netlify.app)

notes:

So my search led me to this speedy part of the universe, Rust frameworks that were all benchmarking at well over 100k, putting them amongst the fastest on the planet.

After looking at them all, I fell in love with Poem, and specifically their first-party rest api plugin, poem-openapi.

Let's look at it

---

```rust[]
struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "get")]
    async fn echo(&self, name: Query<Option<String>>)
		-> PlainText<String> {
        
        match name.0 {
            Some(name) => PlainText(name),
            None => PlainText("hi!".to_string()),
        }
	}
}
```

notes:
Here's a simple plain-text endpoint mounted at /hello

We have our top-level struct that we hang handler functions off
The openapi macro surrounds this, adding lots of boilerplate that we don't care about.
The `oai` macro converts these simple functions into openapi handlers behind the scenes too,
Then we have the handler functions with validated inputs, in this case just one echo handler.
And the query type guarantees safe matching of query params at compile time.

---

# [patreon.com/](https://www.patreon.com/noboilerplate)

# [noboilerplate](https://www.patreon.com/noboilerplate)

notes:

No ad this episode thanks to my patrons, and before we build our api with poem, I'll briefly tell you about how this is possible.

I have quit my day job and am now making videos and podcasts FULL TIME supported by my wonderful patrons and channel sponsors.

Thank you everyone for watching and supporting me along this wild ride.

On my Patreon there are various levels, but all get early ad-free and tracking-free videos and access to a private section of my discord server

---

![[namtao-discord-jul-23.png]]

## <https://noboilerplate.org>

notes:
By the way did you know I run a huge discord server? Over 2k lovely people chatting about Rust, music, mental health and other cool stuff!

---

![[nb-patreon-mentoring-jul23.png]]

notes:

Now that I am doing production full time, I have increased the number of slots on the new Mentoring Patreon tier to 25.
I can teach you anything I talk about on my channel, see the pinned announcement video for details.

If it's fully booked, message me on patreon or email me and I'll add you to the waiting list!

---

# LET'S BUILD

notes:

let's build!

---

```rust
#[derive(Object)]
struct Todo {
    id: i64,
    description: String,
    done: bool,
}
```

```rust
type TodoResponse = Result<Json<Vec<Todo>>>;
```

```rust
struct TodosApi;
```

notes:

## data model

First, as always, we start with the data model.
These first two structs defines both the database and API interface into our data.

The third is a top-level poem-openapi struct that we hang our  api interface on.
We'll see the implement blocks for this in a moment.

Binding your database and rest interface together AT COMPILE TIME allows us to have extreme confidence in our data integrity.

This is a SUPER POWER: if you define your API in rust structs, the COMPILER will stop you from breaking the contract with your api clients.

Not only that, but if you don't see a PR where the structs change, your colleagues and collaborators haven't broken the contract EITHER.

Huge!

---

```rust
#[OpenApi]
impl TodosApi {
    #[oai(path = "/todos", method = "post")]
    async fn create(
        &self,
        pool: Data<&SqlitePool>,
        description: PlainText<String>,
    ) -> Result<Json<i64>> {
```

```rust
        let id = sqlx::query!(
	        "insert into todos (description) values (?)",
            description.0
		)
		.execute(pool.0)
		.await
		.map_err(InternalServerError)?
		.last_insert_rowid();
		
        Ok(Json(id))
    }

```

notes:

Now we roll into the impl block for our TodosApi struct.

Our handler functions are defined here, with our database pool being passed in as the first parameter, and the post body as the second.

This first function creates our TODOS, and handles a post to /todos

The return type is the integer ID returned by the database, wrapped in JSON.

---

```js
// (TodoResponse = Result<Json<Vec<Todo>>>)
```

```rust
    #[oai(path = "/todos", method = "get")]
    async fn get_all(&self, pool: Data<&SqlitePool>
    ) -> TodoResponse {
```

```rust    
        let todos = sqlx::query_as!(
	        Todo, 
	        "SELECT * FROM todos"
	    )
		.fetch_all(pool.0)
		.await
		.unwrap();
        
        Ok(Json(todos))
    }
}
```

notes:

Next we define the get_all handler, also mounted at /todos.
The database pool is passed in here too, as it will be automatically on all our handler functions - it's passed in by the router, as we will see in a moment.

You can see the return value here is our TodoResponse, a vector of Todo structs wrapped in json.

Now we're using sqlx's compile-time verified query to pull our todos from the db.
Note the parameter before the sql query, that's our Todo struct, the same that is used to define the items in our REST interface.

If I break the query, watch what happens:

---

```rust[3]
let todos = sqlx::query_as!(
	Todo, 
	"SELECT TYPO FROM todos"
)
```

```js
 1  error: error returned from database: 
           (code: 1) no such column: TYPO
   --> src/main.rs:44:21
    |
 44 |           let todos = sqlx::query_as!(
    |  _____________________^
 45 | |             Todo,
 46 | |             "SELECT TYPO FROM todos"
 47 | |         )
    | |_________^
    |
```

notes:

SQLx caught my incorrect sql query by connecting to my local database, in this case, sqlite, at compile time.

This SQL error comes from the rust compiler.
SQLx taught the rust compiler to understand how to connect to a database.

black magic right? I have a video that explains what is happening here, called Rust's Witchcraft.

Let's finish up with the poem config in our main function

---

```rust
#[tokio::main]
async fn main() 
	-> Result<(), Box<dyn std::error::Error>> {
```

```rust
    let pool = 
	    SqlitePool::connect("sqlite:todos.db").await?;
    let api_service = 
	    OpenApiService::new(TodosApi, "Todos", "1.0.0")
	    .server("http://localhost:3000");
    let ui = api_service.openapi_explorer();
    let route = Route::new()
        .nest("/", api_service)
        .nest("/ui", ui)
        .data(pool);
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(route).await?;
    Ok(())
}
```

notes:

That's a bit much, let's just focus on the function body

---

```rust[0-2]
let pool = 
	SqlitePool::connect("sqlite:todos.db").await?;
let api_service = 
	OpenApiService::new(TodosApi, "Todos", "1.0.0")
	.server("http://localhost:3000");
let ui = api_service.openapi_explorer();
let route = Route::new()
	.nest("/", api_service)
	.nest("/ui", ui)
	.data(pool);
Server::new(TcpListener::bind("127.0.0.1:3000"))
	.run(route).await?;
Ok(())
```

notes:

First we set up our connection pool using sqlx. This is configurable, but has sensible defaults. Sqlx supports postgres, sqlite, mysql, and mssql.
If you have the choice, you should use postgres.

---

```rust[3-5]
let pool = 
	SqlitePool::connect("sqlite:todos.db").await?;
let api_service = 
	OpenApiService::new(TodosApi, "Todos", "1.0.0")
	.server("http://localhost:3000");
let ui = api_service.openapi_explorer();
let route = Route::new()
	.nest("/", api_service)
	.nest("/ui", ui)
	.data(pool);
Server::new(TcpListener::bind("127.0.0.1:3000"))
	.run(route).await?;
Ok(())
```

notes:
Next we're building the poem-openapi service. Note we've passed in our top-level TodosApi struct, which has all of our handler functions implemented.
Next is a friendly name for the API, and a version number that is exposed to our api clients.

---

```rust[6]
let pool = 
	SqlitePool::connect("sqlite:todos.db").await?;
let api_service = 
	OpenApiService::new(TodosApi, "Todos", "1.0.0")
	.server("http://localhost:3000");
let ui = api_service.openapi_explorer();
let route = Route::new()
	.nest("/", api_service)
	.nest("/ui", ui)
	.data(pool);
Server::new(TcpListener::bind("127.0.0.1:3000"))
	.run(route).await?;
Ok(())
```

notes:

We then build an instance of the openapi_explorer frontend. This is a built-in UI for poem-openapi, and isn't the only one available. Poem also supports swagger-ui, RapiDoc, and Redoc.

Here's what our openapi explorer looks like

---

## Get /todos UI

![[poem-openapi-explorer-get.png]]

notes:
Of the UI options bundled with poem-openapi, Openapi explorer looks best to me - they're all built off the same openapi spec that is built from your handler functions automatically.
Note that this is all derived from your structs.

---

## POST /todos UI

![[poem-openapi-explorer-post.png]]

notes:

Unlike in other languages, there is no need for separate api configuration here, the rich type system captured so much of the interface to our data that it can be auto-generated.

Back to the main function

---

```rust[7-10]
let pool = 
	SqlitePool::connect("sqlite:todos.db").await?;
let api_service = 
	OpenApiService::new(TodosApi, "Todos", "1.0.0")
	.server("http://localhost:3000");
let ui = api_service.openapi_explorer();
let route = Route::new()
	.nest("/", api_service)
	.nest("/ui", ui)
	.data(pool);
Server::new(TcpListener::bind("127.0.0.1:3000"))
	.run(route).await?;
Ok(())
```

notes:

Poem's router is very straightforward, the route is initialised using the builder pattern, adding routes with the nest method, the first parameter being where it is mounted, and the second the handler function of the api.

This is the top-level organisational unit of your app, think of large namespaces like login, user, or search.
You then build loosely couple api structs that define their own url namespaces under them.

Lastly, the data method attaches state to the endpoint, this is how you share state between your handler functions, the most common use of this pattern is to share in a database connection, as we are doing here.

---

```rust[11-12]
let pool = 
	SqlitePool::connect("sqlite:todos.db").await?;
let api_service = 
	OpenApiService::new(TodosApi, "Todos", "1.0.0")
	.server("http://localhost:3000");
let ui = api_service.openapi_explorer();
let route = Route::new()
	.nest("/", api_service)
	.nest("/ui", ui)
	.data(pool);
Server::new(TcpListener::bind("127.0.0.1:3000"))
	.run(route).await?;
Ok(())
```

notes:

And finally, we start the poem server, connecting the routes that we just defined.

09:00ish

---

![[nushell-get-todos-poem.png]]
('http' command built-in to <https://lib.rs/crates/nu>)

notes:

We did it!
A simple post and get interface, with an auto generated prototyping ui and documentation.

What about the frontend?

---

# Frontend Options

| Crate          | Description                    |
| -------------- | ------------------------------ |
| Rstml          | Compile-checked backend HTML   |
| Yew            | Compile-checked frontend HTML  |
| Maud           | Compile-checked backend Pug    |
| ~~Markdown?~~   | Runtime-checked markdown       |
| ~~Templating?~~ | Runtime-checked html templates |

notes:

That's a topic for another video, but I have opinions here:

Though there are many mature templating libraries, and markdown processors, and even a really great pug-like library called Maud, I don't dig these.

For me, Rust code is your templating system, functions and modules are your unit of abstraction, and it's all type safe.

rstml and yew provide RSX style html all checked by the rust compiler, having been programmed by macros in these crates.

---

%%

```rust
struct Api;
```

%%

```rust[6]
#[OpenApi]
impl Api {
    #[oai(path = "/thanks", method = "get")]
    async fn thanks(&self) -> PlainText<String> {
        PlainText(
			"THAT'S ALL, FOLKS!"
		.into())
	}
}
```

notes:

That's the basics of my recommended stack, work with Rust's type system and you'll be rewarded, put all of your logic, from the data, to the api, right up to the HTML into the hands of the type system, play with it with immediate feedback from bacon, and you'll be writing production-grade fast code quicker than you ever thought possible.

Thank you!

---

![[tri-hex-moon-white-transparent.png|300]]

# Thank you

## [Patreon.com/NoBoilerplate](http://www.patreon.com/noboilerplate)

notes:

# OUTRO

If you would like to support my channel, get early ad-free and tracking-free videos, vip discord access or 1:1 mentoring, head to patreon.com/noboilerplate.

If you're interested in transhumanism and hopepunk stories, please check out my sci-fi podcast, Lost Terminal.

Or if urban fantasy is more your bag, do listen to a strange and beautiful podcast I produce called Modem Prometheus.

Transcripts and compile-checked markdown sourcecode are available on github, links in the description, and corrections are in the pinned ERRATA comment.

Thank you so much for watching, talk to you on Discord.
