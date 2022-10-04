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

# Lint tweaks
```rust
#![allow(dead_code)]
#![allow(unused_variables)]
```

# extern crates

```rust

```

# imports
```rust
use anyhow::Result;
```

# setup

```rust
fn main() {
	println!("Rust Makes Cents");

```
%%

![[rust-logo.png]]

# RUST Makes Cents
### Speed is a FEATURE

notes:

Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

Today I'm going to explain a feature of Rust that doesn't get a lot of attention.

Rust is cheap.

---

# good fast cheap
# pick three

notes:

I hope that in my Rust series so far I've explained that Rust is fast. About as fast C, often faster than C++.

I've also talked about Rust's great features that you can't find together in other popular languages.

Some, like lifetimes, ownership and borrowing, you may have never heard of before Rust. Others like the unsafe system, macros, and algebraic types, are available individually in other languages, as Swift, Lisp, and Haskell respectively. 

Cheap is an interesting metric. We don't always think of the total cost of our technology choices.

I don't just mean computer cost, but developer, support and planning costs.

But let's start with the computer costs.


---

![[Pasted image 20221003210549.png]]

Energy Efficiency across Programming Languages: How Do Energy, Time, and Memory Relate? (2017)

&mdash; Pereira et al.

notes:

You may have seen this screenshot on twitter, or read the 2017 paper.

In the paper, the authors postulate that the languages that benchmark fastest and use least RAM then use least electricity and therefore less CO2.

They say that it's not just about speed. But looking at their results, it's basically about speed.

CPU efficiency appears to be highly correlated with RAM efficiency.

Anyway, Rust puts in an extraordinary showing.


---

![[Pasted image 20221003213309.png]]

notes:

Here's the top half of the results. 

Ruby, Python, and Perl are dead last, using around 70x the energy of Rust.

If energy consumption means CO2 we might be building our infrastructure using expensive, wasteful, polluting techniques.

But what about Developer time, surely that counts for something too?

---

# What do developers _DO?_

notes:

Honestly this is a great question that at certain dark periods of the day I often ask myself.

---

- Design code 
- Write code
- Read code
- Maintain code
- Support their code 

notes:

I think we can break it down into these sections.

---

# Design

notes:

Let's talk about Design first.

All modern coding is built on components communicating with each other.
Even something as simple as a text editor can't run everything in a single thread.
We expect syntax highlighting, background linting and building, and some like neovide, even run the UI on the GPU.

It's even more clear with a web service, microservice cluster or similar.

To design a system is to orchestrate a cohesive set of loosely-coupled processes, running on one or more systems.

Let's start small.

---

```rust
struct Response {
	body: String,
	code: Codes,
	todo!()
}
enum Codes {
	OK,
	NotFound,
	BadRequest(String),
	ServerError(String)
}
```

When writing Rust programs, the power of the type system encourages you to put as much of your business logic into types as possible.

This is how I recommend you write Rust:
Start by modelling the valid states of your system.

If you are doing your job right, you make invalid states unrepresentable.

If there are only 5 valid response codes for your system, model them in an enum with 5 varients.
If the body of your Response can only be valid UTF-8, model that with a String, which in Rust is guarenteed at compile time to contain only valid utf-8 codepoints.
- [ ] utf8?

If you design your system based on valid states, be that a desktop application or a web api or a game, the compiler makes invalid states unrepresentable, and will not compile.

# Write 

notes:

Writing code is what most people think we do, isn't it.

The act of writing in many languages is a lonely one.
You write some code, you run the program, you click or type through to the part you're using, and it crashes.
You read the exception and try to guess what has gone wrong.

Sometimes, it's obvious.
Often, it's not.

Using a language like Rust, or Haskell, or to a much lesser extent, Typescript. Allows more problems to be picked up at compile time, rather than runtime.

As I've said before, we programmers live at compile time.
IE, we look at the same code the compiler sees and we try to reason about it.

Because of Rust's very detailed syntax, which we'll talk about more in the next section, the compiler knows much more about your code than in other popular languages.

Writing Rust, then, is like having a conversation with the compiler.
One where it tells you where you've made a mistake, and suggests fixes.

Because you have modelled your valid states, the compiler knows and can suggest correct states to move between in your functions.

It's nothing short of a superpower that makes me feel like a genius.

And it's all powered by the same syntax the developer reads.

---

# Read 

notes:

Rust's got a lot of syntax.

This is a turn-off for people coming from dynamic languages where the compiler can't help you much at all, so the goal is to be as light on syntax as possible, so you can iterate and test move fast and break things.

