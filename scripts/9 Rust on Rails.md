<style>
:root {--r-code-font: "FiraCode Nerd Font";}
</style>

%%

# Cargo.toml 
```toml
[package]
name = "railway"
version = "0.1.0"
edition = "2021"

[build-dependencies]

[dev-dependencies]

[dependencies]
rand = "0.8.5"
rand_derive2 = "0.1.17"
anyhow = "1.0.63"
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
```

# setup



```rust
fn main() {
	println!("Rust on Rails");

```

%%
![[rust-logo.png]]

# RUST on Rails
### Write Code that never crashes

notes:

Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

Today I'm going to tell you how Rust helps us write code that has no execution paths that crash at runtime.

This is a core pillar of Rust's fearless programming.

Let's be real,

---

# Shit happens


notes:

The world is messy and bad code, bad APIs, and bad data are everywhere.

There's no way around errors, they are a fact of life.

The only way we could avoid them is by not interacting with the real world.

But if we write code that doesn't interact with the real world that would mean doing no I/O.

No output to the screen, no input from the keyboard.

The only evidence that our program were running was that the box would get hotter.

So just as in life, worthwhile interaction comes with the possibility of failure.

---


# try / Catch 

notes:

As you know, Exceptions are the common way to manage this.

The first language to handle exceptions in a way we would recognise today was LISP, in 1958

