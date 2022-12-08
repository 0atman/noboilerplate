%%
<style>
:root {--r-code-font: "FiraCode Nerd Font";}
</style>

# Cargo.toml

```toml
[package]
name = "rust-is-easy"
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
#![allow(clippy::useless_format)]
#![allow(unused_must_use)]
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
	println!("Rust talk");
```

%%

![[rust-logo.png]]

# RUST Is Easy

### The COMPILER Teaches You

notes:

Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

Rust is one of the most exciting languages of recent years, only hampered by it's reputation as a complicated systems language.

I've got good news for you.

It's not a systems language.

And it ain't complicated.

%%
In today's video I hope to show that Rust, at it's core, is a very simple language.

It's not, it's more complex than other high-level languages you probably are familiar with, python, javascript, ruby, etc. And that complexity does require more syntax.

But you don't have to use it all at once.

Let's start off with numbers.
%%

---

# How to Learn Rust

notes:

The rust compiler is the most advanced, friendly compiler you will have ever used.

It only requires that you read its clear, colourful output.

This is surprisingly challenging: We developers have been trained over our careers, to not read very much into errors.

Stack traces often have single-line errors.

---

### `JSONDecodingError On Line 1`

### `NullPointerException`

### `IndexError In File <in>`

notes:

Unhelpful, cryptic errors that force you to playback the whole application state using your mind's compiler.

A glance at the first line of the error is often as good as reading the whole useless stack trace.

So we stop reading. And start hacking.

I want you to unlearn this bad habit.

---

# Read 
# the Error 
# Again

notes:

When I teach folks Rust, the most common thing I say is 'read the error again'

This is because the errors are so helpful that you don't actually need me to teach you.

You just need to read the error again, and the compiler will teach you itself.

Let's start with a preposterous example:

---

## Let's Write Some ~~rust~~ Javascript

```rust[]
function hello(name) {
    return "hello " + name
}
```

notes:

This first example assumes you know no rust, and have instead typed JAVASCRIPT into your editor.

Don't laugh, we all have to start somewhere.

As you can see, we are trying to write a function with a single parameter that returns that parameter concatenated with a string.

A typical hello world function.

Obviously this does not compile. There is an error.

But something magical has happened:

---

```sql
1  error: expected one of `!`, `.`, `::`, `;`, `?`,
or an operator, found `hello`
 --> src/main.rs:1:10  
  |  
1 | function hello(name) {  
  | -------- ^^^^^ expected one of 8 possible tokens  
  | | help: 
write `fn` instead of `function` to declare a function
```

notes:

The Rust compiler, written by the kind and thoughtful core developers, is offering us help, even though we've given it some outrageous input.

The error tells us to "write `fn` instead of `function` to declare a function"

---

```python
def hello():
    return "lol"
```

```sql
 --> src/main.rs:7:5  
  |  
7 | def hello():  
  | --- ^^^^^ expected one of 8 possible tokens  
  | |  
  | write `fn` instead of `def` to declare a function
```

notes:

By the way, in case you're wondering if I've cherry-picked my example, if you instead give it python, it works the same way.

---

```rust[]
fn hello(name) {
    return "hello " + name
}
```

notes:

OK back to our javascript, but we've done as we were told, and replaced the word `function` with `fn`.

We are then treated to:

---

```sql[]
1  error: expected one of `:`, `@`, or `|`, found `)`  
7 | fn hello(name) {  
  |              ^ expected one of `:`, `@`, or `|`  
  |  
if this is a `self` type, give it a parameter name  
  |  
7 | fn hello(self: name) {  
  |          +++++  
if this is a parameter name, give it a type  
  |  
7 | fn hello(name: TypeName) {  
  |              ++++++++++  
if this is a type, explicitly ignore the parameter 
  |  
7 | fn hello(_: name) {  
  |          ++
```

notes:

This error.

Do not be afraid of this wall of text.

Do not google the first line "expected one of : @ or |".

Do not continue hacking your source code to make the error go away.

The answers are all here if you only read them.

Let's do so together.

---

