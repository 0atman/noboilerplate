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
```

# Imports

```rust
```

# Setup

```rust
fn main() {
	println!("Rust video");
} 
```

# Title Options:

- From Haskell to Go to Rust
- Is Rust a Functional Language?
- In Search of a Perfect Language

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

These Formal methods are mathematically rigorous techniques used for the specification, development, analysis, and verification of software and hardware systems where everything MUST BE CORRECT.

Systems like pacemakers, autopilot and hospital systems must be formally proved, not just rigorously tested, because human life is on the line.

---

![[fuck-it-ship-it.jpg|800]]

&mdash; The web development world

notes:

But Formal Methods are expensive, require using unusual external verification languages, and most damning for web developers, they are SLOW to develop.

After graduating university and getting a web development job, I despaired that the safety and guarantees of the formal systems that I had been introduced to weren't available to me.

---

![[so-haskell-proved.png]]

notes:

I did what any developer would do, I took to Stack Overflow.

You can read the desperation in my wording in the question, which turned out to be most popular stack overflow post of all time!

One of the last answers I received, all those years ago, very sympathetically said:

---

<i class="fas fa-quote-left fa-2x fa-pull-left"></i>
_I'm not sure whether what you ask for is actually what will make you happy. :-)_

&mdash; [apfeλmus](https://stackoverflow.com/questions/4077970/can-haskell-functions-be-proved-model-checked-verified-with-correctness-properti)

notes:

"I'm not sure whether what you ask for is actually what will make you happy."

I did not know what he meant, until I found Rust.

---

![[cc-logo.png]]

## Public Domain Videos

[https://github.com/0atman/noboilerplate/](https://github.com/0atman/noboilerplate/)

notes:
Everything you see in this video: script, links, and images are part of a markdown document available freely on github under a public domain licence.

---

# Part 1:
## Formal Methods

notes:

Formal Methods in software engineering are used with the aim of increasing the reliability and robustness of a system, similar to how other engineering disciplines design bridges and aircraft based on simulations and physical modelling.

Theoretical computer science fundamentals such as logic calculi,  automata theory, control theory, program semantics, type systems, and type theory are all part of formal methods, not just the formal languages themselves

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

notes:
- [ ] "A Computational Logic for Applicative Common Lisp"_

The first entry on my blog, 0atman.com, in 2013, is this output from ACL2, proving that _a plus b always equals b plus a_.

ACL2 code is written in Lisp, but formal methods can be used for any language, or can machine translate from their own language to, say, java or c++.

Note the time output in this run, this proof took less than 10ms on whatever slow nightmare computer I was running a decade ago.
The explanation for this speed is just above it, prover steps counted 10.

Just 10 steps to prove all combinations of addition are equivalent.

Let me show how this works by showing you what it's NOT doing:

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

If you were to use a general-purpose programming language, like here with Rust and the fantastic Proptest crate, you might test that `a + b` always equals `b + a`, by picking random values within a range and testing that some invariant held.

Here I'm using `proptest` to do this, the `proptest` macro coupled with its `any` type in this very concise example.

But there's a problem:
As soon as we add two random numbers that are larger than `u32::MAX / 2`, we will overflow the integer type.

OK so we can't prove this using randomised iteration.

But proptest did something cool here that we should pay attention to.

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

Here's proptest's output when the overflow happens

Proptest not only proved that we had written potentially overflowing code, but when it did, it then iterated until it found a minimal failing example.

Line 7, here, shows the two values it found right at the edge of the failure.
Add those two values of a an b together and you get exactly one OVER the u32 limit.

Upon finding a failure, `proptest` does a search for minimal failing edge cases automatically, and it can do this for:

Strings, Arrays, Bools, Chars, Ints and Floats, Options, anything from Std::Collection including Vecs, BTrees and Hashes, and with a little help, any custom datatype you make.

- [ ] you can kinda get dependent types by checking your new() method with proptest! (like how str and String do it)

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

Here's how to fix the test, by the way, you test a smaller range of integers.

Or, more likely, you'd realise that you've written a function that can cause an integer overflow, and rewrite it to not do that, perhaps using rust's checked arithmetic, that returns results instead of blindly letting the CPU do whatever nonsense it wants with the bits!

---

![[Turnstile_state_machine_colored.png]]

notes:

I came to realise that formal methods were impractical for the kinds of tasks I wanted to use them with: Web apps.

Even a single string has so much entropy, so many permutations of UTF-8 code points, that Theorem Provers, like ACL2, are not suitable.

Formal Methods are for proving the critical state machine of your autopilot, or proving that your pacemaker never stops beating the patient's heart.

Formal Methods are very theoretically sound, but unpractical.
Normal imperative programming languages are very practical, but not theoretically sound.

I needed to compromise.

My research moved on to what I consider the best compromise available to us: Functional Programming.

---

# Part 2:
## Functional Programming

---

| Language | Features                       |
| -------- | ------------------------------ |
| Scala    | JVM ecosystem, hybrid OO/FP                  |
| Haskell  | Strictly FP, Lazy, Type System |
| Clojure  | JVM, FP, LISP!                               |
| Go       | Popular, FAST, Low-level                               |
| Rust 🦀         | FP, Type System, Low-level                                |

notes:

Scala has a advanced type system, which I knew was good for getting more logic checked by the compiler, and back in 2013 had statically-typed html built-in, which was quite a trick!

But people told me haskell was a more powerful functional language, and after a few rocky starts, I learned to appreciate the "if it compiles, it works" mindset. It's even the most popular ML language, but I thought that it's academic background, aesoteric ML syntax, was holding it back from wide adoption. It's the best language in the world, no doubt, but that's not all there is to consider in the real world.

Clojure, the most popular lisp in the world, built on the JVM, came close, for me. It was so popular I was able to join a startup bank and code it for 2 years! But as with nearly all lisps, has no type system. Though I know about Typed closure, it's an ugly add-on, not a core part of the language.
Access to compile-time changed my life with Macros

Go very nearly worked for me. Fast, lower-level compared to all others on this list except for Rust, and was multi-paradigm, with Functional Programming with a strong showing.

But it was 

---

![[rust-logo.png]]

notes:

I have great news, Rust is

---

# Set Up

```toml
[dependencies]
color-eyre = "0.6.2"
rstest = "0.18.2"
```

notes:

Classic set-up

---

## Killer Crates

```toml
rayon = "1.8.0"
static_assertions = "1.1.0"
tailcall = "0.1.6"
```

notes:

We're going to use some of these cool crates to demo the utility of functional techniques

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

notes:

---

```toml
[profile.release]
opt-level = 'z'   # Optimize for size.
lto = true        # Enable Link Time Optimization
codegen-units = 1 # Reduce number of codegen units to increase optimizations.
panic = 'abort'   # Abort on panic
strip = "symbols" # Strip symbols from binary*
```

---

---

```rust

