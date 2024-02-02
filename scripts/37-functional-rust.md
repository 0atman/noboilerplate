<style>
:root {--r-code-font: "FiraCode Nerd Font";}
.reveal .hljs {min-height: 50%;}
</style>

f7f7f7 background slide colour

# Cargo.toml

```toml
[package]
name = "functional-rust"
version = "0.1.0"
edition = "2021"
```

# Lint Tweaks

These lints make clippy less noisy when I'm building the video

```rust
#![allow(clippy::items_after_statements)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::no_effect)]
#![allow(unused_must_use)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::never_loop)]
#![allow(clippy::eq_op)]
```

# Crate Attributes

```rust
#![feature(const_for)]
#![feature(const_fn_floating_point_arithmetic)]
```

# Setup

```rust
fn main() {
	println!("Rust video");
} 
```

---

### Title Ideas

#### In Search of Code Purity

#### Rust's Secret Purity System

#### My Purity Journey from Haskell to Go to Rust

#### Const Fn: Pure Functions in Rust

#### ~~Is Rust a Functional Language?~~

#### ~~In Search of a Perfect Language~~

#### Const Fn: Using Rust To Make Your Life Worse

#### Const Fn: How To Make Rust More Annoying

---

### Thumbnail Ideas

#### I'm not Sure whether what You Ask for is Actually what Will Make You Happy

#### As Pure as Possible, but no Purerer

---

![[tri-hex-moon-white-transparent.png|300]]

notes:

Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

---

## Formal Methods

- Z
- B
- ACL2
- Coq 🐓

notes:

I was taught formal methods at university, which included the languages Z, B, ACL2, and the excitingly named Coq.

Systems like pacemakers, autopilots and hospital ventilators must be formally proved, not just rigorously tested, because human life is on the line.

---

![[fuck-it-ship-it.jpg|800]]

&mdash; The web development world

notes:

But Formal Methods are expensive, require using unusual external verification languages, and most damning for web and application developers, they slow down iteration.

After graduating from university and getting a web development job, I despaired that the safety and guarantees of the formal systems that I had been introduced to weren't available to me as a web developer.

I was going to have to act if I wanted to live in a different world.

---

![[so-haskell-proved.png]]

notes:

So I did what any engineer would do, and took to Stack Overflow.

You can read my desperation in the question here, in what turned into my most popular stack overflow post.

One of the last answers I received, all those years ago, very sympathetically said:

---

<i class="fas fa-quote-left fa-2x fa-pull-left"></i>
_I'm not sure whether what you ask for is actually what will make you happy. :-)_