(https://en.wikipedia.org/wiki/Exception_handling#History)

I don't know why I was surprised by this fact when I looked it up, basically LISP created everything we know as high-level programming today, and we're still learning the lessons 50 years later.
A topic for another video, or a previous video, perhaps. Rust is getting very close.
%% pin: Clojure video %%


Nearly every language from the 1980s onwards copied exceptions, they have become standard in nearly every programming language.


---


# But they *suck*

notes:

Exceptions add a separate execution system outside the normal function call flow.

You might not notice this if your language is stuffed full of classes, mixins, prototype trees, templates, and competing fashionable idioms.

If there's already 10 ways to do things, what's adding one more to the mix?

Let me make a clear argument.

---


# GOTOs are bad right? 

notes:

Of course they are, everyone agrees.
Here's the thing:


---

# Gotos are bad
## because they take you out of the normal flow of execution

notes:

Gotos are bad because they take you out of the normal flow of execution

Why do we allow Exceptions, our error handling, to take us out of the normal flow of execution but not allow GOTOs to do so?

Come to think of it, if errors are the inevitable byproduct of interacting with the real world, and programs that do useful work interact with the real world, why are errors called Exceptions? Shouldn't they be expected?



---

# Errors shouldn't be exceptional

notes:

And here we have the central problem with exceptions.

Errors are very normal, and acting like they're exceptional harms our ability to think about them, and write robust complex distributed code.


---

# What is the alternative?

notes:

Luckily, we have a clear example from the world of mathematics.


---

# n ↦ n + 1

notes:

In mathematical functions we work with just numbers and our equations are either valid or they are not.

They might be WRONG but they won't give us a unexpected `NullPointerException` instead of the number 2.

Of course, unlike mathematicians, we live in the real world and must interact with the messy world with all its humans and state and data loss. 


---


```rust
fn get_num_fixed(num: String) -> i32 {
	42 // always the right answer
}
```

notes:

Here's some Rust, finally.

If we're never doing computation, like here, then we don't need Errors.

This function accepts any String, and returns an integer, specifically, the integer 42.

You'll remember that most Rust integers are i32, which is a 32-bit signed integer.
32 bits is enough for most applications, and signed integers have fewer surprises than un-signed.

This is why i32 is the rust default when you don't specify a type, such as in ranges.


---

```rust[4]
use std::num::ParseIntError;

fn parse_num_bad(num: String) -> i32 {
	num.parse()
}
```

_(this does not compile)_

notes:

But what if you want to do something that could fail, such as parsing that string to an integer?

In other languages, this would be fine, because the function signature expresses the happy path, and an exception would break the flow and we could catch it somewhere else.

Rust doesn't have exceptions, so if you write this you are treated to one of rust's excellent compiler errors.

---


```rust[6]
 error[E0308]: mismatched types
 --> src/main.rs:13:2
 |
 | fn parse_num_bad(num: String) -> i32 {
 |     num.parse()
 |     ^^^^^^^^^^^ expected `i32`, found enum `Result`
```

notes:

As usual Rust has told us exactly the problem.

I've lightly edited this error to fit on a slide.

Rust's errors talk about Types, because that is the language of the compiler, but we know what types mean, and should talk about them in terms of their MEANING.

The error is "expected i32, found Result.".

The MEANING is "expected a number, found a fallible number"

Results are Rust's way of saying that you might not have what you wanted, there might be an error.

And because Errors are values, the type system can help us.

`num.parse()` returns a Result, which means we can't get at the number directly, there might have been an error.

Let's tweak the function signature to fix this and pass the result back up.


---

```rust
use std::num::ParseIntError;

fn parse_num(num: String) -> Result<i32, ParseIntError> {
	num.parse()
}
```

notes:

Rust encapsulates this fallibility in the Result type and the compiler can help you
- you can't return a integer or an error, the type has to be one type only - but you can return a Result enum that has two valid states:
- an integer
- or an error 

Now the compiler is happy.
And if the compiler is happy, I am happy.

We have clearly stated in the function signature that you won't get an integer back from this function, you will get it wrapped in a Result and you must handle it.

Treating Errors as just another value is one of the features of Rust that allow you to fearlessly write complex distributed systems confidently.

An example of a company building large-scale complex distributed systems in Rust is today's sponsor Ditto.


---


# ![[scripts/private/ditto_logo_white_text.png]]

notes:

- Unlike other sponsors, ditto don't want your money, they actually want to pay you.
- This is because they've asked me to tell you about open Rust positions at their company.
- Here's what you'll want to know about them, their tech, and their open positions:

---

### Ditto mesh architecture 
![[ditto_mesh_architecture.png|850]]
notes:

- Ditto use rust to power their cross-platform data sync system
- They're growing their team and are looking for people passionate about Rust, if you're watching this video, that might be you.
- The problems they are solving include mesh networking, replication protocols, Conflict-free Replicated Data Types, and database design to name a few.

---
### Ditto rainbow connection
![[ditto_rainbow connection.png]]

notes:


- They're looking for people with
    - demonstrable Rust experience or previous work with C/C++
    - Rust backend developers for their Big Peer cloud system
    - Low-level bare-metal coders working with FFI and C interop
    - Algorithm junkies to work on their data stores, and
    - Networking coders at either the low or high level to work on their actor-like frameworks in replication and multihop work.

---

![[scripts/private/ditto_logo_white_text.png]]

**[www.ditto.live](https://www.ditto.live)** 

_[www.ditto.live/jobs](http://www.ditto.live/jobs)_

notes:

Find out more about ditto at ditto.live, and see their
open positions at ditto.live/jobs.

My thanks to ditto for their support of this channel.

Let's get back on the rails:

---


## Railway-oriented programming

![[rails-1-success-failure.png]]


[fsharpforfunandprofit.com/rop/](https://fsharpforfunandprofit.com/rop/)

&mdash; [@ScottWlaschin](https://twitter.com/ScottWlaschin)

notes:

- You may have heard of this metaphor, It's one I found very useful when trying to understand functional errors like the Result type in Rust. 
- It was created by, and I will be borrowing a few slides from, Scott Wlaschin (pronounced: Vuh-losh-in) in 2014
- Each function is a set of points, and if there is a computation error, you divert the train from the happy path, from which it can never recover.
---

![[rails-2-composition.png]]

notes:


- All future functions the error interacts with, never execute any code, the payload continues uninspected through each station on its way to the destination. only then do we need to find out what is inside with .unwrap(), expect(), or, preferably, one of the many runtime safe methods.

---


# Show me the code

notes:

let's build some error handling in Rust

---

## fallible functions

```rust
#[derive(Debug)]
enum MathError {
	DivisionByZero,
	NonPositiveLogarithm,
	NegativeSquareRoot,
}

```

notes:


I am basing this section on the code in the book Rust By Example
https://doc.rust-lang.org/rust-by-example/std/result.html

As ever we start with modelling our data.

We are going to impalement a `check_math()` function, to avoid panicking at runtime.

Here we're enumerating the three kinds of mathematics errors we are going to be catching in our toy example.

---

```rust
fn div(x: f64, y: f64) -> Result<f64, MathError> {
	if y == 0.0 {
		Err(MathError::DivisionByZero)
	} else {
		Ok(x / y)
	}
}
```

notes:

And here's how we are going to do that.

Division by zero operations panic at runtime, instead let's return the reason for the failure wrapped in the  `Err` variant of the Result enum.
If the divisor y is not zero then this operation is valid, and we return the result wrapped in `Ok`

Note that we don't return the number, we still wrap it in the OK variant of the Result enum.

This stops calling code using the integer directly, which of course won't work if unexpectedly there is an error.

Predictable return values are essential for good design in any language.

The next time you are tempted to return false, or null from a function to signify a fail state, remember this.

Source: https://doc.rust-lang.org/rust-by-example/std/result.html

---

## Square Root

```rust
fn sqrt(x: f64) -> Result<f64, MathError> {
	if x < 0.0 {
		Err(MathError::NegativeSquareRoot)
	} else {
		Ok(x.sqrt())
	}
}
```

notes:

We'll write two more functions to do similar checked behaviour, first a square root function that errors on a negative number

---


## Natural log

```rust
fn ln(x: f64) -> Result<f64, MathError> {
	if x <= 0.0 {
		Err(MathError::NonPositiveLogarithm)
	} else {
		Ok(x.ln())
	}
}
```

notes:

and the same for natural log.

Now let's put them together and test our train tracks!

---

```rust[]
fn check_math(divis: f64) -> Result<f64, MathError> {
	let answer1 = div(1.0, divis)?;
	let answer2 = sqrt(answer1)?;
	let answer3 = ln(answer2)?;
	Ok(answer3)
}

check_math(1.0).unwrap();  // == 0.0;
check_math(0.0).unwrap();  // panic: DivisionByZero
let err = check_math(0.0); // == Err(DivisionByZero)
```

notes:

Here we have a function that uses our new maths system we have just made.
the `check_math()` function does some division, then square root, then log and returns the answer

Note that the three calculations on lines 2-5 use the question-mark operator to unwrap the number inside the result, or, if it is an error, immediately return it.

This is the railway part of the function.
Though not a track of functions accepting and returning results, as in the original railway oriented design, this is functionally the same.

As soon as you get an error, you early-return it from the function.

The last answer, answer3, on line 5, if we get it, we want to re-package inside the Ok variant of the Result enum, so it can be returned in a type that is compatible with our function signature.

---

# This looks an awful lot like exceptions

notes:

If you have been paying attention, you will note that early-returning from a function with an error sounds and looks an awful lot like exception handling.

That is due to the `?` operator, one of Rust's excellent patterns, which deliberately makes this behaviour familiar.

But the difference is clear.

---

Javascript

```js
function parse_int(text) {
	//todo
}
```

Typescript

```ts
function parse_int(text: string): number {
	//todo
}
```

Rust

%%
```rust
use anyhow::Error;
```
%%

```rust
fn parse_int(text: String) -> Result<i32, Error> {
	todo!()
}
```

notes:

All three of these functions can fail.

But only Rust makes it clear to the compiler, and more importantly to you, that it can, and will force you to handle the failure.

Javascript is worse, as it also could return null instead of the number you hope to get. This is common across Python and Ruby, too.

---

# You can't 
# fix nulls

notes:

This is my core feeling, after programming professionally for 15 years.

Tony Hoare, inventor of Null in ALGOL in 1965, calls it his Billion Dollar Mistake, as nearly every language has adopted it and perpetuated the misery.

The core problem with null is that the type system must make a special case for it.
You might expect a function to return a string, but SURPRISE you got a null. 

---

unwrapping at runtime

```rust
fn bad_idea(something: Option<String>) -> String {
	something.unwrap()
}
```

unsafe indexing at runtime
```rust
fn also_bad(users: Vec<String>) -> String {
	users[99].clone()
}
```

notes:

Rust's design helps you avoid runtime errors.
But if you write code like this, you'll still get them.

Making sure your code doesn't have unwrap()s or expect()s gets you nearly all of the way there, but how can you defend against unchecked vector indexing, like in the second example.

Well you could set up a lint error for this, and clippy can help you there. 

But, friends, I have something very exciting to share with you.

---

```rust[1]
#[no_panic]
fn bad_idea(something: Option<String>) -> String {
	something.unwrap()
}
```

```md
   Compiling no-panic-demo v0.0.1
error: linking with `cc` failed: exit code: 1

		[compiler output removed]

ERROR[no-panic]: detected panic in function `bad_idea`
collect2: error: ld returned 1 exit status
```

[crates.io/crates/no-panic](https://crates.io/crates/no-panic)

notes:

The no-panic macro can give us this automatically.

no-panic is a Rust attribute macro that requires the compiler prove a function can't ever panic.

I don't fully understand how it works, but by the message, I think it causes a compiler error if your code links to the panic handler function in the Rust standard library.

This means if a single branch of your code unwraps a Result unsafely, does unsafe runtime operations, or calls any crates that do so, no-panic won't allow that function to compile.

The github repo is archived, but I've tested it and it works!
If you want to use it to prove important parts of your code, you can do so.

---

# Rust had no nulls from the start

notes:

Most languages start with null and have to build in controls for this terrible feature on top of it.

Though Java now has options, nulls still exist in every popular library, hidden, waiting to strike at 4am.

Typescript attempts to paper over the damage with it's own the Option type, but it's too late for JavaScript too.

These languages along with Python and Ruby infected their standard libraries and ecosystems with nulls years ago and it's impossible to get the horse back in the stable after it has bolted.

Even Go, an otherwise very good modern language, has nulls.

Rust had no nulls from the start.

so you always get what you expect and you can easily 

---

![[rust-logo.png|350]]

# Write Code that never crashes


notes:

# OUTTRO

If you'd like to see what you can write in rust, click the top video: I used it to make a fun retro computer visualisation for my hopepunk podcast, Lost Terminal.

Or if urban fantasy is more your bag, click the bottom video to listen to a strange and beautiful podcast I produce called Modem Prometheus.

If you would like to support my work, head to patreon.com/noboilerplate.
Transcripts and compile-checked markdown sourcecode are available on github, links in the description, and corrections are in the pinned ERRATA comment.

Thank you so much for watching, talk to you on Discord.

```rust
  println!("That's all folks!");
} 
```