#[cfg(test)]
mod tests {

use rstest::*;

const fn adder(a: i32, b: i32) -> i32 {
    a + b
}


```

notes:

I'm very interested in typed lambda calculus. It's a shame that the lisp community are so against typing, they're real dynamic proponents! I sadly left the Clojure world because of this. Typed Clojure exists, but it suffers from the same problem Typescript does: It's a rarely-used afterthought.

My primary goal is to make things, to build a team, to change the world in a tiny way with programs that make people's lives better.

As Dijkstra said, "A program is like a poem, you cannot write a poem without writing it."

The worst-kept secret in software development is that we're more like poets and artists than masons and bricklayers. You ask a poet how many poems she will write in a year, and she truthfully says, "I cannot know".

So, though learning pure ML and FP languages has enriched my thinking a great deal, to get things done, I am obliged to compromise. Rust's compromises are much smarter to my mind than other popular languages because if you add too much FP, you get languages like you've suggested without wide adoption. But if you don't add enough, you end up with languages like Go, a very good modern language, but with no beauty and (for reasons perhaps best known to Ken Thompson) no sum types.

Scala was the focus of my interest for many years, but it's hamstrung by the JVM (as well as buoyed by it). Allowing nulls to infect your type system dramatically reduces the safety and confidence of it. Rust doesn't allow nulls into the type system, which means I get the strongest feeling of compiler-driven development in Rust of all the popular languages. Just like when I write Haskell, if it compiles, it WORKS!

I love this feeling in languages with comprehensive near-proof-level type systems, and I think one of the reasons Rust does this so well is that, unlike ALL other popular languages you mentioned, they didn't bolt on the FP ideas and type system as an after-thought.

All of Scala is an add-on to the JVM, and you can feel the drag when using it, Kotlin too.

F# has the same problem, the influence of the underlying CLR platform causes the same sorts of problems.

ML and FP languages that start off with, say, sum types, can build their entire ecosystem on them, using them everywhere (Haskell's `Maybe` monad, for instance). This is just what Rust did, the FP features aren't bolted on to a low-level language, they existed from the start, so there's:

- No exception system
- No errno returns
- No nulls as exceptions to the type system
- And side-effects are handled so well that you can get functionality like Haskell's `par` [3], to allow computation to occur in parallel, safely, with no code changes because the type system knows about side effects. [4]

BTW: The Rust compiler was built in OCaml, and the influence shines through, even though it's now self-hosting. Early on in development, Rust had a `pure fn` system, just like Haskell! It was abandoned because the powerful Borrow Checker doesn't need function annotation to understand the side effects of functions, the type system and language ALREADY encode that.

I have yet to do a video on rust and fp, though it might be the very next one I publish, so I'm afraid I can't point you to one of my videos, but you're wrong with most of your assumptions about Rust. I originally thought the same, there's a lot of bad takes on the internet about rust being 'just a better C'.

Context: I've written lisp professionally at a bank for 2 years (I'm as surprised as you are!), and I first learned Haskell in 2004, back in the bad old days of cabal.

Believe me when I say that I love fp, we are on the same page there. This channel wouldn't exist, and you and I would not be talking if Rust did not go above and beyond the small amounts of fp that other popular languages borrow.

I don't need Rust to have a Haskell-style side effects system and a lazy evaluation model. We've got that already, it's called Haskell.

Rust has the most important parts of lisp AND the most important parts of Haskell, all wrapped up in c's clothing. Look at this astonishing list of features that Rust snuck in so subtly that you don't think it's a functional language:

- pure functions
- dataclasses (rust's 'enums' are sum types, ie tagged unions)
- first-class functions and closures
- lazy evaluation (with iterators)
- full lisp-style macro system where all the language is available (ie you can rewrite syntax and make network requests at compile time, just like lisp)
- tail call optimisation for recursion (currently in testing in the `tailcall` crate)

That's literally 100% of what Wikipedia tells me counts as functional programming!

Haskell, of course, does all of this in a more strict way, and more. But that's all unfamiliar to mainstream developers, and limits Haskell's adoption.

I say again, we don't need another Haskell, we've got that already, it's called Haskell. What we need is a way to get more of fp's good ideas into the mainstream, and THAT is what Rust is doing, with clever concessions to practicality and familiarity.

I'm sorry to keep going on and on, but Rust is the best answer right now to my real-world provable language dreams, and until something better comes along, I'm going to keep shouting about it!

[1] <https://stackoverflow.com/questions/4065001/are-there-any-provable-real-world-languages-scala>

[2] <https://stackoverflow.com/questions/4077970/can-haskell-functions-be-proved-model-checked-verified-with-correctness-properti>

[3] <https://wiki.haskell.org/Par_and_seq>

[4] <https://lib.rs/crates/rayon>

# Functional Programming

## First-class Functions

- "first-class" is a computer science term for programming language entities that have no restriction on their use (thus first-class functions can appear anywhere in the program that other first-class entities like numbers can, including as arguments to other functions and as their return values).
- partial application (nearly the same as currying)

## Pure Functions

- no side-effects (memory or i/o)
- if a result isn't used, it can be removed
- if a pure function is called twice with the same inputs, the result is the same
    - this is called referential transparency or idempotence
- enables perfect caching
- perfect parralisation
- `const fn` can be used for this?
- side-effects are known at compile time in many cases by `let/mut` definitions

## Closures

<https://lib.rs/crates/static_assertions>

start with a quick fasterthanlime setup of rust
- lazynvim
    - a quick aside to obsidian.nvim why not
- binstall
- bacon
- color_eyre
- a few lints (configed using cargo.toml!)
- and some release binary fixes whynot

<https://www.youtube.com/watch?v=HlgG395PQWw>

const fn
<https://doc.rust-lang.org/reference/const_eval.html>

- clippy pedantic will tell you when you can upgrade to constant fun

## Recursion

- recursive functions invoke themselves
- tail recursion can be recognised by a compiler, but is not yet guarenteed in rust, work is ongoing

```rust
fn fibonacci_plain(n: u64) -> u64 {
    fn f(n: u64, a: u64, b: u64) -> u64 {
        match n {
            0 => a,
            _ => f(n - 1, a + b, a),
        }
    }
    f(n, 0, 1)
}
```

This example DOES eliminate the tail call, when we decompile it we get 12 instructions, no `call` used, and almost exactly the same as if we hand-rewrote it as a while loop.

```asm
example::fibonacci:
        push    1
        pop     rdx
        xor     ecx, ecx
.LBB0_1:
        mov     rax, rdx
        test    rdi, rdi
        je      .LBB0_3
        dec     rdi
        add     rcx, rax
        mov     rdx, rcx
        mov     rcx, rax
        jmp     .LBB0_1
.LBB0_3:
        ret
```

### Tailcall Crate

just like many rust features, this is being prototyped as a macro. You do not have to wait toget future features TODAY:

<https://lib.rs/crates/tailcall>

> Many implementations achieve this by using a device known as a [trampoline](https://en.wikipedia.org/wiki/Trampoline_(computers) "Trampoline (computers)"), a piece of code that repeatedly calls functions. All functions are entered via the trampoline. When a function has to tail-call another, instead of calling it directly and then returning the result, it returns the address of the function to be called and the call parameters back to the trampoline (from which it was called itself), and the trampoline takes care of calling this function next with the specified parameters. This ensures that the C stack does not grow and iteration can continue indefinitely.

this implementation is planned to eventually be folded into the language using the `become` keyword.

- [ ] does this work?
    - seems like debug mode never works
    - release mode always works

## Strict Vs Non-strict Evaluation

- lazy evaluation
- `print length([2+1, 3*2, 1/0, 5-4])` would equal 4, because the compiler has not been obliged to evaluate each array item
- iterators?

## Referential Transparency

- Functional programs do not have assignment statements, that is, the value of a variable in a functional program never changes once defined.
- `let` does this by default, and is a pattern that is greatly encouraged, but `let mut` exists as a concession to practicality.

## Data Strucutres

- persistant data sctructures, which copy themselves on modification, are used instead of data structures that have interior mutatability

## Lazyness

```rust
//(0..).map(|x| x * 2)
```

# Tagged Unions (ie Haskell's Datatypes, Rust's enums)

# Haskell

> Haskell features [lazy evaluation](https://en.wikipedia.org/wiki/Lazy_evaluation "Lazy evaluation"), [lambda expressions](https://en.wikipedia.org/wiki/Anonymous_function "Anonymous function"), [pattern matching](https://en.wikipedia.org/wiki/Pattern_matching "Pattern matching"), [list comprehension](https://en.wikipedia.org/wiki/List_comprehension "List comprehension"), [type classes](https://en.wikipedia.org/wiki/Type_class "Type class") and [type polymorphism](https://en.wikipedia.org/wiki/Type_polymorphism "Type polymorphism"). It is a [purely functional language](https://en.wikipedia.org/wiki/Purely_functional_language "Purely functional language"), which means that functions generally have no [side effects](https://en.wikipedia.org/wiki/Side_effect_(computer_science) "Side effect (computer science)"). A distinct construct exists to represent side effects, [orthogonal](https://en.wikipedia.org/wiki/Orthogonal#Computer_science "Orthogonal") to the type of functions. A pure function can return a side effect that is subsequently executed, modeling the [impure functions](https://en.wikipedia.org/wiki/Pure_function#Impure_functions "Pure function") of other languages.

Haskell has a [strong](https://en.wikipedia.org/wiki/Strongly_typed_programming_language "Strongly typed programming language"), [static](https://en.wikipedia.org/wiki/Static_type#Static_typing "Static type") type system based on [Hindley–Milner type inference](https://en.wikipedia.org/wiki/Hindley%E2%80%93Milner_type_inference "Hindley–Milner type inference"). Its principal innovation in this area is type classes, originally conceived as a principled way to add [overloading](https://en.wikipedia.org/wiki/Polymorphism_(computer_science) "Polymorphism (computer science)") to the language,[[40]](https://en.wikipedia.org/wiki/Haskell#cite_note-wadler89-40) but since finding many more uses.[[41]](https://en.wikipedia.org/wiki/Haskell#cite_note-hallgren01-41)

The construct that represents side-effects is an example of a [monad](https://en.wikipedia.org/wiki/Monad_(functional_programming) "Monad (functional programming)"): a general framework which can model various computations such as error handling, [nondeterminism](https://en.wikipedia.org/wiki/Nondeterministic_algorithm "Nondeterministic algorithm"), [parsing](https://en.wikipedia.org/wiki/Parsing "Parsing") and [software transactional memory](https://en.wikipedia.org/wiki/Software_transactional_memory "Software transactional memory"). They are defined as ordinary datatypes, but Haskell provides some [syntactic sugar](https://en.wikipedia.org/wiki/Syntactic_sugar "Syntactic sugar") for their use.

```rust

/// Recursion
fn fibonacci(n: u64) -> u64 {
    fn f(n: u64, a: u64, b: u64) -> u64 {
        match n {
            0 => a,
            _ => f(n - 1, a + b, a),
        }
    }
    f(n, 0, 1)
}
    #[rstest]
    #[should_panic(expected = "overflow")]
    fn recursion() {
        fibonacci(100);
    }


