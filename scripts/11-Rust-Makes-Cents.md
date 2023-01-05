<style>
:root {--r-code-font: "FiraCode Nerd Font";}
</style>

%%

# Cargo.toml

```toml
[package]
name = "template"
version = "0.1.0"
edition = "2021"

[build-dependencies]

[dev-dependencies]

[dependencies]
tokio = "1.21.1"
anyhow = "1.0.65"
```

# Lint Tweaks

```rust
#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(decl_macro)]
```

# Extern Crates

```rust
#[macro_use] extern crate rocket;
```

# Imports

```rust
use anyhow::Result;
use rocket::response::Flash;
use rocket::response::Redirect;
use rocket::form::Form;
```

# Setup

```rust
fn main() {
	println!("Rust Makes Cents");

```

%%

---


![[rust-logo.png]]

# RUST Makes Cents

### Speed is a FEATURE

notes:

Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

Today I'm going to explain a feature of Rust that doesn't get a lot of attention.

Rust is cheap.

---

# Good Fast Cheap

## Pick Three

notes:

I hope that in my Rust series so far I've explained that Rust is fast. About as fast as C and C++.
I've also talked about Rust's great features that you can't find together in other popular languages.

Some, like lifetimes, ownership and borrowing, you may have never heard of before Rust. I certainly hadn't. Others like the unsafe system, macros, and algebraic data types, are available individually in other languages, in Swift, Lisp, and Haskell respectively. But not all together.

Cheap is an interesting metric however. We don't always think of the total cost of our technology choices.

I don't just mean computer cost, but developer, support and planning costs too.

let's start simply with the computer costs.

---

![[Pasted image 20221003210549.png]]

Energy Efficiency across Programming Languages: How Do Energy, Time, and Memory Relate? (2017)

&mdash; Pereira et al.

notes:

You may have seen this screenshot on twitter, or perhaps read the 2017 paper.

In the paper, the authors postulate that the languages that benchmark fastest and use least RAM use less electricity and therefore less CO2.

They say that it's not just about speed. But looking at their results, it is basically about speed.

CPU efficiency appears to be highly correlated with RAM efficiency.

Anyway, Rust puts in an extraordinary showing.

---

![[programming-lang-efficiency-table-full.png]]

notes:

Here's the top half of their results.

Ruby, Python, and Perl are dead last, using around 70x the energy of Rust.

It's a compelling thesis:
If energy consumption means CO2 we might be building our infrastructure using expensive, wasteful, polluting techniques.

And that doesn't spark joy.

Don't take the actual numbers presented in this paper too seriously, benchmarks often differ wildly. But I think the authors were on the right track. The numbers certainly are indicative that some languages are more energy efficient than others.

And with the price of electricity only increasing, that means costs too.

Synthetic benchmarks can only take us so far. Let's look at practical speed.

---

## AWS Lambda Comparison

```md
# Rust
Init Duration: 33.60 ms 
Billed Duration: 393 ms Max Memory Used: 31 MB  
Billed Duration: 51 ms  Max Memory Used: 31 MB  
```

```md
# Node.js
Init Duration: 236.67 ms
Billed Duration: 916 ms Max Memory Used: 81 MB  
Billed Duration: 268 ms Max Memory Used: 81 MB
```

