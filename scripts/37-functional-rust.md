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
```

# Crate Attributes

```rust
#![feature(const_for)]
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
- Using Rust To Make Your Life Worse
- How To Make Rust More Annoying

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

After graduating university and getting a web development job, I despaired that the safety and guarantees of the formal systems that I had been introduced to weren't available to me as a web developer.

I was going to have to act if I wanted to live in a different world.

---

![[so-haskell-proved.png]]

notes:

So I did what any engineer would do, and took to Stack Overflow.

You can read my desperation in the question here, in what turned into my most popular stack overflow question.

One of the last answers I received, all those years ago, very sympathetically said:

---

<i class="fas fa-quote-left fa-2x fa-pull-left"></i>
_I'm not sure whether what you ask for is actually what will make you happy. :-)_

&mdash; [apfeλmus](https://stackoverflow.com/questions/4077970/can-haskell-functions-be-proved-model-checked-verified-with-correctness-properti)

notes:

"I'm not sure whether what you ask for, is actually what will make you happy."

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

_I just want everything to be perfect_

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

https://www.0atman.com/articles/13/ACL2

notes:

The first entry on my blog, 0atman.com, in 2013, is this output from ACL2, proving that _a plus b always equals b plus a_.

ACL2 code is written in Lisp, but formal methods can be used for any language, or can machine translate from their own language to, say, java or c++.

Note the time output in this run, this proof took 0ms on whatever slow nightmare computer I was running a decade ago.
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

Here I'm using the `proptest` macro coupled with its `any` type in this very concise example.

But there's a problem:
As soon as we add two random numbers that are larger than `u32::MAX / 2`, we will overflow the integer type.

we can't prove this exhaustively using randomised iteration.

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

Here's `proptest`'s output when the overflow happens

It not only proved that we had written potentially overflowing code, but when it did, it then iterated until it found a minimal failing example.

Line 7, here, shows the two values it found right at the edge of the failure.
Add those two values of a an b together and you get exactly one OVER the u32 limit.

Upon finding a failure, `proptest` does this search for minimal failing edge cases automatically, and it can do this for:

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

Here's one way to fix the test, by the way, by testing a smaller range of integers, if you know the state of your application is bounded like this.

But, more likely, you'd realise that you've written a function that can cause an integer overflow, and you should rewrite it to not do that, perhaps using an auto-promoting integer crate, or checked arithmetic, that returns results instead of blindly letting the CPU do whatever nonsense it wants with the bits!

---

### Formal proof requires finite state
![[Turnstile_state_machine_colored.png]]

notes:

I came to realise that formal methods were impractical for the kinds of tasks I wanted to use them with: general-purpose Web apps and services.

Even a single string has so much entropy, so many permutations of UTF-8 code points, that Theorem Provers, like ACL2, are not suitable.
they would take longer than the predicted age of the universe to run.

---


venn diagram this

notes:

Formal Methods are for proving the critical state machine of your autopilot, or proving that your pacemaker never stops beating the patient's heart.

They are very theoretically sound, but impractical.
Normal imperative programming languages are very practical, but not theoretically sound.
I needed to compromise.

My research moved on to what I consider the best compromise available to us: Functional Programming.

---


## [Patreon.com/NoBoilerplate](http://www.patreon.com/noboilerplate)

notes:

It's just me running this channel, and I'm so grateful to everyone for supporting me on this wild adventure.

If you'd like to see and give feedback on my videos up to a week early, as well as get discord perks, and even your name in the credits, it would be very kind of you to check my Patreon.

I'm also offering a limited number of mentoring slots. If you'd like 1:1 tuition on Rust, Personal organisation, creative production, Web tech, or anything that I talk about in my videos, do sign up and let's chat!

Let's talk about Functional Programming:

---

# Part 2:
## Functional Programming


_I just wanna chill_

---

| Language | Features                       |
| -------- | ------------------------------ |
| Scala    | JVM ecosystem, hybrid OO/FP                  |
| Haskell  | Strictly FP, Lazy, Type System |
| Clojure  | JVM, FP, LISP!                               |
| Go       | Popular, Fast, Low-level                               |
| Rust 🦀         | FP, Type System, Low-level                                |

notes:
Here's the broad strokes of my 15-year journey:

1. Scala has an advanced type system, which I knew was good for getting more logic checked by the compiler, and back in 2013 it had statically-typed html built-in, which was quite a trick!
2. But people told me Haskell was a more powerful functional language, and after a few rocky starts, I learned to appreciate the "if it compiles, it works" mindset. But I think that its narrow, high-level usage and esoteric ML syntax is holding it back from wideer adoption. It's the best language in the world, no doubt, but that's not all we must consider in the real world.
3. Clojure, the most popular lisp, built on the JVM, came close, for me. It was so popular I was able to join a startup bank and and get paid to code it for 2 years! But as with nearly all lisps, it has no compile-time type system. Though I know about Typed closure, it's an ugly add-on, not a core part of the language, same problem as typescript and python, and ruby Lisp's access to compile-time changed my life with Macros, and you know how that ended.
5. Go very nearly worked for me. Fast, lower-level compared to all others on this list except for Rust, and is multi-paradigm, with a strong showing from Functional Programming.

But Go is inelegant. 

And I don't mean the syntax, beauty is in the eye of the binaryholder, and you'll never get two people to agree upon that.
If anything, it's TOO practical. There's no beauty to be found in Go, just a sort of crushing march of efficiency, and I don't mean that as a complement.

It's got the practicality of a chainsaw, but what I want is a poem that cuts just as sharp.

7:00

---

![[rust-logo.png]]

notes:

And in 2020, after a few false starts I found rust.

And I have great news, Rust is more functional than I realised until recently.

The functional ML roots of the language, Graydon's first Rust compiler was written in OCaml, shine through, influencing it right from the start.
- [ ] seed this earlier by talking about how great ML languages are
It's not "C++ but better". It's Haskell standing on Lisp's shoulders hiding in C's overcoat to sneak into the popular language's party. 

Let's see some code.

---

# Set Up

```toml
[dependencies]
color-eyre = "0.6.2"
rstest = "0.18.2"
```

(in `cargo.toml`)

notes:

Here is the classic set-up I include in all of my Rust projects, pretty and simple error handling with `color-eyre`, and the `rstest` testing framework, the most important feature for me is it's ability to use fixtures.

---

## Killer Crates

```toml
rayon = "1.8.0"
static_assertions = "1.1.0"
tailcall = "0.1.6"
```

notes:

Next, I'm going to use some of these cool crates to demo the utility of functional techniques today.
- `Rayon` is the simplest parallelism library you've ever seen, built on Rust's immutability fundamental,
- `static_ssertions` allow you to simply reason about your compile-time code without touching macros, and
- [ ] say this right idiot
- We'll look at `tailcall` as a glimpse of Rust's future functional features. 

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

[clippy negs you here]
```

notes:

If you're using rust without using Clippy, you're doing yourself a great disservice.

VSCode or your editor will use it behind the scenes, but you must also run clippy in a terminal to answer the most important question: 

What was the first error, that's the one you have to fix.

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

And here's a reasonable starting point for optimising the size of your release builds, cutting a helloworld down from 3MB to 300K.

You can get it down to 8K if you really want to, but I recommend stopping here and starting building. 

---

```rust
mod tests {

#[allow(clippy::wildcard_imports)]
use rstest::*;

#[fixture]
fn setup() {
	color_eyre::install().expect("color_eyre installed");
}
```

notes:
Here's our test module setup.

Everything you see in this video will be inside this module.
All of my code examples are concatenated into main.rs and compiled, this is how I statically type my literate programming videos.
(see my repo for details)

I'm importing the `rstest` prelude, which pulls in test attributes such as `fixture`, here, which runs this line ahead of every test, setting up `color_eyre`'s pretty errors.

---

### Functional features every language has

1. First-class functions
2. Anonymous functions (lambdas)
3. iterators
4. `map()`, `filter()`, et al

notes:

Most modern languages have these functional-light features.
Javascript, Go, Python, and Ruby all have most of these.

They are hugely useful, and Rust, of course, has them all. But Rust has a killer feature.

Rust has Pure functions.

Kindof.

---

# Pure Functions

- Deterministic
- Side-effect-free



```rust[]
fn f(x: i32) -> i32 { x + 1 }
```



notes:

Pure functions are WILDLY useful.

Normal functions can do anything, which means you have to read the code to figure out what they are doing.

Pure functions' utility comes from their limitations, they can't do everything:

They are functions where their output only depends on their inputs, and they have no side-effects (eg. memory or I/O)

If your language has a way of separating or tagging functions that are pure, and then can hold you to that contract, both you and the compiler can reason about your code in useful, new ways:

- if a pure function is called twice with the same inputs, the result is guaranteed to be the same every time
    - this is called referential transparency or idempotence or determinism
	- this enables perfect, predictable, caching,
	- and easier debugging!
- Pure functions that don't cause side-effects also allow perfect parallelisation, the functions can run on separate threads, processes, machines, or even in different geographic datacenters.
	- Their output is only affected by their inputs, so you bundle the inputs with the functions, and they can run anywhere.
- if a pure function is never called, it can be automatically removed by the compiler, or ignored by the developer.

Early on in development, Rust had a `pure fn` system, just like Haskell! 
- [ ] look this up maybe here https://internals.rust-lang.org/t/const-pure/13824
It was abandoned because it was soon discovered that the language had already solved many of the problems that a pure function system would guard against:

The compiler and developer don't usually need further function annotation to understand the side effects of functions, the type system and language ALREADY encode that to an enormous extent.

But if you really want a function purity system in Rust, like I do, there kindof is one.

If you've set up clippy like I recommend, this rust function here will not compile.

---

```rust[]
$ cargo clippy

 1  error: this could be a `const fn`
   --> src/main.rs:20:1
    |
 20 | fn f(x: i32) -> i32 { x + 1 }
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

The fix:

```rust
  const fn f(x: i32) -> i32 { x + 1 }
```

notes:

Clippy tells us that that this function could be a `const fn`.

This error was my first clue that rust is doing something interesting.

The fix, as ever, is to do what clippy says, obey the compiler and turn the function into a const function.

---


![[nb-teepublic-error-obey.png|700]]

note:

(stupid "obey the compiler" merch available at noboilerplate.org)

---

(previous slide)

notes:

const functions are functions that can be executed at compile time.
They differ from rust macros, which can do anything, by being much more limited.

And just like with Pure functions, these limits make them exciting:

---


<i class="fas fa-quote-left fa-2x fa-pull-left"></i>
_Behaviors such as out of bounds [array indexing](https://doc.rust-lang.org/reference/expressions/array-expr.html#array-and-slice-indexing-expressions) or [overflow](https://doc.rust-lang.org/reference/expressions/operator-expr.html#overflow) are compiler errors if the value must be evaluated at compile time._

&mdash; [rust-lang.org/reference/const_eval.html](https://doc.rust-lang.org/reference/const_eval.html)


notes:
`const` functions have access to a limited subset of Rust, you could read about it here, but let's experiment:

---

## Pure-ish Functions in Rust


```rust
const fn working(x: i32, y: i32) {
    x + y;
    x / y;

}
```


notes:

So what can we do in a const function, and perhaps more importantly, what can we not do?

12:00

- arithmetic operators on int and floats
- tuples
- arrays
- structs
- let assignment
- array slicing with usize
- range expressions
- closure expressions without capturing
- shared borrows, except interior mutability
    - test this
- casting
    - except for casting to memory addresses
- calling const functions
- loop, while, while let
- if if let and match
    - no for yet, though it's being worked on



---

```rust[]
const fn circle_area(radius: f64) -> f64 {
    (std::f64::consts::PI * radius).powf(2.0)
}
```

```rust[]
 1  error[E0658]: floating point arithmetic is not 
    allowed in constant functions
   --> src/main.rs:27:5
    |
 27 |     (PI * radius).powf(2.0)
    |     ^^^^^^^^^^^^^
```




---

```rust
//#![feature(const_for)] set at the crate level

const fn non_const_fn() {
	for n in 1..101 {}
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

https://lib.rs/crates/const_soft_float

---


```rust
const fn be_careful_with_macros() {
    std::env!("PATH");
}
```

Rust's const functions are only pure once you get to *runtime*

If you understand this caveat, you can get enormous benefit from these pure-ish functions.

---


so what should be const fn?

vital business logic

---

piecemeal purity

type signature encodes most things

> I just want to reiterate something that I say in a child comment in this thread: thanks to its thoughtful design, many of the advantages of purity are less appreciable in Rust than they would be in, say, C++.
> 
> In Rust, everything is immutable unless you opt-in to mutability. Looking at a function signature will tell you which of its arguments can possibly be mutated. Global mutable state is _highly_ discouraged, by requiring you to wrap code that accesses global mutable state in a dreaded `unsafe {}` block. As for optimization capabilities, LLVM itself can (AFAIK) infer when functions are "pure" and mark them as `readonly` or `readnone` (not sure what the limitations to this approach are, though).
> 
> So don't make the mistake of thinking that Rust is a free-for-all due to the long-ago removal of its `pure` keyword. Many (dare I say a majority?) of the nice features of purity are merely provided by different mechanisms (for those use cases that Rust does not satisfy, Graydon's own explanation should suffice regarding their enormous added complexity in a language that is not Haskell).

---


- ✅ Deterministic (at runtime)
- ✅ Side-effect-free

notes:

Are const fns pure?

No.

They're better than that.

They're practical

Rust is as pure a language you can get without adding an actual purity system!

(mention Nim)

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

I'm very interested in typed lambda calculus. It's a shame that the lisp community are so against typing, they're real dynamic proponents! I sadly left the Clojure world because of this. Typed Clojure exists, but it suffers from the same problem Typescript does: It's a rarely-used afterthought.

My primary goal is to make things, to build a team, to change the world in a tiny way with programs that make people's lives better.

As Dijkstra said, "A program is like a poem, you cannot write a poem without writing it."

The worst-kept secret in software development is that we're more like poets and artists than masons and bricklayers. You ask a poet how many poems she will write in a year, and she truthfully says, "I cannot know".

So, though learning pure ML and FP languages has enriched my thinking a great deal, to get things done, I am obliged to compromise. Rust's compromises are much smarter to my mind than other popular languages because if you add too much FP, you get languages like Haskell and Lisp without wide adoption. But if you don't add enough, you end up with languages like Go, a very good modern language, but with no beauty and (for reasons perhaps best known to Ken Thompson) no sum types.

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

generators

```rust[]
(0..).map(|x| x * 2)
```

and 

https://doc.rust-lang.org/reference/expressions/operator-expr.html#lazy-boolean-operators

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




```

```rust
fn laziness() {
    let _ = (0..).map(|x| x * 2);
}
```

---


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
