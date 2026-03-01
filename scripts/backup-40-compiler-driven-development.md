---
audience:
---
<style>
:root {--r-code-font: "FiraCode Nerd Font";}
.reveal .hljs {min-height: 50%;}
</style>
%%
f7f7f7 background slide colour
or maybe 191919

# Cargo.toml

```toml
[package]
name = "compiler-driven"
version = "0.1.0"
edition = "2021"

[build-dependencies]

[dev-dependencies]

[dependencies]
axum = "0.7.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0.68"
tokio = { version = "1.0", features = ["full"] }

```

# Lint Tweaks

These lints make clippy less noisy when I'm building the video

```rust
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::items_after_statements)]
#![allow(clippy::no_effect)]
#![allow(unused_must_use)]
#![allow(clippy::unused_self)]
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

![[rust-logo.png|200]]

- Higher-level than Go, JavaScript, Java,
	- but as fast as C

- No runtime or garbage collector,
	- but thread and memory safety guaranteed

- Advanced type system (with monads!),
	- but friendliest compiler errors in the business
notes:

# CDD: Compiler-Driven Development in Rust

Hi friends, my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

Rust has many extremely impressive, seemingly even contradictory, claims.

- High level lisp-style metaprogramming AND as fast as low-level C
- A zero-cost hard-realtime language with no garbage collection overheads, but with perfect memory and thread safety.
- Extremely rich type system allowing modelling of data impossible in other popular languages AND gorgeous helpful errors when your modelling is wrong

How can Rust do all these and other languages can't?

**Comprehensive understanding of your code at compile-time.**

---

# Developers Live at Compile time

notes:

Whatever language you code in:
JavaScript, Python, Java, Go, even HTML and CSS, to a certain extent, we developers live at compile time.
(you compile your HTML and CSS, right?)

Sure, we test the code, either by running it interactively or automating that in our tests, but the time when our code is actually executing on the CPU plays a minimal part of our day.

Most of our time is not spent writing or running code, but reading it.

Rust's superpowers are the direct result of a language where the compiler can reason about as much of the code as you can.

The compiler is your best friend, your wingmate, a friendly and infallible pair programmer, and this opens ways of programming that you've never experienced before.

If you know how to use it.

---

![[cc-logo.png]]

## Public Domain Videos

[https://github.com/0atman/noboilerplate/](https://github.com/0atman/noboilerplate/)

(for all [links]() read my scripts here ⬆)

notes:
Everything you see in this video: script, links, and images are part of a markdown document available freely on GitHub under a public domain licence.

---

## Chapter 1

## The Rich Type System

<i class="fas fa-quote-left fa-2x fa-pull-left"></i>
Show me your flowcharts and conceal your tables, and I shall continue to be mystified. Show me your tables, and I won't usually need your flowcharts; they'll be obvious.

&mdash; Fred Brooks, _The Mythical Man-Month_

notes:

Just as good data design can make runtime errors impossible, good program design in Rust can extend that to our program logic, too.

---

## No documentation thanks

# I'm a rustacean

notes:

I'll talk about some patterns to model enormous parts of your code later on, but first, let's see what compiler-driven development can do for us.

We'll start by making it much less necessary to read human-readable documentation.

Self-documenting code is a joke in other languages, it's a reality in Rust:

---

![[axum-readme-github.png]]

<https://github.com/tokio-rs/axum>

notes:

This is the readme of Axum, the most popular Rust web framework, with 3.4M downloads a month.

Reading documentation is for nerds and principle engineers, we're going to reproduce the example in the read me and start hacking, using only the rust compiler, no LSP tricks here.



---

%%

axum boilerplate:

```rust
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
```

%%

```rust
#[tokio::main]
async fn axum_server() {
    let app = Router::new()
        .route("/",      get(root))
		.route("/users", post(create_user));
		
	let listener = TcpListener::bind("0.0.0.0:8080")
		.await.unwrap();
		
    axum::serve(listener, app).await.unwrap();
}
```

%%

```rust
println!("serving...");
axum_server();
```

%%

notes:

Here's the main function. It creates two routes, and serves them on port 8080.
Simple enough, but what are those two routes?

---

```rust
async fn root() -> &'static str {
    "Hello, World!"
}
```

```shell
$ curl localhost:8080
Hello, World!
```

notes:
The first one serves a simple static string. OK great.

---

```rust[]
async fn create_user(Json(payload): Json<CreateUser>) 
    -> (StatusCode, Json<User>) {
    
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}
```

notes:

The second is more interesting, a post handler creating a user from a validated payload of json, and returning the new user as json.
BOTH input and output of this handler are documented and their enforced by the type signature. 

OK, let's break this, what status codes have we got?

---

![[40-clippy-no-item-destroyed.png]]

notes:

Looks like there's an opening for a new status code.

Now this is interesting. Instead of giving us some options of what valid codes are available, which it would if we'd misspelled one

---

## LIVE Block


```rust
async fn create_user(Json(payload): Json<CreateUser>) 
    -> (StatusCode, Json<User>) {
    
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::DELETED, Json(user))
}
```


---

```rust[9]
async fn test1(Json(payload): Json<CreateUser>) 
    -> (StatusCode, Json<User>) {
    
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::from_u16(201).unwrap(), Json(user))
}
```

---

```rust
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
```

notes:

# Axum demo

---

```rust
enum FlightState {
	Boarding,
	Taxiing,
	Takeoff,
	Cruising,
	Landing,
	Deboarding,
} 
```

```rust
fn takeoff(flight: FlightState) -> FlightState {
	match flight {
		FlightState::Taxiing => FlightState::Takeoff,
		_ => flight
	}
}
```

---

# Rust is the opposite of Perl. Perl Makes Easy Things Easy and Hard Things Possible, Rust Makes Easy Things Possible and Hard Things Easy

<https://devclass.com/2023/03/20/microsofts-visual-basic-why-it-won-and-why-it-had-to-die/>

<https://www.amazon.com/Learning-Perl-Making-Things-Possible/dp/1491954329>

_You know how it's hard to learn something you can't see the point of?

I think this is why senior developers, particularly, flock to Rust.
We've been in the trenches, we've been paged at 4am, we've debugged the same missing semicolon or bad indentation errors a thousand times.

You tell me that with just a bit more syntax, Rust can fix my PTSD? I say SIGN ME UP! 🎉

A junior developer, the sweet summer child, only wants things to be easy NOW.

Rust isn't optimised for easy NOW.
It's optimised for easy FOREVER."_

---

# Compiler-Driven Development

notes:

how to start with CDD in Rust is the same as with TDD, test driven development, but the tests are already written for you.

Red.
Green.
Refactor.

---

# Amortized Complexity

 - first run slow, subsequent runs fast
 - like rust compile times
 - like life

---
"Writing a compiler that would accept all of the valid programs is not possible, thus we're left with the next best thing: a compiler that will reject all invalid programs at a cost of being overly strict."
- [x] who said this?

Rust forces you to fix all your future bugs before you deploy. This causes the steeper learning curve, but given that all future bugs are crammed into the first compile, it's a surprisingly flat curve!

---

# Optimise for Readability

notes:
As you know, languages are read far more often than they are written.
But there's another part to the story here.
Your programs, if you're lucky and doing your job right, will be used by orders of magnitude more people than those who read the code.
So it follows that some small sacrifice of readability is valid, if it benefits you.

---

# Parse Dont Validate

<https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/>

is this the same as [[The Typestate Pattern in Rust]]

---

# Typestate Pattern

## States

```rust[]
struct Light<State> {
    state: State,
}