```sql[1-4]
1  error: expected one of `:`, `@`, or `|`, found `)`  
7 | fn hello(name) {  
  |              ^ expected one of `:`, `@`, or `|`  
  |  
if this is a `self` type, give it a parameter name  
  |  
7 | fn hello(self: name) {  
  |          +++++  
if this is a parameter name, give it a type  
  |  
7 | fn hello(name: TypeName) {  
  |              ++++++++++  
if this is a type, explicitly ignore the parameter 
  |  
7 | fn hello(_: name) {  
  |          ++
```

notes:

The top of the output gives us the syntax error.

We have closed the paren at the end of the function definition without doing something Rust wants.

It was expecting us to type : @ or | before the end of the closing paren.

It might be tempting to guess how to fix the error at this point, but read on.

---

```sql[5-8]
1  error: expected one of `:`, `@`, or `|`, found `)`  
7 | fn hello(name) {  
  |              ^ expected one of `:`, `@`, or `|`  
  |  
if this is a `self` type, give it a parameter name  
  |  
7 | fn hello(self: name) {  
  |          +++++  
if this is a parameter name, give it a type  
  |  
7 | fn hello(name: TypeName) {  
  |              ++++++++++  
if this is a type, explicitly ignore the parameter 
  |  
7 | fn hello(_: name) {  
  |          ++
```

notes:

We now have some help text, the first of which is trying to solve our problem with an example.

If this is a self type, like you might have on a method, give it a parameter name, it says.

It doesn't seem likely that we ARE writing a method, so let us read on.

---

```sql[9-12]
1  error: expected one of `:`, `@`, or `|`, found `)`  
7 | fn hello(name) {  
  |              ^ expected one of `:`, `@`, or `|`  
  |  
if this is a `self` type, give it a parameter name  
  |  
7 | fn hello(self: name) {  
  |          +++++  
if this is a parameter name, give it a type  
  |  
7 | fn hello(name: TypeName) {  
  |              ++++++++++  
if this is a type, explicitly ignore the parameter 
  |  
7 | fn hello(_: name) {  
  |          ++
```

notes:

Now we're getting somewhere.

The help text says "if this is a parameter name, give it a type"

This is exactly what we were trying to do, make a function with a parameter!

So we must give it a type, and the compiler provided an example of what that looks like. We put the type name after a colon.

Terrific!

---

```sql[13-16]
1  error: expected one of `:`, `@`, or `|`, found `)`  
7 | fn hello(name) {  
  |              ^ expected one of `:`, `@`, or `|`  
  |  
if this is a `self` type, give it a parameter name  
  |  
7 | fn hello(self: name) {  
  |          +++++  
if this is a parameter name, give it a type  
  |  
7 | fn hello(name: TypeName) {  
  |              ++++++++++  
if this is a type, explicitly ignore the parameter 
  |  
7 | fn hello(_: name) {  
  |          ++
```

notes:

For completeness, we'll just check the final help example.

I don't think we want to explicitly ignore the param as it suggests.

OK onward.

---

```sql
if this is a parameter name, give it a type  
  |  
7 | fn hello(name: TypeName) {  
  |              ++++++++++  
```

notes:

SO! We're back to this addition.

We must tell the compiler what kind of data the 'name' parameter is.

The reason we do this, and the reason Rust has much more syntax than javascript, is that the more you tell the compiler, the more it can help you, to the point of giving you superpowers.

See my other videos for examples of these.

---

```rust[1]
fn hello(name: string) {
    return "hello " + name
}
```

notes:

So, adding a type to our param.

Let's put a string in there, seems reasonable.

But woops! again, we have guessed wrong, but AGAIN the compiler tells us exactly what to do.

---

```sql
1  error[E0412]: cannot find type `string` in this scope  
   --> src/main.rs:7:16  
    |  
7   | fn hello(name: string) {  
    |                ^^^^^^ help: a struct with a similar 
      name exists (notice the capitalization): `String`  
```

notes:

AH, so structs in Rust are capitalised, like Classes in other languages, ok, fair enough.

Let's fix that:

---

```rust[]
fn hello(name: String) {
    return "hello " + name
}
```

notes:

now we've got a valid parameter type.

The rust error moves on to focus on the body of the function.

---

```sql[]
1  error[E0369]: cannot add `String` to `&str`
 --> src/main.rs:2:20  
  |  
2 |    return "hello " + name  
  |           -------- ^ ---- std::string::String
  |           |        |  
  |           |       `+` cannot be used to concatenate
              |       a `&str` with a `String`
  |           &str  
  |  
help: create an owned `String` on the left
      and add a borrow on the right  
  |  
2 |     return "hello ".to_owned() + &name  
  |                   +++++++++++   +
```