    #[fixture]
    fn setup() {
        color_eyre::install().expect("color_eyre installed");
    }


```

```rust
fn laziness() {
    let _ = (0..).map(|x| x * 2);
}
```

```rust
const fn pure_function(a: i32, b: i32) -> i32 {
    a + b
}
```

---

```rust[]
const fn impure_function_env() {
    std::env::var("PATH");
}
```

```sql
 1  error[E0015]: cannot call non-const fn `std::env::var::<&s
 tr>` in constant functions
   --> src/main.rs:55:5
    |
 55 |     std::env::var("PATH");
    |     ^^^^^^^^^^^^^^^^^^^^^
    |
    = note: calls in constant functions are limited to constan
 t functions, tuple structs and tuple variants
```

---

```rust[]
const fn impure_function_fs() {
    let _ = std::fs::File::create("foo.txt");
}
```

```sql
 1  error[E0015]: cannot call non-const fn `std::fs::File::cre
 ate::<&str>` in constant functions
   --> src/main.rs:55:13
    |
 55 |     let _ = std::fs::File::create("foo.txt");
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: calls in constant functions are limited to constan
 t functions, tuple structs and tuple variants

```

---

```rust
const fn be_careful_with_macros() {
    std::env!("PATH");
}
```

Rust's const functions are only pure at *runtime*

---

# Const Diversion

```rust
use static_assertions::const_assert;
const_assert!(1 == 1);


#[rstest]
#[should_panic(expected = "assertion")]
fn assertter() {
	assert_eq!(1,2);
}

```

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

# Thumbnails

---

## I'm not Sure whether what You Ask for is Actually what Will Make You Happy

---
