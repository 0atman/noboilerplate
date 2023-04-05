<style>
:root {--r-code-font: "FiraCode Nerd Font";}
.reveal .hljs {min-height: 50%;}
</style>

f7f7f7 background slide colour

# Cargo.toml

```toml
[package]
name = "magic-macros"
version = "0.1.0"
edition = "2021"

[build-dependencies]

[dev-dependencies]

[dependencies]
macro_lisp = "0.1.0"
html-to-string-macro = "0.2.5"
```

# Lint Tweaks

```rust
#![allow(dead_code)]
#![allow(unused_variables)]
```

# Extern Crates

```rust
#[macro_use]
extern crate macro_lisp;
```

# Imports

```rust
// unused due to syntax highlighting issues
//use html_to_string_macro::html;
```

# Setup

```rust
fn main() {
	println!("Rust talk");

```

---

![[rust-logo.png]]

# (It's Macros)

notes:

Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

You and I would not be here today if it weren't for my favourite feature of rust, and indeed of any language: Macros.

I've mentioned them in the past, but today I'm going to explain why they're SO POWERFUL.

---

## Open Source Videos

<https://github.com/0atman/noboilerplate/>

notes:

As ever, all Rust code you see in this video is part of a literate programming document that can be extracted and compiled with native Rust tooling.

---

# We Need to Talk about Macros

notes:

Most articles about macros minimise their most powerful feature, compile-time code execution, and instead focus on their ability to DRY up your code, as if they were just metaprogramming or a way to save keystrokes.

This is because most languages don't have a way to run arbiatry code at compile time.
And those that do, limit this to just syntax rewriting.

If you're used to staring at the shadows of metaprogramming flickering on the wall, you will have no concept of the world available to you outside of the cave.

---

```md[9]
Conditionals. 
A function type. 
Recursion.
Pass-by-reference.
Garbage-collection.
Programs composed of expressions, not statements.
A symbol type.
A notation for code using trees of symbols.
The whole language always available.
```

_"What Made Lisp Different"_

&mdash; Paul Graham

notes:

Let's be clear, proper macros ARE the most powerful metaprogramming technique to decrease boilerplate we have.
No other language feature is as powerful or as flexible.

We've known this for half a century, macros were one of the key innovations in lisp.

Most languages have implemented about half of this list by the way.
Rust has nearly everything including macros.

This 9th point here is macros, the whole language is available at compile time as well as runtime.

That's what a macro is, a function that runs at compile time, with full access to the underlying system, and can rewrite your syntax.

---

```c[]
#include <stdio.h>
 
// Macro definition
#define AREA(l, b) (l * b)
 
int main()
{
    int l1 = 10, l2 = 5, area;
    area = AREA(l1, l2);
 
    printf("Area of rectangle is: %d", area);
    return 0;
}
```

notes:
Other languages have things they call macros, like here with C.

This is simple text manipulation.
It's good for code reuse, but extremely surprising because there's no indication that you're using a macro, the macro invocation on line 9 looks just like the function invocation on line 11.

---

```rust
macro_rules! say_hello {
    () => {
        println!("Hello!")
    };
}


say_hello!(); // replaced with `println!("Hello!")`

```

<https://doc.rust-lang.org/rust-by-example/macros.html>

notes:
Rust DOES indeed have this kind of macro, and they're so common that a very simple definition is possible with macro_rules here.

This is a simple macro named `say_hello`.
It takes no arguments and will expand into the contents of the inner block.

This call will expand into `println!("Hello");` at compile time.
Note that the print statement isn't executed at compile time, it's just being inserted into the code at compile time, ready to be executed at runtime like all functions.

You may note that this looks exactly like a match statement.

macro_rules! are like match statements that are executed at compile time.

Let's look at a real-world example

---

```rust[]
macro_rules! bounded_impl {
    ($t:ty, $min:expr, $max:expr) => {
        impl Bounded for $t {
            #[inline]
            fn min_value() -> $t {
                $min
            }
            #[inline]
            fn max_value() -> $t {
                $max
            }
        }};} //ew
```

```rust[]
bounded_impl!(u8, u8::MIN, u8::MAX);
bounded_impl!(usize, usize::MIN, usize::MAX);
bounded_impl!(u16, u16::MIN, u16::MAX);
// 15 more...
```

<https://crates.io/crates/num-traits>

notes:

The num-traits crate here uses a small macro to repeat this implement block 15 times, you don't use this when using num-traits, it's just some private code inside this library the authors wrote to reduce code repetition.

But it's a great example.

Simple macros like this are easy to read, i can teach you right now:

---

```rust[1]
macro_rules! bounded_impl {
    ($t:ty, $min:expr, $max:expr) => {
        impl Bounded for $t {
            #[inline]
            fn min_value() -> $t {
                $min
            }
            #[inline]
            fn max_value() -> $t {
                $max
            }
        }
    };
}
```

```rust[]
bounded_impl!(u8, u8::MIN, u8::MAX);
```