Rust takes the opposite approach.

---

## Code is read far more often that it is written

notes:
We all know this truism.

Optimise for readability we know.
Use Self documenting techniques where it makes sense to.
Don't write `n` or `x`, write `username` or `number_of_orders`

- [ ] paste in here the notation comment

---

---

# Maintain

notes:

As I mentioned in my previous video _Stop Writing Rust_, most languages optimise for simpicity of getting started.

---


- [ ] xkcd python
notes:

Look how simple a hello world is in Python, they say!

As a python developer of 15 years, I can confidently say that at scale, python sacrifices so much on the alter of simplicity.

While there is value in simple beginings for a teaching language, certainly I found Python a welcome breath of fresh air from Java, once you've learned how to write complex distributed systems, you don't wish the syntax was simpler.

---

- [ ] mypy demo
- [ ] typescript demo

notes:

In fact the opposite is true.
The larger and more complex your codebase is the more STUFF you want to build in to your build system, your CI tooling, and your tests.

Everyone loves typescript, and with good reason. It has the syntax that javascript is missing that developers are crying out for.

---

notes:

Rust has a very steep learning curve.

The reason for this is that the compiler is forcing you to fix all your future bugs TODAY before it will compile.

Bugs like:
- Memory unsafety
- Violations of your business model
- Incorrect sharing of variables in concurrent code
- Misusing futures
- Ignoring erorrs, just to get things working quickly

Coding in Rust, even for experienced developers feels like you are constantly being corrected by the compiler.
I love this.
And you will too, once you try it.

Instant compiler feedback on day 1, is much better than finding out you've called a method on a null at 4am 6 months later and the production system is down.

You very reasonably could expect to have dramatically lower maintainance with Rust projects because of this feature.

But how did this come about? What feature causes this reliability?



---

# The Rust community cares about correctness

notes:

I think it is the attitude of the community.

And it all started off, 16 years ago, with memory safety.

Graydon Hoare 
- [ ] spelling

working for mozilla wanted to solve the problem that there were so many memory bugs in Firefox that it was harming feature writing.

As microsoft famously learning with 
- [ ] word/excel?
Technical debt and bugs will drown feature writing, and you'll miss every deadline.

So Graydon wrote Rust, a language where RAII is enforced.
You can't trust developers to do the right thing, but you can trust a compiler.

From these humble beginnings a community formed.
Not a community that was impressed by quick hacks.
Or a community that was impressed by moving fast and breaking things.
But a community that cares about correctness.

They then built everything else.

Rust has

---

- Cargo
- Decoupled async
- Macros
- Unsafe
- 


---

# Support

notes:

Let's talk about Design first

---






- good fast cheap, pick three
- TCO of rust
- Cheap on developer time
	- not at first, but over the whole lifetime of the app
	- Fast tests
	- whole category of errors avoided
- The whole stack is one language
	- down at the hardware level in unsafe rust
	- great ergonomics like iter compile down to the same machine code as hand-crafted loops
	- build tools aren't separate, they're macros


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

This test by @aviillouz (/avi e-louse/) is small but demonstrable of the efficiencies gained with Rust.

As you can see, in this test 
The init duration was 7x faster
hot boot was 5x faster
cold boot was 2x faster

All while using 2.5x less memory.




---

## ARM lambdas cost 80% of x64

| x64 price/ms   | ARM price/ms   |
| -------------- | -------------- |
| \$0.0000000021 | \$0.0000000017 |

https://aws.amazon.com/lambda/pricing/

(eu-west-2)

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

## Lambda http hander
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

This compiles, the aws sdk throws up no errors, which means we can be confident about a lot of things.

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
- Ah, not bools.

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
Though the error talks about traits, because that's the language of the compiler, we know what traits MEAN and we should think of them in this way.

The error is that A bool isn't a valid response from aws lambda. The interface of a foreign system, far away in the cloud, is modelled using the type system right here on our machine.

I didn't have to test this ins a simulation of a lambda on my machine to find this, the type system is so rich it provides this feedback immediately, in 1ms.

The second section is just an encore. 
Because the compiler knows about the trait, it can give us a hint about what types implement this trait.
Again, that is the language of the compiler.
We should think of this error as telling us what valid responses are from aws lambda.

Strings, any serialisable json value, thanks to serde, and some lists.

All makes sense.
And we didn't have to run a single line of code to know about it.

This is the productivity gains of using Rust that I want to highlight.


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