notes:

Another wall of text, but it's so good.

Let's read this error together with all it's retro asci graphics.

---

```sql[1]
1  error[E0369]: cannot add `String` to `&str`
 --> src/main.rs:2:20  
  |  
2 |    return "hello " + name  
  |           -------- ^ ---- std::string::String
  |           |        |  
  |           |       `+` cannot be used to concatenate
              |       a `&str` with a `String`
  |           &str  
  |  
help: create an owned `String` on the left
      and add a borrow on the right  
  |  
2 |     return "hello ".to_owned() + &name  
  |                   +++++++++++   +
```

notes:

The error is "cannot add `String` to a string slice" that's how &str is pronounced.

You will very often come across this problem with two strings in Rust, but it's so easily worked around when you start out.

You don't need to know the difference to continue working on your function, I'm not even going to tell you it, because, as we'll see, Rust gives you the exact answer in the help text.

But let's dutifully walk through the error, as you would do if you were looking at this with fresh eyes, trying to grok it.

---

```sql[4,9]
1  error[E0369]: cannot add `String` to `&str`
 --> src/main.rs:2:20  
  |  
2 |    return "hello " + name  
  |           -------- ^ ---- std::string::String
  |           |        |  
  |           |       `+` cannot be used to concatenate
              |       a `&str` with a `String`
  |           &str  
  |  
help: create an owned `String` on the left
      and add a borrow on the right  
  |  
2 |     return "hello ".to_owned() + &name  
  |                   +++++++++++   +
```

notes:

Rust produces ascii diagrams to help point out what it's talking about, which in heavily-nested code can be a life saver!

Here, it's drawn a line between "hello" and it's label, which is

a &str.

It's telling us that this "hello" string is of type string slice, which is perhaps not quite what we expected.

We will fix it in a moment. Let's continue walking the error.

---

```sql[7,8]
1  error[E0369]: cannot add `String` to `&str`
 --> src/main.rs:2:20  
  |  
2 |    return "hello " + name  
  |           -------- ^ ---- std::string::String
  |           |        |  
  |           |       `+` cannot be used to concatenate
              |       a `&str` with a `String`
  |           &str  
  |  
help: create an owned `String` on the left
      and add a borrow on the right  
  |  
2 |     return "hello ".to_owned() + &name  
  |                   +++++++++++   +
```

notes:

We are warned here, on lines 7 and 8 that the + symbol can't be used to concatenate a string slice and a string.

Fair enough, they are different types, different types often can't be simply concatenated by using plus.

---

```sql[4,5]
1  error[E0369]: cannot add `String` to `&str`
 --> src/main.rs:2:20  
  |  
2 |    return "hello " + name  
  |           -------- ^ ---- std::string::String
  |           |        |  
  |           |       `+` cannot be used to concatenate
              |       a `&str` with a `String`
  |           &str  
  |  
help: create an owned `String` on the left
      and add a borrow on the right  
  |  
2 |     return "hello ".to_owned() + &name  
  |                   +++++++++++   +
```

notes:

At the end of line 5, the compiler is telling us that `name` is of type String, capital S String, as we set in the function signature.

Now that we've decomposed this ascii diagram explaining the pieces of the puzzle and what the problem is, let's focus on the compiler's proposed solution:

---

```sql[11-15]
1  error[E0369]: cannot add `String` to `&str`
 --> src/main.rs:2:20  
  |  
2 |    return "hello " + name  
  |           -------- ^ ---- std::string::String
  |           |        |  
  |           |       `+` cannot be used to concatenate
              |       a `&str` with a `String`
  |           &str  
  |  
help: create an owned `String` on the left
      and add a borrow on the right  
  |  
2 |     return "hello ".to_owned() + &name  
  |                   +++++++++++   +
```

notes:

We are told the exact code to solve this common beginner problem.

Turn that literal string slice into a proper String, with to_owned(), and borrow the `name` variable with &.

You do not yet need to know exactly what the explanation means to keep using Rust, and to use Rust's superpowers, you merely need to do what the compiler tells you to do.

We will do just that.

---

