<style>
:root {--r-code-font: "FiraCode Nerd Font";}
.reveal .hljs {min-height: 50%;}
</style>
%%

# Cargo.toml

```toml
[package]
name = "rust-testing"
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
```

# Extern Crates

```rust

```

# Imports

```rust
```

# Setup

```rust
fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
	println!("Rust Testing");
	assertion_test();
	Ok(())
}
```

%%

![[rust-logo.png]]

# RUST Tests Itself

%% Kind of %%
notes:

Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

Today we're going to talk about how to test Rust, and how this is different to other popular languages.


---

> If debugging is the process of removing software bugs, then programming must be the process of putting them in.

&mdash; Dijkstra

notes:

Rust is so reliable because if used correctly, whole categories of bugs are impossible to express.

For the remainder of the bugs that ARE possible to express, you will indeed need to test.

---

| Happy        | Unhappy   | Random     | Proxy    |
| ------------ | --------- | ---------- | -------- |
| Assertations | Black Box | QuickCheck | Mocking  |
| Doctests     | White Box | Proptest   | Doubling |
| Examples     | Compiler  | Fuzzing    | Contract |

notes:

Happy path
- assertion
- doctests
- examples

Unhappy path
- black box test in `tests/`
- white box test in-file using mod test with cfg(test) conditional compilation
- compile failures using `compiletest`

Random
- quickcheck
- proptest
- fuzzing

proxy
- Mocking with mockall
- Doubling
- Contract tests

we have a lot of ground to cover today, I won't spend more than a minute on each of these, with crate recommendations and tips.

As ever, I have exceedingly good news for you.

---

# Rust Tests

# Are MAGIC

notes:

Rust tests are another example of why we accept more syntax in Rust than in other languages.

Code is only boilerplate when it doesn't give us anything.

Rust syntax gives us superpowers.

---

# Happy Path Testing

notes:
Let's start simple with happy path testing.
These are minimal sanity checks you might do anyway, poking your code with sensible data, and ensuring it does the right thing

---

## Assertions

```rust
fn assertion_test(){
	assert!(0 == 0);
	debug_assert!(0 == 1);
}
```

```md
The application panicked (crashed).  
Message:  assertion failed: 0 == 1  
Location: src/main.rs:12
```

```toml
color-eyre = "0.6.2" # if you want coloured errors
```

notes:
Assertions are the bread and butter of code testing, we've used them in every language, and they're built in to Rust.

---

## Doctests

---

## Examples

---

# Unhappy Path Testing

## Black Box

---

## White Box

---

## Compile Failures

---

# Random

## Quickcheck

---

## Proptest

---

## Fuzzing

---

# Proxy

- [ ] better name?

## Assertations

---

## Doctests

---

## Examples

An example:

---

```python
def hello(name):
	return "hello " + name
```

notes:

If we want to generate random test input for this hello function in python, we still have work to do.
This is because we don't know what kind of data the input to the function is, the `name` parameter could be anything.
You and I might reasonably guess it's a string, but it could equally be an object, list, or even an integer.
More work is needed.

---

```rust
fn hello(name: String) -> String {
	format!("hello {name}")
}
```

notes:

With Rust, we know exactly what to do, and more importantly, the compiler also knows exactly what to do.

The name is a String, which in Rust means a valid UTF-8 string, and not only that, we also notate the return value.
This is something we would have had to infer or annotate from the python example.

Here we have more syntax, but it's not boilerplate, we can now use it to give us superpowers.

---

# Proptest

```toml
proptest = "1.0.0"
```

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn hello_with_strings(a: String) {
        hello(a);
    }
}
```

notes:

This is proptest, a property testing framework inspired by the [Hypothesis](http://hypothesis.works/) framework for Python.
It allows us to test that certain properties of our code hold for randomised inputs, and if a failure is found, it automatically finds the minimal test case to reproduce the problem.

Note that Proptest is taking advantage of two features of Rust that are not available in other popular languages:
1. The rich algebraic type system, and
2. Macros

---

```rust[1]
fn hello_with_strings(a: String) {
	hello(a);
}
```

[crates.io/crates/proptest](https://crates.io/crates/proptest)

```python[1]
@given(text())
def hello_with_strings(a):
    hello(a)