[@aviillouz](https://dev.to/aviillouz/writing-a-lambda-with-rust-using-aws-lambda-rust-runtime-and-aws-sdk-rust-1aln)

notes:

This test by @aviillouz (/avi e-louz/) is small but demonstrable of the efficiencies gained with Rust.

As you can see, in this test
The lambda initialisation duration was 7x faster
hot boot was 5x faster
cold boot was twice as fast

All while using 2.5x less memory.

---

## ARM Lambdas Cost 80% of x64

| x64 price/ms   | ARM price/ms   |
| -------------- | -------------- |
| \$0.0000000021 | \$0.0000000017 |

https://aws.amazon.com/lambda/pricing/

(eu-west-2, price for 128MB)

notes:

And remember why we're talking about this.
On lambda, you are charged by execution time as well as per request.
It's worth noting that Amazon pass on the power efficiencies of the ARM architecture to you, as a cost saving over x64.
Rust of course has support for this and many other architectures as build targets, and cross compilation from x64 machines is trivial.

So far, so good.
But what about the most important cost: The human cost of writing rust?

---

# What Do Developers _DO?_

notes:

Honestly this is a great question that at certain dark periods of the day I often ask myself.

---

- Design code
- Write code
- Read code
- Test code
- Maintain code
- Support their code

notes:

I think we can break it down into these sections.
Let's talk about Design first.

---

# Design

notes:

All modern coding is built on components communicating with each other.
Even something as simple as a text editor can't run everything in a single thread.
We expect syntax highlighting, background linting and building, and some, like neovide, even run the whole UI on the GPU.

It's even more clear with web apps, web service, microservice clusters or similar.

To design a system is to orchestrate a cohesive set of loosely-coupled processes, running on one or more machines.

Let's start small.

---

# Build Structure

```rust
struct Response {
	body: String,
	code: Codes,
}
enum Codes {
	OK,
	NotFound,
	BadRequest(String),
	ServerError(String)
}
```

notes:

When writing Rust programs, the power of the type system encourages you to put as much of your business logic into types as possible.

This is how I recommend you write Rust:
Start by modelling the valid states of your system.

If you are doing your job right, you will make invalid states unrepresentable in your code.

for example, If there are only 4 valid response codes for your system, model them in an enum with 4 variants.
If the body of your Response can only be valid UTF-8, model that with a String, which in Rust is guaranteed at compile time to contain only valid utf-8 code points.

If you design your system based on valid states, be that a desktop application or a web api or a game, the compiler will make invalid states unrepresentable, and will not compile if you make a logical mistake.

---

# Write

notes:

Writing code is what most people think we do, isn't it.

Sometimes we even tell ourselves that is all we do.

The act of writing in many languages is a lonely one.
You write some code, you run the program, you click or type through to the part you're testing, and it crashes.
You read the exception or log and try to guess what has gone wrong.

Sometimes, it's obvious.
Often, it's not.

You then repeat this all day.

Using a language like Rust, or Haskell, or to a lesser extent, Typescript. Allows more problems to be picked up at compile time, rather than by testing, at runtime.
That's not to say you shouldn't test your code in Rust, but that the dumb test cases have been proved for you, by the compiler.

To demonstrate this, let's look at some actual code that runs on AWS lambda

---

%%

```rust
use lambda_http::{service_fn, Error, IntoResponse, Request, RequestExt};

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_http::run(service_fn(hi)).await?;
    Ok(())
}
```

%%

---

## Lambda Http Hander

```toml
lambda_http = "0.6.1"
```

```rust
async fn hi(req: Request) -> Result<impl IntoResponse> {
    let params = req.query_string_parameters();
    
	let name = params
		.first("name")
		.unwrap_or("stranger");
		
    Ok(format!("hello {}", name))
}
```

notes:

Here is an http hello world lambda handler in Rust using the official aws sdk, which contains all functions and types for interacting with the aws cloud.

This compiles, the comipler throws up no errors, which means we can be confident about a lot of things without any extra testing.

---

```rust[2-3]
async fn hi2(_: Request) -> Result<impl IntoResponse> {
    Ok("hello there")
} // OK
```

```rust[2-3]
async fn hi3(_: Request) -> Result<impl IntoResponse> {
    Ok(vec![1,2,3])
} // OK
```

```rust[2-3]
async fn hi4(_: Request) -> Result<impl IntoResponse> {
    Ok(true)
} // ERROR!
```

notes:

Let's see what we can return from this lambda handler.
- Strings, great.
- Vectors, nice.
- Ah, not booleans.

We don't have to guess what is supported the beautiful rust error message tells us:

---

```js
async fn hi4(_: Request) -> Result<impl IntoResponse> {
 _____________________________________________________^
|     Ok(true)
| }
|_^ the trait `IntoResponse`is not implemented for `bool`
```

```js
the following other types implement trait `IntoResponse`:
	  &[u8]
	  &str
	  lambda_http::Response<B>
	  serde_json::value::Value
	  std::string::String
	  std::vec::Vec<u8>
```

(error edited)

notes:

I've split the error up into two sections here.
The first section is the rust compiler telling us what is wrong.
Though the error talks about traits, because that's the language of the compiler, we know what traits MEAN and we should think of them in terms of their MEANING.

The error is that a bool isn't a valid response from aws lambda. The interface of this foreign system, far away in the cloud, is modelled using the type system right here on our machine.

I didn't have to test this in a simulation of a lambda on my machine to find this, the type system is so rich it provides this feedback immediately
Now this error is simple, but the aws sdk is so comprehensive that it stops you from making interface mistakes.

The second error section is just an encore.
Because the compiler knows about the trait, it can give us a hint about what types implement this trait.
Again, that is the language of the compiler.
We should think of this error as telling us what the valid responses are from an aws lambda.

Strings, any serialisable json value, thanks to serde, and some lists.

All makes sense.
And we didn't have to run a single line of code to know about it.

Writing Rust, then, is like having a conversation with the compiler.
One where it tells you where you've made a mistake, and suggests fixes, as we've seen earlier.

Because you have modelled your valid states, the compiler knows and can suggest correct states to move between in your functions.

It's nothing short of a superpower that makes me feel like a genius.

And it's all powered by the same syntax the developer reads.

---

# Read

notes:

Rust's got a lot of syntax.

This is a turn-off for people coming from dynamic languages where the compiler can't help you much at all, so the goal in these languages is to be as light on syntax as possible, so you can iterate and test and move fast and break things.

Rust takes the opposite approach.

---

# Code is Read Far More Often Than it is Written

notes:

You will be aware I'm sure of the idea that code is read far more often than it is written. That's why we often choose more readable languages and styles over something that executes faster, but is less readable. Python vs C, for example.

---

```python
def add_user(connection, username, password):
	connection.user.add(username, password)
```

a minimal python example

notes:

Reading code includes testing in a repl or shell or debugger. Skimming code to find what you want, for instance.
As developers, this is what we spend most of our time doing.
What would this code look like if we optimised it EVEN MORE for reading? Reading by a human, I mean.

---

```python
def add_user(connection, username, password):
	# connection must be open or this line won't work
	connection.user.add(username, password)
```

notes:

Maybe you'd add comments that say "this is only run once" or "connection must be open or this line won't work".

You could give the next programmer (or you, next week) all the context they need to get back in the zone.

---

```python
def add_user(connection, username, password):
	#DBOPEN
	connection.user.add(username, password)
```

notes:
And perhaps you'd formalise this with your colleagues, into standard comments. #TODO #FIXME #RUNTIME #DBOPEN etc.

---

```python[]
def add_user(connection, username, password):
	#DBOPEN
	connection.user.add(username, password)

def __main__():
	conn = connection(ENV['DATABASE'])
	add_user(conn, "Tris", "secret")
	conn.close()
	add_user(conn, "Lucy", "verysecret")
```

```shell
$ cool_team_linter main.py
linting...
ERROR ON LINE 9:
	DBOPEN FUNCTION USED AFTER CONNECTION CLOSED
```

notes:
Maybe even write a linter to make sure they're all in the same format and used in the right place?

You can see where I'm going with this, so I won't continue the hypothetical.

Rust has HUGE amounts of syntax, but it's not boilerplate (trust me on that!) it MEANS something.

lifetime annotations are an example of this superpower, if your language has lifetimes, you can annotate not just WHAT the data is (a string or int etc) but WHEN.

---

# Testing

(benchmarks from

https://github.com/kostya/benchmarks)

notes:

Testing is cheap in Rust, because there's fewer tests to write and those you do are lightning fast.

Unit tests are a real win for Rust's speedups, because they often are pure cpu, with very little integration with external services.
By design, you're supposed to mock those out.

Rust unit tests are 80x faster than python, 20x faster than ruby, 5x faster than Javascript, if you single thread them.

Rust's fearless concurrency extends to tests, which means that usually you can also run your tests concurrently without modification, meaning hundreds of times faster speedups are to be expected.

Rust's speed is a feature that touches everything about the language.

---

# Maintain

notes:
Maintenance is expensive, because it harms our ability to get new work done.

As I mentioned in my previous video _Stop Writing Rust_, most languages make it easy get started on a project, but Rust makes it easy to finish, and move on to then next project.

---

![[xkcd-python.png]]

notes:

Look how simple a hello world is in Python, everyone says!

As a python developer of 15 years, I can confidently say that at scale, python sacrifices too much on the alter of simplicity.

Note that Randell wrote this xkcd before python 3, which means that as you see python had a print statement not a print function.

Python 3 got rid of this print statement because though it was simpler to start with, it caused headaches later on.

While there is value in simple beginnings for a teaching language, certainly I found Python a welcome breath of fresh air from Java, once you've learned how to write complex distributed systems, you don't wish the syntax was simpler.

---

typescript

```ts
interface User {
  name: string;
  id: number;
}
const user: User = {
  name: "Hayes",
  id: 0,
};
```

python

```python
def fib(n: int) -> Iterator[int]:
    a, b = 0, 1
    while a < n:
        yield a
        a, b = b, a+b
```

notes:

In fact the opposite is true.
The larger and more complex your codebase is the more guarantees you want to build-in to your compilers your linter, and your tests.

Everyone loves typescript (almost as much as they love rust) and with good reason. It has some of the syntax that javascript is missing that developers are crying out for.
and the python world is slowly embracing static typing, since type annotations were added in Python 3.

---

https://rocket.rs

```rust
#[derive(FromForm)]
struct Task<'r> {
   #[field(validate = len(1..))]
   description: &'r str,
   completed: bool
}

#[post("/", data = "<task>")]
fn new(task: Form<Task<'_>>) -> Flash<Redirect> {
    Flash::success(Redirect::to("/"), "Added.")
}
```

```toml
rocket = "0.5.0-rc.2"
```

notes:

Rust has a very steep learning curve.

The reason for this is that the compiler is forcing you to fix all your future bugs TODAY before it will compile.

Bugs that don't feature in this web service here include:
- Memory unsafety
- Violations of your business model
- Incorrect sharing of variables in concurrent code
- Misusing futures or channels, and
- Ignoring errors, just to get things working quickly

In short, nearly all the mistakes humans make in production code in most languages.

Coding in Rust, even for experienced developers feels like you are constantly being corrected by the compiler.
I love this.
And you will too, once you try it.

Instant compiler feedback on day 1, is much better than finding out you've called a method on a null at 4am 6 months later and the production system is down.

You very reasonably could expect to have dramatically lower maintenance costs with Rust projects because of this strictness.

You can be sure of the infrastructure you build with Rust.

But how did this come about? What causes this reliability?

---

## The Rust Community Cares About Correctness

notes:

I think it comes from the community.

And it all started off, 16 years ago, with memory safety.

As you must know, Graydon Hoare, while working for mozilla wanted to solve the problem that there were so many memory bugs in Firefox that it was harming feature writing and the velocity of the team.

As often happens, tech debt and bugs drown feature writing, and you miss every deadline.

So Graydon wrote Rust
You can't trust developers to do the right thing, but you can trust a compiler.

From these humble beginnings a community formed.
Not a community that was impressed by quick hacks.
Or a community that was impressed by moving fast and breaking things.
But a community that cares about correctness.

They then built everything else with that same attitude.

---

# Support

notes:

Supporting a complex software stack is difficult.

Many moving parts written in multiple different systems require a great deal of experience or training to support.

And here is another place where Rust is cheap.

---

# Rewrite it in Rust

notes:

There's a meme you'll be aware of, the 'rewrite it in rust' meme.

---

![[rir.png|400]]

notes:

To be fair, I enjoy it a great deal.

But This meme has it's beginnings in concrete features

---

# Your Whole Stack in One Language

notes:

The reason this idea is so pervasive is that it's extremely easy to write any level of your program, whatever the domain, in Rust.
So what ever part of your system is not working well, you can rewrite it.

If you've come from high-level web languages, you can now write custom kernel code.

If you've come from low-level C, you can now write webgl animations.

All in the same language.

All at industry-leading speeds of development and execution.

---

|             |                   |
| ----------- | ----------------- |
| Yew.rs      | Wasm frontend     |
| Rocket.rs   | Backend API       |
| Embassy.dev | Embedded hardware |
| Tauri       | Mobile app        |
| RusTLS      | Native OpenSSL    |
| Rust Macros | Build tooling     |

(other frameworks are available)

notes:

Imagine we work at a company building something like apple airtags.

We have a centralised web service, and embedded chips in our physical hardware devices that we sell to consumers.

Our web frontend is written in yew, a webassembly framework that is faster than react.
Our backend api is written in Rocket, a easy to use framework like express, sinatra, or flask, but faster and safer than them all.
Our chips are flashed with our bare-metal rust app, written in Embassy, a low-level framework.
Our users manage their devices with a tauri-powered rust app on their phones.
And all these components communicate over the internet not with openssl, but with the Rust replacement, RusTLS, and are built with no extra moving parts by Rust macros and the cargo build system.

Though in all likelihood, you'd probably just write a piece of your infrastructure in Rust, there's nothing stopping you from oxidising your entire stack.

And if you were to do so, your code could be-

---

# Good Fast Cheap

notes:

Correct and Fast and CHEAP.

---

![[rust-logo.png]]

# Speed is a FEATURE

notes:

# OUTTRO

In the top video, I used Rust to make a fun retro computer visualisation for my hopepunk podcast, Lost Terminal.

If urban fantasy is more your bag, click the bottom video to listen to a strange and beautiful podcast I produce called Modem Prometheus.

Transcripts and compile-checked markdown sourcecode are available on github, links in the description, and corrections are in the pinned ERRATA comment.

Thank you so much for watching, talk to you on Discord.

```rust
  println!("That's all folks!");
} 
```