#[derive(PartialEq)]
struct On {}


#[derive(PartialEq)]
struct Off {}

```

## Transitions

```rust[]
impl Light<Off> {
    fn turn_on(self) -> Light<On> {
        Light {
            state: On {},
        }
    }
}
impl Light<On> {
    fn turn_off(self) -> Light<Off> {
        Light {
            state: Off {},
        }
    }
}
impl<State: PartialEq> Light<State> {
    fn flip<T>(self) -> Light<T> {
        if self.state == (On {}) {
	        Light { state: Off {} }
        } else {
	        Light { state: On {} }
        }
    }
}
```

---

## Correct Transitions

```rust[]
fn correct_transitions() {
	let bedroom_light = Light {
		state: Off {},
	};
	bedroom_light.turn_on().turn_off().turn_on();
}
```

---

## Incorrect Transitions

```rust[2]
let bedroom_light = Light { state: Off {} };
bedroom_light.turn_on().turn_on(); // can't call twice 
```

```sql
error[E0599]: no method named `turn_on` found 
for struct `Light<On>` in the current scope
   |
9  | struct Light<State> {
   | ------------------- method `turn_on` not found
...
41 |     start_state.turn_on().turn_on();
   |     -----------           ^^^^^^^
   |     |
   |     method `turn_on` is available on Light<Off>
```

notes:

- [x] replace this with a screenshot of the error
---

# Typestate Pattern With Traits

## States

```rust[]
fn correct_transitions() {
    let bedroom_light = Light {
        state: Off
    };
    bedroom_light.turn_on().turn_off().turn_on();
}

// Primary struct
struct Light<State: ResponseState> {
    state: State,
}


// States
struct On;
struct Off;



// Trait wiring for Start and Headers,
trait ResponseState {}
impl ResponseState for On {}
impl ResponseState for Off{}

```

## Transitions

```rust[]

// Methods available only in the Start state
impl Light<Off> {
    fn turn_on(self) -> Light<On> {
        Light { state: On }
    }
}

// Methods available only in the Headers state
impl Light<On> {
    fn turn_off(&self) -> Light<Off> {
        Light { state: Off }
    }
}


//impl<T, S> Transformable<T, S> for SomeStruct<T> {
//    type Output = SomeStruct<S>;

//impl<State: PartialEq> Light<State> {
//    fn flip<T>(self) -> Light<T> {
// Methods available in any state
impl<State: ResponseState> Light<State> {

    fn transition() -> Light<State> {
        Light {
            state: State
        }
    }

    /*
    pub fn tst(self) -> Light<On> {
        Light::transition()
    }
    */

/*
    fn flip(self) -> Self {
	    match self {
			Light { state: (On {}) } => Light { state: (Off {}) }
	    }
    }
*/


}
```

---

# Auto-commit Passing Builds

the commit message is

> Fixes [previous error]

---

# Designing at Compile Time

build a whole system with just they type system

show how prototyping at compile time can give real predictions
maybe use dependent types, or at least fake dependent types with custom new()

---

![[tri-hex-moon-white-transparent.png|300]]

# Thank You

## [Patreon.com/NoBoilerplate](http://www.patreon.com/noboilerplate)

notes:

# OUTRO

If you would like to support my channel, get early ad-free and tracking-free videos, vip discord access or 1:1 mentoring, head to patreon.com/noboilerplate.

If you're interested in transhumanism and hopepunk stories, please check out my weekly sci-fi podcast, Lost Terminal.

Or if urban fantasy is more your bag, do listen to a strange and beautiful podcast I produce every full moon called Modem Prometheus.

Transcripts and compile-checked markdown sourcecode are available on github, links in the description, and corrections are in the pinned ERRATA comment.

Thank you so much for watching, talk to you on Discord.

```rust
  println!("That's all folks!");
} 
```
