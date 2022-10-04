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

Rust Makes Cents (cost is a feature)" - talking about the TCO of Rust, how it's cheap on CPU, RAM and indeed developer time, and the whole stack can be written in one language


---


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

# Rust is also Cheap on developer time

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