notes:

Like functions, macros have a name, in this case `bounded_impl`

---

```rust[2]
macro_rules! bounded_impl {
    ($t:ty, $min:expr, $max:expr) => {
        impl Bounded for $t {
            #[inline]
            fn min_value() -> $t {
                $min
            }
            #[inline]
            fn max_value() -> $t {
                $max
            }
        }
    };
}
```

```rust[]
bounded_impl!(u8, u8::MIN, u8::MAX);
```

notes:

You can read line 2 as the signature of the macro.
There are 3 parameters.
We've got a parameter t, which is of type type,
parameter min, which is of type expression
and max, another expression.

The type of t is quite easy to imagine, its type is a type.
Look at the first parameter of the usage at the bottom, a u8 is being passed in. That's a type.

The second two parameters are of type expression.
Rust is an expression-based language, basically any line of rust code can go here, though as we know expressions are more than just lines (though they often are).

The two expressions that are being passed in are `u8::MIN` and `u8::MAX`

---

```rust[3-13]
macro_rules! bounded_impl {
    ($t:ty, $min:expr, $max:expr) => {
        impl Bounded for $t {
            #[inline]
            fn min_value() -> $t {
                $min
            }
            #[inline]
            fn max_value() -> $t {
                $max
            }
        }
    };
}
```

```rust[]
bounded_impl!(u8, u8::MIN, u8::MAX);
```

notes:

And finally we have the body of the macro.
This is the code that you can imagine being pasted in, at the point in your program that you call the macro.

You can see that the `t`, `min`, and `max` variables are being referenced with the dollar notation.

In this example, nothing complex is happening, those variables simply are being substituted.

Here's what the intermediate code would look like if you asked the compiler to show you its working.
(which you can do)

(using `rustc -Zunpretty=expanded`)

---

```rust[]
bounded_impl!(u8, u8::MIN, u8::MAX);
```

expands to:

```rust[]
impl Bounded for u8 {
	#[inline]
	fn min_value() -> u8 {
		u8::MIN
	}
	#[inline]
	fn max_value() -> u8 {
		u8::MAX
	}
}
```

notes:

This is the macroexpansion.

If you've ever used a templating language, and I think perhaps every programmer has, this should feel right at home.

But in Rust we can do so much more.

---

# The Secret Sauce

notes:

Now that you're familiar with how macros execute and can re-write syntax in-place, it's time to do impossible things.

There are two kinds of Rust macros, declarative macros that do simple syntax rewriting using macro_rules that you've just seen, and procedural macros that can do all that, AND execute arbitrary code at compile time.

Let's talk about a few examples of what you can do with Procedural Macros.

---

# Simple Syntax Rewriting

notes:

Macros are the ultimate boilerplate killer. Where other languages have to invent new syntax or force boilerplate on the user through copy and paste, rust's macros maximise code reuse at compile time.

Anything you can write you can template, and for procedural macros it doesn't even have to be valid rust.

---

```html
let page = html! {
  <html>
  <head>
	  <title>"My blog"</title>
  </head>
  <body>
	  <div id="my_div"></div>
  </body>
  </html>
};
```

<https://crates.io/crates/html-to-string-macro/>

notes:

Would you like to write html inside your rust code?
Of course you would, and you get syntax highlighting and compile checking for FREE.

Let me break this code a little to show you:

---

```js
1  error: close tag has no corresponding open tag  
  --> src/main.rs:15:21  
   |  
15 |       <div id="my_div"></badtag>  
   |                        ^
```

notes:

Beautiful!

I'll remind you Rust does not have native html or xml literals. We BUILT one with the macro system.

The dream of scala is alive in Rust.

But it gets BETTER: html is just data, a markup language, but what if you want to embed an entirely different programming language inside Rust?

---

# Entirely New Languages

```rust
lisp!(defun factorial ((n i32)) i32
  (if (<= n 1)
    1
    (* n (factorial (- n 1)))));

let graydons_way  = factorial(5 + 5);
let mccarthys_way = lisp!(factorial (+ 5 5));
assert!(graydons_way == mccarthys_way);
```

<https://crates.io/crates/macro_lisp>

notes:

making a lisp is a common toy project for computer science learners.
I recommend giving Make A Lisp. a go.
But this lisp isn't written WITH rust, it's embedded INSIDE rust as a macro.

The lisp macro block defines a function called `factorial`, which is expanded into a normal rust fn at compile time.

In this example Rust code looks like it can call lisp code and lisp can call rust, because what's happening is that it's all being compiled down to normal rust function syntax before being fed into the regular compiler.

Let me break it a bit:

---

```sql
 error[E0369]: cannot add `{integer}` to `&str`  
--> src/main.rs:16:21  
 |  
 | let mccarthys_way = lisp!(factorial (+ "five" 5));  
 |                     ^^^^^^^^^^^^^^^^^^^------^-^^  
 |                                        |      |  
 |                                        |      {integer}
 |                                        &str  
```