```

[pypi.org/project/hypothesis](https://pypi.org/project/hypothesis/)

notes:

Proptest, and the python test framework it is taking inspiration from, Hypothesis, are extremely similar in operation.
But Proptest requires no wrapping of the test function at runtime, due to Rust's type system already encoding that data.

In the python example, the Hypothesis framework must be told what kind of data `a` contains, text, for it to be generated.

Note that even this simple example is wrong, in Python's case.
`a` is supposedly text, but it could be any type at run time, python makes no guarantees.

The tests assumes `a` will quack like a string when it is in use. Rust, however, guarantees it.

---

- [ ] proptest-regressions

---

# Clippy

```sh
cargo clippy --fix -- \
-W clippy::pedantic \
-W clippy::nursery \
-W clippy::unwrap_used \
```

notes:

When used right, rust's built-in linter won't just make your code cleaner and more idiomatic, but with unwrap_used, here, safer and more correct, too.

The unwrap_used warning reminds you that while `.unwrap()`ing a result is fine for prototype code, you must not let it creep into your production code.

More details in my previous video about Rust errors.

---

```sql
$ cargo clippy --fix
```

```md
error:
the working directory has uncommitted changes,
and `cargo fix` can potentially perform destructive changes
if you'd like to suppress this error pass `--allow-dirty`,
`--allow-staged`, or commit the changes to these files
```

notes:

If you run clippy with --fix, which can change your code if it is safe to do so, by default you will see this warning if you are not checked-in in version control.

The cargo developers really have thought of everything!

---

%% (this is commented out to keep the ad spot under 1 minute, sdaly!)

# SPONSOR QUIZ TIME

- Old hardware
- 100 MPH
- Dynamic Data Centre Combinations
- No grid power
- No fibre internet

notes:

- What data centre contains 100 decade-old servers
- re-configures its entire network when it merges or splits with another data centre
- must operate without a reliable power source or internet connection,
- And travels at over 100 MPH?

To answer this, I will introduce Today's sponsor, Razor Secure
%%

<!-- slide bg="[[rs-train.jpg]]" -->

![[rs-logo.png|400]]
notes:
_(disclosure: The company's CTO is my brother!)_

- As with previous sponsors, Razor Secure don't want your money, they actually want to pay you.
- This is because they've asked me to tell you about their open full-stack positions at their company.

---

<!-- slide bg="[[rs-train.jpg]]" -->

![[rs-logo.png|200]]

1. Cross-platform Rust Agent
2. Cloud microservices
3. Embedded hardware platform

notes:

- RazorSecure is a 50-person startup bringing cutting-edge security tech to the rapidly-advancing world of trains
- They do this through:
   - A Rust intrusion detection and monitoring agent running on whatever hardware is installed on-board.
   - A cloud environment running K8s, Python microservices, and event-based data processing, and
   - A yocto hardware platform running custom embedded linux.
- Their team members and customers are based across Europe and North America, so if you have taken a train journey in any of those areas you may have already been kept safe by their security systems.

---

<!-- slide bg="[[rs-train2.jpg]]" -->
<split even>

![[Python_logo_icon.png|200]]
![[kubernetes-logo.png|200]]
![[rust-logo.png|200]]

</split>

notes:

- If you are a Python full-stack developer and are excited by this challenge and stack, then they are VERY interested in speaking to you as they are hiring NOW.
- The company is fully remote, so wherever you are based they offer challenging work in an interesting field with some awesome technology and a dynamic team.

---

![[rs-logo.png|200]]

[RazorSecure.com](https://www.razorsecure.com/)

[RazorSecure.com/careers](https://www.razorsecure.com/careers)

notes:

Find out more about RazorSecure at RazorSecure.com, and see their
open positions at RazorSecure.com/careers, and remember to mention No Boilerplate as your referrer so they know I sent you.

My thanks to RazorSecure for their support of this channel.

---

---

![[rust-logo.png]]

# Subtitle

notes:

# OUTTRO

If you would like to support my channel, get early ad-free and tracking-free videos and vip discord access head to patreon.com/noboilerplate.

If you're interested in transhumanism and hopepunk stories, please check out my sci-fi podcast, Lost Terminal.

Or if urban fantasy is more your bag, click the bottom video to listen to a strange and beautiful podcast I produce called Modem Prometheus.

Transcripts and compile-checked markdown sourcecode are available on github, links in the description, and corrections are in the pinned ERRATA comment.

Thank you so much for watching, talk to you on Discord.
