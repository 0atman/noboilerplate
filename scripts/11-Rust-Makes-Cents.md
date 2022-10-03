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
lambda_http = "0.6.1"
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

https://twitter.com/alexxubyte/status/1569348391167197185?t=J5ApLHw0xOjRBfWUJS1vgA&s=19

- good fast cheap, pick three
- TCO of rust
- cheap on CPU
- cheap on RAM
- Cheap on developer time
	- not at first, but over the whole lifetime of the app
	- Fast tests
	- whole category of errors avoided
- The whole stack is one language
	- down at the hardware level in unsafe rust
	- great ergonomics like iter compile down to the same machine code as hand-crafted loops
	- build tools aren't separate, they're macros


---

# AWS Lambda

https://dev.to/aviillouz/writing-a-lambda-with-rust-using-aws-lambda-rust-runtime-and-aws-sdk-rust-1aln

```md
# Rust
Duration: 358.57 ms Billed Duration: 393 ms Memory Size: 128 MB Max Memory Used: 31 MB  Init Duration: 33.60 ms 
Duration: 39.76 ms  Billed Duration: 40 ms  Memory Size: 128 MB Max Memory Used: 31 MB  
Duration: 52.98 ms  Billed Duration: 53 ms  Memory Size: 128 MB Max Memory Used: 31 MB  
Duration: 49.17 ms  Billed Duration: 50 ms  Memory Size: 128 MB Max Memory Used: 31 MB  
Duration: 50.71 ms  Billed Duration: 51 ms  Memory Size: 128 MB Max Memory Used: 31 MB  

# node.js
Duration: 915.67 ms Billed Duration: 916 ms Memory Size: 128 MB Max Memory Used: 81 MB  Init Duration: 236.67 ms
Duration: 90.40 ms  Billed Duration: 91 ms  Memory Size: 128 MB Max Memory Used: 81 MB
Duration: 331.29 ms Billed Duration: 332 ms Memory Size: 128 MB Max Memory Used: 81 MB
Duration: 320.97 ms Billed Duration: 321 ms Memory Size: 128 MB Max Memory Used: 81 MB
Duration: 267.81 ms Billed Duration: 268 ms Memory Size: 128 MB Max Memory Used: 81 MB
```


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

## Lambda http hander

```rust
async fn hi(req: Request) -> Result<impl IntoResponse> {
    let params = req.query_string_parameters();
    
	let name = params
		.first("name")
		.unwrap_or("stranger");
		
    Ok(format!("hello {}", name))
}
```

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