notes:
Don't misunderstand this error, this looks like a normal rust error BUT THE ERROR OCCURS INSIDE LISP, inside our new language. And we get this feature FOR FREE.

I can't stress enough how incredible this is, even thought most of us won't write whole new languages with it.

If you build your new DSL, language, or new syntax inside a rust macro, you don't throw away the whole language to do it, like you have to do if you build external source pre-processors.

Like jsx

---

## Counter-example: JSX

```js
$ node jsxtest.js    
/home/deck/projects/jsxtest.js:2  
 <h1>  
 ^  
  
SyntaxError: Unexpected token '<'
```

notes:

JSX requires extra IDE support on top of javascript because it's not javascript, it's just a well-supported templating language that you must build with babel or whatever.

And you better hope you've configured your source maps correctly or errors will happen on different lines than you expect

Let's look at compile-time execution.

---

# Arbitary Compile-time Execution

```rust[]
// let mut conn = <impl sqlx::Executor>;
let account = sqlx::query!(
    "SELECT name id, FROM account")
    .fetch_one(&mut conn)
    .await?;

println!("{:?}", account);
println!("{}: {}", account.id, account.name);
```

<https://crates.io/crates/sqlx>

notes:
this is the sqlx database library.
It's really excellent and the one I recommend for your next project.

It's not an ORM, or a DAL it's so much more.

Though it has normal functions for querying the database, it also has the query!() macro, as you can see here.

At runtime, this is just the same as any other string-based sql querying library.
It sends that string query along to the db through the connection.
But at compiletime, the query!() macro does some magic.

SQLx leverages the power of Rust's macros AND rich type system to fill in the string query with test data and execute it on your local dev database at compile time.

That statement was a bit dense.
Let me slow down and explain that again.

---

## Compile-time SQL Validation

![[18-rusts-magic-macros 2023-01-25 13.35.32.excalidraw]]

notes:

When you compile your code, the query!() macro executes, taking in the string literal of your sql query, parsing it and adding in random type-valid parameterised data.
random strings or numbers, perhaps.

Then this query is executed on your local dev database, perhaps postgres, inside a transaction.

This will either execute cleanly or fail, perhaps because you've got the type or name of parameters or table wrong.

If it passes, compilation continues normally.
If it fails, sqlx brushes up the error returned by your database and inserts it into the compiler error that is generated by the macro, and you see it inside your ide or after a cargo build at the exact right line.

Either way, the side-effects of the query, if there are any, are reverted by aborting the transaction.

Let me break it a bit so we can see what that looks like:

---

```js
❯ : cargo build
   Compiling sqlxtest v0.1.0
error: error returned from database:
(code: 1) near "badsyntax": syntax error
 --> src/main.rs:9:20
  |
9 |     let _account = sqlx::query!("badsyntax (1) as id")
  |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note this error originates in the macro
   `$crate::sqlx_macros::expand_query`
```

notes:

As you can see, the error was caught, in a rolled-back transaction on our local dev database.
the database tells us that the error is *near* "badsyntax", the exact mistake I introduced to the query.

---

![[neovim-sqlx-error.png]]

notes:
And this is bubbled up into my IDE using no extra plugins, just native Rust LSP, built in to vscode, neovim, and many other editors.

---

```rust[]
//DECLARATIVE
let v: Vec<u32> = vec![1, 2, 3];
```

```rust[]
//DERIVE
#[derive(MyHelloMacro)]
struct Pancakes;

Pancakes::hello_macro();
```

```rust[]
//ATTRIBUTE-LIKE
#[get("/")] 
fn index() {} 
```

```rust[]
//FUNCTION-LIKE
command!(mkv --fs);
```

notes:

In Rust, as we've seen, there are 4 ways of defining a macro depending on what you want to do.
The first and easiest is declarative macros, where you use the templating system to write code by example.

Next are three procedural macros:
- Custom `#[derive]` macros that add code to structs and enums with the `derive` attribute
- Attribute-like macros that define custom attributes usable on any item, and
- Function-like macros that look like function calls but operate on the tokens specified as their arguments and therefore do not need to ingest valid rust syntax

You and I will probably seldom have to write a macro, but our code will be given superpowers by the libraries we use that do.

To get these compile-time features in other languages, you must wait for the language or precompiler authors to update their code.

With Rust you can have any new language feature you can imagine TODAY.

---

![[rust-logo.png]]

# Rust's Witchcraft

notes:

# OUTTRO

If you would like to support my channel, get early ad-free and tracking-free videos and vip discord access head to patreon.com/noboilerplate.

If you're interested in transhumanism and hopepunk stories, check out my sci-fi podcast, Lost Terminal.

Or if urban fantasy is more your bag, click the bottom video to listen to a strange and beautiful podcast I produce called Modem Prometheus.

Source on github, links in the description, and corrections are in the pinned ERRATA comment.

Thank you so much for watching, talk to you on Discord.

```rust
  println!("That's all folks!");
} 
```