&mdash; [apfeλmus](https://stackoverflow.com/questions/4077970/can-haskell-functions-be-proved-model-checked-verified-with-correctness-properti)

notes:

"I'm not sure whether what you ask for, is actually what will make you happy."

I didn't know what he meant, until I found Rust after previously trying Haskell.

---

![[cc-logo.png]]

## Public Domain Videos

[https://github.com/0atman/noboilerplate/](https://github.com/0atman/noboilerplate/)

notes:
Everything you see in this video: script, links, and images are part of a markdown document available freely on github under a public domain licence.

---

# PART 1: $λ$ Haskell

Pure by Default

```[]
factorial :: (Integral a) => a -> a
factorial 0 = 1
factorial n = n * factorial (n - 1)
```

Side-Effects (IO) are Contained

```[]
main :: IO ()
main = print (factorial 10)
```

notes:

I ADORE Haskell. In my 'how to learn rust' video, I recommended learning the basics to teach you functional programming quickly.

The language has many incredible features that give me similar confidence to using Formal Methods, and the one I'll hightlight today is functional purity, which is a term you might be familiar with.

In Haskell, and maybe, kinda, Rust, it's a first-class feature.

The first function here, factorial, is a pure function, a function that doesn't cause or rely upon side-effects.
We know this without reading the function body because it doesn't have IO.

The second function, main, prints to the screen, and so MUST have IO in the signature.

This is a fantastic way to keep side-effects managed, and covers half of the nightmare errors I've seen throughout my career.

But just like Formal Methods, Haskell, my beloved, is too complicated to use for general programming.

---

![[37-functional-rust 2024-01-15 12.20.29.excalidraw.svg]]

notes:

Formal Methods are for proving the critical state machine of your autopilot, or proving that your pacemaker never stops beating the patient's heart.

They are very theoretically sound, but impractical.

Normal popular programming languages are very practical, but not theoretically sound.

---

<i class="fas fa-quote-left fa-2x fa-pull-left"></i>
_I'm not sure whether what you ask for is actually what will make you happy. :-)_

&mdash; [apfeλmus](https://stackoverflow.com/questions/4077970/can-haskell-functions-be-proved-model-checked-verified-with-correctness-properti)

notes:

I realise now that this was probably what Apfelmus was telling me here.

I would HATE having to use formal methods to prove my webapps, just as I'd probably dislike coding them in Haskell for similar reasons.

I need to compromise.

My research moved on to what I consider the best compromise available to us:

ML-inspired languages, with their comprehensive compile-time type safety, functional programming, and specifically, side effect elimination.

---

## Formal Methods and Proptest Deep-dive

_I just want everything to be perfect!_

&nbsp;

## [Patreon.com/NoBoilerplate](http://www.patreon.com/noboilerplate)

notes:

I've had to cut for time a deep-dive into Formal Methods, but the full ad-free video is available on my Patreon if you're interested.

It's just me running this channel, and I'm so grateful to everyone for supporting me on this wild adventure.

If you'd like to see and give feedback on my videos up to a week early, as well as get discord perks, and even your name in the credits, it would be very kind of you to check my Patreon.

I'm also offering a limited number of mentoring slots. If you'd like 1:1 tuition on Rust, Personal organisation, creative production, Web tech, or anything that I talk about in my videos, do sign up and let's chat!

---

### Patreon-Only Formal Methods and Proptest Deep-dive

_I just want everything to be perfect_

notes:

Formal Methods in software engineering are used with the aim of increasing the reliability and robustness of our systems, similar to how other engineering disciplines design bridges and aircraft based on simulations and physical modelling.

Theoretical computer science fundamentals such as logic calculi, automata theory, control theory, program semantics, type systems, and type theory are all part of formal methods, not just the formal languages themselves

But they have limits.

---

### $A + B \equiv B + A $

```lisp
ACL2 !> (thm (=(+ a b) (+ b a)))

Q.E.D.

Summary
Form:  ( THM ...)
Rules: ((:DEFINITION =)
        (:EXECUTABLE-COUNTERPART TAU-SYSTEM))

Prover steps counted:  10
Time:  0.00 seconds
Proof succeeded.
```

[`0atman.com/articles/13/ACL2`](https://www.0atman.com/articles/13/ACL2)
notes:

The first entry on my blog, 0atman.com, in 2013, is this output from ACL2, proving that _a plus b always equals b plus a_.

ACL2 code is written in Lisp, but formal methods can be used for any language, or can machine translate from their own language to, say, java or c++.

Note the time output in this run, this proof took 0ms on whatever nightmare slow computer I was running a decade ago.
The explanation for this speed is just above it:
`Prover steps counted: 10.`

Just 10 steps to prove that all infinite combinations of addition are equivalent.

Let me explain how this works by showing you what it's NOT doing:

---

```rust[5-6]
use proptest::prelude::*;

proptest! {
    #[test]
    fn adder(a in any::<u32>(), b in any::<u32>()) {
        assert_eq!(a + b, b + a);
    }
}
```

```toml
proptest = "1.4.0"
```

notes:

If you were to use a general-purpose programming language, like here with Rust and the fantastic Proptest crate, you might reasonably test that `a + b` always equals `b + a`, by picking random values within a range and testing that some invariant held.

Here, I'm using the `proptest` macro coupled with its `any` type in this very concise example.

But there's a problem:
As soon as we add two random numbers that are larger than `u32::MAX / 2`, we will overflow the integer type.

Which means we can't prove this exhaustively using randomised iteration.

But `proptest` did something cool here that we should pay attention to.

---

```rust[7]
[1] failed: adder
attempt to add with overflow
thread 'adder' panicked at src/main.rs:13:1:
 
Test failed: attempt to add with overflow.

minimal failing input: a = 2955326243, b = 1339641053
     successes: 0
     local rejects: 0
     global rejects: 0

```

```rust
#[test]
fn u32_max() {
	assert_eq!(u32::MAX, 4_294_967_295);
}
```

notes:

Here's `proptest`'s output when the overflow happened.

It not only proved that we had written potentially overflowing code, but when it did, it then iterated until it found a minimal failing example.

Line 7, here, shows the two values it found right at the edge of the failure.
Add those two values of a an b together, and you get exactly one OVER the u32 limit.

Upon finding a failure, `proptest` does this search for minimal failing edge cases automatically, and it can do this for:

Strings, Arrays, Bools, Chars, Ints and Floats, Options, anything from Std::Collection including Vecs, BTrees and Hashes, and with a little help, any custom datatype you make.

---

```rust[]
fn adder(a in 0..1000u32, b in 0..1000u32) {
```

```rust[6]
   Compiling functional-rust v0.1.0 
    Finished test target in 1.02s
     Running unittests src/main.rs 

running 1 tests
test adder ... ok

test result: ok. 1 passed; finished in 0.01s
```

notes:

Here's one way to fix the test, by the way, by testing a smaller range of integers, if you know the state of your application is bounded like this.

But, more likely, you'd realise that you've written a function that can cause an integer overflow, and you should rewrite it to not do that, perhaps using an auto-promoting integer crate, or checked arithmetic, that returns results instead of blindly letting the CPU do whatever nonsense it wants to with the bits!

---

### Formal Proof Requires Finite State

![[Turnstile_state_machine_colored.png]]

notes:

I came to realise that formal methods were impractical for the kinds of tasks I wanted to use them with: general-purpose Web apps and services.

The big problems are state, side effects, and mutation.

Theorem Provers, like ACL2, must restrict themselves to just tiny portions of your app because they can only prove these sections in isolation.

To prove a webapp end-to-end would take longer than the predicted age of the universe to run because we can't just test the happy path, we must PROVE ALL POSSIBLE STATES OF THE INTERNET.

And there's quite a few of those!

---

# Part 2

## Functional Programming

_I just wanna chill_

---

| Language | Features |
| ---- | ---- |
| Scala | JVM ecosystem, hybrid OO/FP |
| Haskell | ML, Lazy, Type System |
| Clojure | JVM, FP, LISP! |
| Nim | Pure functions, fast, low-level |
| Go | Popular, Fast, low-level |
| Rust 🦀 | FP, Type System, Low-level |

notes:
Here are the broad strokes of my 15-year journey so far:

1. I started with Scala, it has an advanced type system, which I knew was good for getting more logic checked by the compiler, and back in 2013 it had statically-typed html built-in, which was quite a trick!
2. Then people told me Haskell was an even more powerful functional language, with a best-in class purity system and after a few rocky starts, I learned to appreciate the "if it compiles, it works" mindset. But I think that Haskell's narrow, high-level niche and esoteric ML syntax is holding it back from wider adoption. It's the best language in the world, no doubt, but that's not all we must consider in the real world.
3. Clojure, the most popular modern lisp, built on the JVM, came close, for me. It was so popular I was able to join a startup bank and get paid to code it for 2 years! (Imagine that!) But as with nearly all lisps, it has no compile-time type system. Though I know about Typed closure, it's an ugly add-on, not a core part of the language, same problem as typescript and python and ruby. Lisp's access to compile-time changed my life with Macros, and you know how that ended!
4. NIM was a huge revelation: This language reads like python, but compiles to fast static binaries, with almost every single good design choice I wanted! But its garbage collector and obscurity made me move on, despite recent impressive work with arc and oarc subsystems.
5. Go very nearly worked for me. Fast, lower-level compared to all others on this list except for Rust, and is multi-paradigm, with a strong showing from Functional Programming.

But Go is inelegant.

And I don't mean the syntax, beauty is in the eye of the binaryholder, and you'll never get two people to agree upon that.
If anything, it's TOO practical. There's no beauty to be found in Go, just a sort of crushing march of efficiency, and I don't mean that as a compliment.

It's got the practicality of a chainsaw, but what I want is a poem that cuts just as sharp.

---

![[rust-logo.png]]

notes:

in 2020, after a few false starts I found rust.

And I have great news, Rust is more functional than I realised until recently.

The functional ML roots of the language, Graydon's first Rust compiler was written in OCaml, shine through, influencing it right from the start.

It's not "C++ but better".

It's Haskell standing on Lisp's shoulders, hiding in C's coat to sneak into _PRDCTN_. (The fancy nightclub where all the popular languages hang out)

Let's look at how Rust can manage side effects

---

# Set Up

```toml
[dependencies]
color-eyre = "0.6.2"
rstest = "0.18.2"
```

(in `cargo.toml`)

notes:

Here is the classic set-up I include in all of my Rust projects, pretty and simple error handling with `color-eyre`, and the `rstest` testing framework, the most important feature for me is its ability to use fixtures.

---

## Killer Crates

```toml
rayon = "1.8"
const_panic = "0.2"
```

notes:

Later on, I'm going to use these two cool crates to demo the utility of Rust's functional techniques today.
- `Rayon` is the simplest parallelism library you've ever seen, built on Rust's controlled side-effect systems, and
- `const_panic` allows string formatting for asserts in a const context.

---

## Superpower Lints

```toml
[lints.rust]
unsafe_code = "forbid"

[lints.clippy]
enum_glob_use = "deny"
pedantic = "deny"
nursery = "deny"
unwrap_used = "deny"
```

```sh
$ cargo clippy
$ bacon clippy # or even better!

[clippy negs your code here]
```

notes:

Enable all these lints. Allow individual lines if you need to.

If you're using rust without using `bacon` and `clippy`, you're doing yourself a great disservice.

VSCode or your editor will likely use clippy behind the scenes, but you must also run clippy in a terminal to answer the most important question:

"What was the first error", as that's the one you have to fix.

---

```toml
[profile.release]
opt-level = 'z'   # Optimize for size.
lto = true        # Enable Link Time Optimisation
codegen-units = 1 # Reduced to increase optimisations.
panic = 'abort'   # Abort on panic
strip = "symbols" # Strip symbols from binary
```

More: https://github.com/johnthagen/min-sized-rust

notes:

And, as a bonus, here's a reasonable starting point for optimising the size of your release builds, cutting a helloworld down from 3MB to 300K.

You can get it down to 8K if you really want to, but I recommend stopping here and starting building.

---

```rust
// #[cfg(test)] omitted because I want linting
mod tests {

#[allow(clippy::wildcard_imports)]
use rstest::*;

#[fixture]
fn setup() {
	#[allow(clippy::unwrap_used)]
	color_eyre::install().unwrap();
}
```

notes:
Here's our test module setup.

Everything you see in this video from now on will be inside this module.
All of my code examples are concatenated into main.rs and compiled, this is how I statically type my literate programming videos.
(see my repo for details)

I'm importing the `rstest` prelude, which pulls in test attributes such as `fixture`, here, which runs this line ahead of every test, setting up `color_eyre`'s pretty errors.

---

### Functional Features In Most Languages

1. First-class functions
2. Anonymous functions (`lambda`s or `function()`s)
3. Iterators
4. `map()`, `filter()`, et al

notes:

Most modern languages have these functional-light features.
Javascript, Go, Python, and Ruby all have most of these.

---

### Rust's Functional Features

1. Cofigurable Closures (move or borrow)
2. Dataclasses (Enums)
3. Lazy Evaluation (limited with iterators, unlimited with macros)
4. Tail call recursion (with the `tailcall` crate, and in the future `become` keyword)
5. **PURITY** 👼 (kindof, stay tuned)
notes:

They are hugely useful, and Rust, of course, has them all as well as a few more.

But Rust has a killer feature.

Rust has Pure functions.

Kindof.

---

# Pure Functions

### Deterministic

### Side-effect-free

```rust[]
fn f(x: i32) -> i32 { x + 1 }
```

notes:

If you want to write code you can reason about and guarantee, pure functions are WILDLY useful.

Pure functions are good neighbours: They don't look through your windows or throw junk in your garden.

Normal functions can do anything, which means you have to read the code to figure out what they are doing.

Pure functions' utility comes from their limitations, they can't do everything:

They are functions where their output only depends on their inputs, and they have no side-effects (eg. memory or I/O)

If your language has a way of separating or tagging functions that are pure, and then can hold you to that contract, both you and the compiler can reason about your code in useful, new ways:

- if a pure function is called twice with the same inputs, the result is guaranteed to be the same every time
    - this is called referential transparency or idempotence or determinism
	- this enables perfect, predictable, caching of return values, which your compiler might do automatically,
	- and easier debugging!
- Pure functions that don't cause side-effects also allow perfect parallelisation, the functions can run on separate threads, processes, machines, or even in different geographic datacenters.
	- Their output is only affected by their inputs, so you bundle the inputs with the functions, and they can run anywhere.
- If a pure function is never called, it can be automatically removed by the compiler, or ignored by the developer.

---

#### Rust's Guards against Side-effects

- Immutable design by default
- Borrow checker enforcement

notes:

Early on in its development, Rust had a `pure fn` system, just like Haskell!

It was abandoned because it was soon discovered that the language had already solved many of the problems that a pure function system would guard against:

The compiler and developer don't usually need further function annotation to understand the side effects of functions, the type system and language ALREADY encodes that to an enormous extent.

I'll talk more about that later.

But if you really want a function purity system in Rust, like I do, there kindof is one.

---

```rust[]
fn addone(x: i32) -> i32 { x + 1 }
```

What is wrong here?

notes:

If you've set up clippy like I recommend, this simple rust function from earlier will not compile.

What on earth could be wrong with it?

---

```js
$ cargo clippy

 1  error: this could be a `const fn`
   --> src/main.rs:20:1
    |
 20 | fn addone(x: i32) -> i32 { x + 1 }
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

The fix:

```rust
const fn faddone(x: i32) -> i32 { x + 1 }
```

notes:

Clippy tells us that this function could be a `const fn`.

This error was my first clue that rust is doing something interesting.

The fix, as ever, is to do what clippy says, obey the compiler and turn the function into a const function.

---

![[teepublic-obey-sticker.png|500]]

https://noboilerplate.org

note:

("obey the compiler" merch available at noboilerplate.org)

---

## Constant Functions

```rust[]
const fn addone(x: i32) -> i32 { x + 1 }
```

notes:

const functions are functions that can be executed at compile time AS WELL AS runtime.
They differ from rust macros, which can only run at compile time, and can do anything, by being much more limited.

And just like with Pure functions, these limits make them exciting:

---

<i class="fas fa-quote-left fa-2x fa-pull-left"></i>
_Behaviors such as out of bounds [array indexing](https://doc.rust-lang.org/reference/expressions/array-expr.html#array-and-slice-indexing-expressions) or [overflow](https://doc.rust-lang.org/reference/expressions/operator-expr.html#overflow) are compiler errors if the value must be evaluated at compile time._

&mdash; [rust-lang.org/reference/const_eval.html](https://doc.rust-lang.org/reference/const_eval.html)

_(try https://lib.rs/crates/konst for const std functions)_

notes:
`const` functions have access to a limited subset of Rust, you could read about it here, but let's experiment because that's more fun:

(By the way, if you're curious, the `konst` crate has const equivalents of many standard library functions and methods that are in the process of being made const compatible in the standard library)

---

## Const Functions in Rust

```rust
const fn demo(x: i32, y: i32, f: f64) -> &'static str {
    x + y - x / y * x;
	(1, 2, 3).0;
	[1, 2, 3][0];
	let maybe_number = Option::Some(42);
	let adder = |z: i32| x + 1;
	let number_ref = &maybe_number;
	10 as f64 + f;
	loop {break 1};
	match maybe_number {
		Some(42..=42) => "Answer found!",
		_             => "Keep searching..."
	}
}
```

notes:

So what can we do in a const function, and perhaps more importantly, what can we not do?

Well, it's obvious that a lot works here:
- arithmetic operators on ints and floats
- tuple creation and indexing
- arrays and array slicing with usize
- struct creation and use
- closure expressions without capturing
- shared borrows, except for borrowing of values with interior mutability
- casting
    - except for casting to memory addresses
- calling other `const` functions
- `loop`, `while`, `while let`, and
- `if` `if let` and `match`
    - no proper `for` loop yet, though it's being worked on, and
- range expressions

That's quite a lot!

But not everything, and some of what works is qualified. What's the pattern and is it all pure?

Let's look at some examples to find out:

---

## No Mutable References

```rust[3]
const fn mut_refs() {
	let name = "Tris";
	let name_mut_ref = &mut name;
}
```

```js
error: mutable references are not allowed in const fn
   --> src/main.rs:43:21
    |
 43 |     let name_mut_ref = &mut name;
    |                        ^^^^^^^^^
```

notes:

Well that makes sense, that would leak our side-effects into calling code.

What else?

---

## No Interior Mutability

```rust[2-3]
const fn borrow_interior() {
	let z = std::cell::UnsafeCell::new(0);
	let zref = &z;
}
```

```js
error: cannot borrow here, since the borrowed element 
may contain interior mutability
   --> src/main.rs:43:13
    |
 43 |     let zref = &z;
    |                ^^
```

notes:

Ok, good, mutability and state is what we're trying to avoid with purity.

---

## No String Comparison

```rust[4]
const fn str_matching() {
	let name = "Tris";
	match name {
		"Tris" => "Great!",
		_      => "Nobody's perfect!"
	};
}
```

```js
error: cannot match on str in const fn
    |
 44 |         "Tris" => "Great!",
    |         ^^^^^^
    = note str cannot be compared in compile-time, and 
 therefore cannot be used in matches
```

notes:

OK, that's interesting, but we can work around that.

---

### Only Partial Floating Point Support

```rust[2]
const fn circumference(radius: f64) -> f64 {
    (std::f64::consts::PI * radius).powi(2)
}
```

```js
error: cannot call non-const fn std::f64::powi 
in constant functions
   --> src/main.rs:44:37
    |
 44 |     (std::f64::consts::PI * radius).powi(2)
    |                                     ^^^^^^^
```

`powi()` et al is available in [`const_soft_float`](https://lib.rs/crates/const_soft_float)

notes:

Even with allowing floating point arithmetic with a crate attribute, see my source code for details, not all floating point operations seem to be supported.

And this is to do with the side-effect that your processor, from the code's point of view, isn't dependent on the inputs to your pure function when doing floating-point operations.

Change the target, change the floating point hardware, and you could get a different output for the same input.

If you think about it, const functions are not weird for disallowing them, the REST of us are weird for assuming they work!

---

## No Iteration

```rust[2]
const fn non_const_fn() {
	for n in 1..101 {}
}
```

```js
error: cannot convert std::ops::Range<i32> 
into an iterator in constant functions
44  |     for n in 1..101 {}
    |              ^^^^^^
error: mutable references are not allowed
in constant functions
44 |     for n in 1..101 {}
   |              ^^^^^^
error: cannot call non-const fn <std::ops::Range<i32>
as Iterator>::next in constant functions
   |
44 |     for n in 1..101 {}
   |              ^^^^^^
```

notes:

OK, iteration is not possible without using a crate like `konst`.

The state held inside an iterator is inherently challenging in a const environment.

For loops and ranges are partially supported, but as soon as they start allocating, side-effects become a challenge.

Some of this may become possible in future version of rust, I would imagine, most release notes for even minor versions of Rust mention functions that have now been tweaked to be const compatible.

Rust disallows any mutating iter methods, which is, like, nearly all of them.

This is how it works for the whole standard library, by the way:

---

## No File System Access

```rust[2]
const fn impure_function_fs() {
    let _ = std::fs::File::create("foo.txt");
}
```

```js
 1  error: cannot call non-const fn std::fs::File::create
in constant functions
   --> src/main.rs:55:13
    |
 55 |     let _ = std::fs::File::create("foo.txt");
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

notes:

For example, none of the file system handling functions are marked as const.

Because of course!
That's a big side-effect!

---

## Debug Printing

```rust[2]
const fn do_something() {
	dbg!("hi"); // or println!()
}
```

```js
error: cannot call non-const fn `std::io::_eprint` 
in constant functions
  --> src/main.rs:43:2
   |
43 |     dbg!("hi");
   |     ^^^^^^^^^^
```

notes:
You can't print either, this is a big one.

Because you can only call functions that are marked as const from a const function, you can't print to the screen.

io functions, like print, can't be called safely because they cause the side-effect of printing to the screen.

This feels very similar to how IO functions in Haskell work, and makes me quite excited for their potential purity.

---

### Compiler-driven Development

```rust[2]
const fn single_digit(value: i32) -> i32 {
	assert!(value < 10, "Value too big");
	return value
}
const _: i32 = single_digit(10);
```

```rust[4]
use const_panic::concat_assert;

const fn single_digit2(value: i32) -> i32 {
	concat_assert!(value < 10, "Too big: ", value);
	return value
}
const _: i32 = single_digit2(10);
```

notes:

So, when debugging, the only way to get information out of a const function is by its return type or halting compilation with a panic! or a failed assertion, like in the first code block here.

Because there's no runtime difference with making a function const, most of the standard library's functions that can safely be called from a const context are already marked const and ready to use.

So printing isn't possible, but if you take my advice, you'll debug with the compiler, not printing to the screen, and I love Rust's compiler driven development workflow!

Note that only simple error messages available in const assertions, you can't use string formatting to interpolate values.

But, in the second code block, the `const_panic` crate provides const versions of assertions that panic with an error message with variables and other useful context.

Const functions are looking great so far!

But there is a caveat to my dreams of purity:

---

```rust[2]
const fn impure_function_env() {
    std::env::var("PATH");
}
```

```js
 1  error: cannot call non-const fn std::env::var
    in constant functions
   --> src/main.rs:55:5
    |
 55 |     std::env::var("PATH");
    |     ^^^^^^^^^^^^^^^^^^^^^
```

notes:

To illustrate this caveat and to button up our understanding of const functions, lets look at environment variables.

Of course, you can't read them from const functions, that's wildly out of scope, it's the host operating system's environment, a huge bundle of state we don't want to be influenced by nor influence!

---

```rust[2]
const fn be_careful_with_macros() {
    let path: &'static str = std::env!("PATH");
}
```

WAIT THIS WORKS!?

notes:

But OH NO, the macro version of `std::env` IS allowed in a const function.

WHAT is happening here, this is not pure at all!

Don't worry: You will find this pattern in Rust again and again:

Strict rules, but with well-signposted concessions to practicality.

Just like the safety hatch of unsafe, const functions have their own escape hatch: Macros.

Macros execute arbitrary code at compile time, and then can insert the results of that processing as potentially const values, like here, the result of interrogating the path is a const, static string.

Here's how I think of it:

---

## Rust's Const Functions Are only Pure at *runtime*

# Does This Matter?

notes:
Rust's const functions are only pure once you get to *runtime*

Once you understand this small caveat, you can still get enormous benefits from these pure-ish functions.

Most of my code runs at runtime, despite my best intentions!

I want pure business logic functions to be:
- predictable,
- never change based on runtime state, and
- never cause unpredictable side-effects in other runtime systems.

OK, this isn't proper Haskell-style purity, Rust doesn't have a single purity system.
But does that matter?

What it does have is a sort of _granular purity_ built-in to the whole language.

---

"Granular Purity"

```rust
fn granular_purity(impure: &mut String, pure: &String) {
	*impure = pure.to_owned();
}
```

Global state possible, but discouraged

```rust[1]
static GLOB: Mutex<Vec<i32>> = Mutex::new(Vec::new());
fn eww_gross_dont_do_this() {
    GLOB.lock().unwrap().push(42);
    println!("{:?}", GLOB.lock().unwrap());
}
```

More details: https://stackoverflow.com/a/27826181

notes:

- Rust's design means that the advantages of a purity system are less necessary than in less strict programming languages.
- See the first code block, everything is immutable unless you opt-in to mutability,
- and that is clearly signposted by the function signature telling you what can and can't be mutated.
- Global state is deliberately discouraged by the language and ecosystem - look at the second ugly code block here,
- Rust promotes side-effect-free patterns and functional and reactive programming.

The whole language is already aligned to solve the problems that a purity system solves in other languages, and Rust does so without sacrificing practicality like Haskell and Formal Methods do.

---

## Rayon

```rust
use rayon::prelude::*;
fn sum_of_squares(input: &[i32]) -> i32 {
    input.par_iter() // <-- just change that!
         .map(|&i| i * i)
         .sum()
}
```

(RAYON IS 9 YEARS OLD OMG)

notes:

By the way, this built-in granular control of side-effects is how Rayon is able to turn most iterators into parallel iterators, with a single-line change to your existing code.

Encoding side-effects into the type system allows you and the compiler and smart crates like Rayon to reason about the soundness of parallel code without a restrictive, gated, purity system.

So:

---

### Are Const Fns Pure? No

- ✅ Deterministic (at runtime)
- ✅ Side-effect-free

&nbsp;

notes:

Are const fns pure in rust?
No.

They're better than that.
They're practical

---

<i class="fas fa-quote-left fa-2x fa-pull-left"></i>

## _"Rust Is as Pure as Possible, but no purerer"_

&mdash; No Boilerplate, the People's Poet, 2024

notes:

Rust is as pure as possible, but no purerer.

Thank you!

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

---

```rust
// that's all folks!
}
```

---