```rust[]
fn hello(name: String) {
    return "hello ".to_owned() + &name
}
```

notes:

here We have dutifully added the to_owned() call, converting the string slice into a proper String, and borrowed the name variable.

We are on the home stretch, there is just one more correction, and if you have used a statically-typed language before, you may have guessed it already:

---

```sql
1  error[E0308]: mismatched types  
 --> src/main.rs:8:12  
  |  
7 | fn hello(name: String) {  
8 |     return "hello ".to_owned() + &name  
  |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`,
                                    found struct `String`

help: try adding a return type: `-> String`

For more information about this error, try
`rustc --explain E0308`.  
```

notes:
the problem is that Rust expected an empty return value, which in rust is the empty tuple, pronounced "unit", but it found a String.

The language of the compiler is that it expects the return type of the function signature.
And what you actually return is what it finds.

Expects and finds.

in this case, we have not specified any return type, so it expects nothing, but it found a String.

The solution, as in every example today, is told to you by the help text of the compiler.

Try adding a return type, String, it says.

Before we do that, it's worth noting the last two lines of the error.

Most rust errors have this link to longer explanations and examples and counterexamples of errors.

Let's try running `rustc --explain E0308`

---

##### Expected Type Did Not Match the Received Type

```rust[]
fn plus_one(x: i32) -> i32 {  
   x + 1  
}  
  
plus_one("Not a number");  
//       ^^^^^^^^^^^^^^ expected `i32`, found `&str`
  
if "Not a bool" {  
// ^^^^^^^^^^^^ expected `bool`, found `&str`
}  
  
let x: f32 = "Not a float";  
//     ---   ^^^^^^^^^^^^^ expected `f32`, found `&str`
//     |  
//     expected due to this  
```  

notes:

This is the output, small toy examples of other mistakes that can lead to this error, along with a longer explanation of the problem, which I will read to you now.

> This error (the output states) occurs when an expression was used in a place where the compiler expected an expression of a different type. It can occur in several cases, the most common being when calling a function and passing an argument which has a different type than the matching type in the function declaration.

Exactly what we did.

Let's fix our function by adding the missing String return type.

---

```rust
fn hello(name: String) -> String {
    return "hello ".to_owned() + &name
}
```

```sh
$ cargo build
Finished dev [unoptimized + debuginfo] target(s) in 0.01s
```


notes:

The compiler is now happy, and if the compiler is happy, I am happy.

We have satisfied the compiler, which means many things are guaranteed about this simple function that are not guaranteed in other languages:
- This code has no memory leaks, is thread-safe, and has no scaling surprises.
- Memory is automatically freed when it goes out of scope, you don't have to stop execution to do garbage collection
- There are no execution paths that cause errors in the function, you can tell this by the signature. If a Rust function can error, it returns a Result type. OUR function always returns a pure string, which means it is infallible, it will ALWAYS work because the compiler guarantees that the calling code is providing a valid UTF-8 string. (panics are still possible, though rare)

Not only that but these guarantees hold through concurrent code, through threads, locks, channels, high-performance web apis, and bare-metal coding.

---

```sh
cargo run                 # run your code in debug mode
cargo doc                 # local package documentation  
cargo bench               # built-in benchmarking
cargo test                # built-in parallel testing
cargo add aws-sdk         # easily add dependencies
cargo install cargo-watch # install exes into .cargo/bin
cargo watch               # extend cargo and use these
cargo publish             # publish packages to crates.io
cargo build --release     # build release binaries
```

notes:

Rust is the easiest language I have ever used because everything is right at hand.

You don't need a teacher, the compiler is right on your machine

The tooling is almost as incredible as the language.

Not just incredible compared with C or C++, but compared with ANY language.


---

![[rustlings.png]]

https://github.com/rust-lang/rustlings

notes:

To take this iterative approach to learning the whole of the Rust language, try Rustlings, a simple code kata system where the compiler and unit tests teach you.

---

# <https://Rustup.rs>

`curl https://sh.rustup.rs | sh`

notes:

So what are you waiting for?

Visit rustup.rs right now!

The installer is a one-liner on linux, WSL, and macos, and an graphical installer is available for windows.

Give Rust a try, have fun, and don't forget to Read the error again.

---

![[rust-logo.png]]

# The COMPILER Teaches You

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