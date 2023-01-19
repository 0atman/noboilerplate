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


# Don't 
<!-- element style="font-size: 4em;color: white; text-shadow: 5px 5px black;" -->

# Test Me
<!-- element style="font-size: 4em;color: white; text-shadow: 5px 5px black;" -->

---

![[rust-logo.png]]

# RUST Tests Itself

Compiler-checked markdown video scripts:
[github.com/0atman/noboilerplate](github.com/0atman/noboilerplate)

%% Kind of %%
notes:

Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

Today we're going to talk about how to test Rust, and how this is different to testing in other popular languages.

---

> If debugging is the process of removing software bugs, then programming must be the process of putting them in.

&mdash; Dijkstra

notes:

Rust is so reliable because if used correctly, whole categories of bugs are impossible to express.

For the remainder of the bugs that ARE possible to express, you will indeed need to test.

---

| Happy Path   | Comprehensive | Probabilistic |
| ------------ | ------------- | ------------- |
| Assertations | Black Box     | QuickCheck    |
| Doctests     | White Box     | Proptest      |
| Examples     |               | Fuzzing       |

notes:

we have a lot of ground to cover today, I won't spend more than a minute on each of these, with crate recommendations and tips.

As ever, I have exceedingly good news for you.

---

# Rust Tests

# Are MAGIC

notes:

Rust tests are another example of why we accept more syntax in Rust than in other languages.

Code is only boilerplate when it doesn't give us anything.

Rust's syntax gives us superpowers because the COMPILER can do so much work for us.

---

```rust
// Create an `enum` to classify a web event
enum WebEvent {
    // An `enum` may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
}
```

(from _Rust By Example_)

notes:
Here we are modelling a browser web event.
We have variants for the page loading and unloading, and user interaction.

In enums, names and type information together specify the variant:
PageLoad != PageUnload and KeyPress(char) != Paste(String)`.
Each of these enumerations is independent, and the compiler knows they mean different things.

---

```js
// A function which takes a `WebEvent` enum as an argument
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum`.
        WebEvent::KeyPress(c) => println!("pressed {c}."),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={x}, y={y}.");
        },
    }
}
```

notes:

Then in our code, instead of writing spaghetti if statements, we match the current state of the application, and execute different behaviours based on this state.
No other actions are possible, because we're responding to exactly the state we are given.
If more nuance is needed, then don't write an if statement, add more detail to your model.

This keeps us safe.

You might not have noticed that we've not handled all cases of the WebEvent enum.
Paste is unhanded.
It's easy for a human to miss, even in this little example.

But the compiler didn't miss it.

---

```js
error[E0004]: non-exhaustive patterns:
  --> src/main.rs:24:11  
|
|    match event {  
|          ^^^^^ pattern `WebEvent::Paste(_)` not covered
  
help: ensure that all possible cases are being handled by
adding a match arm with a wildcard pattern or
an explicit pattern as shown
| 
|        WebEvent::Paste(_) => todo!(),  
|
```

notes:

If you model your whole application state using enums and structs it becomes very difficult to make logic errors.
The incredibly thorough compiler, powered by the extra syntax Rust has compared to other languages, allows us to express our intentions in a machine-readable way.

I explained this in more detail in my previous video "Rust Makes Cents", so for now, we will move on from language to tooling.

---

# Clippy

```sh[4]
$ cargo clippy --fix -- \
-W clippy::pedantic \
-W clippy::nursery \
-W clippy::unwrap_used
```

notes:

When used right, rust's built-in linter won't just make your code cleaner and more idiomatic, but with the unwrap_used warning, here, safer and more correct, too.

The unwrap_used warning reminds you that while `.unwrap()`ing a result is fine for prototype code, you must not let it creep into production.

In fact, I recommend in your CI pipeline, you configure this warning to be an error, and fail the build.
Force you and your team to explain why they are so sure the result is safe, using the `.expect("reason")` method.

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

If you run clippy with the --fix option, which can correct your code if it is safe to do so, by default you will see this warning if you are not checked-in in version control.

The cargo developers really have thought of everything!

---

# Happy Path Testing

notes:
Let's start simple with happy path testing.
These are minimal sanity checks you might do anyway, poking your code with sensible data, and ensuring it does the right thing

---

## Assertions

```rust
fn assertion_test() {
	assert!(0 == 0);
	debug_assert!(0 == 1, "Maths is hard");
}
```

```md
The application panicked (crashed).  
Message:  Maths is hard
Location: src/main.rs:12
```

```toml
color-eyre = "0.6.2" # if you want coloured errors
```

notes:
Assertions are the bread and butter of code testing, we've used them in every language, and they're built in to Rust.

Note that here I'm using both the assert! macro, which always runs, and debug_assert!, which is bypassed in release builds. Due to the nature of the macro system, it is not even included in the final binary, causing no overhead in production.

On to my favourite kind of tests

---

## Doctest

```rust[]
/// ```
/// /// Some documentation for this function.
/// my_adder(1, 2)
///
/// use std::io;
/// let mut input = String::new();
/// io::stdin().read_line(&mut input)?;
/// # Ok::<(), io::Error>(())
/// ```
fn my_adder(x: i32, y: i32) -> i32 {
	x + y	
}
```

[doc.rust-lang.org/rustdoc/](https://doc.rust-lang.org/rustdoc/)

notes:

Doctests rule. Combining documentation and testing into one feature, it was my favourite in Python, and I'm delighted they're here in Rust.

I recommend choosing doctests for the lightest-touch testing, before moving on to the heavier strategies we're going to talk about later.

In your doctests, you can test the annotated function, see line 3, but also test anything you want to bring into scope, such as the read_line code in the second half of this doctest.

Note that you can avoid using a main method inside doctests by using the turbofish syntax for the ok result, as on line 8.

There's a few more cool features, head to the rust doc website to learn them.

---

## Examples

```clojure
bevy/examples    
❯ : ls  
  -  2022-12-28 11:27  2d  
  -  2022-12-28 11:27  3d  
  -  2022-12-28 11:27  android  
  -  2022-12-28 11:27  ui  
  -  2022-12-28 11:27  wasm  
  -  2022-12-28 11:27  window  
148  2022-12-28 11:27  hello_world.rs  
28k  2022-12-28 11:27  README.md  
9.8k 2022-12-28 11:27  README.md.tpl

❯ : cargo run --example breakout
```

[github.com/bevyengine/bevy](https://github.com/bevyengine/bevy)

notes:
(hello reader, the ls output is rendered by the crate exa, and my shell is nushell)

It can be useful, especially if you are building a framework or library to build some small, working examples of using your code.

examples that terminate after running, or can be made to terminate after a pause, in the case of UI or server code, can be run as part of your CI pipeline to ensure core functionality works as expected.
This technique can be a very powerful and fast way to avoid regressions when combined with assertions.

Testing our code works how we expect is only one side of the coin, of course.
For more confidence in our code, we must also show that it doesn't work how we don't expect so that bad actors or incorrect usage is handled correctly.

This is vital in public-facing and security-focused projects, like today's sponsor, Razor Secure, who are hiring.

---

# Razor Secure

---

<!-- slide bg="[[rs-train.jpg]]" -->

![[rs-logo.png|400]]
notes:
_(disclosure: The company's CTO is my brother!)_

A quiz: What data-centre:
- travels at 100 miles-per-hour
- re-configures every IP when it attaches or detaches to other data-centres
- all without a reliable power source or internet connection, and must never fail?

The Answer: A train.

---

<!-- slide bg="[[rs-train.jpg]]" -->

![[rs-logo.png|200]]

1. Cross-platform Rust Agent
2. Cloud microservices
3. Embedded hardware platform

notes:

- RazorSecure is a 50-person startup bringing cutting-edge security tech to the rapidly-advancing world of rail
- They do this through:
   - A Rust intrusion detection and monitoring agent running on-board.
   - A cloud environment running K8s, Python micro-services, and event-based data processing, and
   - A Yocto hardware platform running custom embedded linux.
- Their team and customers span Europe and North America, so if you have taken a train journey here, then RazorSecure's security systems may have already kept you safe.

---

<!-- slide bg="[[rs-train2.jpg]]" -->
<split even>

![[Python_logo_icon.png|200]]
![[kubernetes-logo.png|200]]
![[rust-logo.png|200]]

</split>

notes:

- If you are a Python full-stack developer and are excited by this challenge and tech, then they are VERY interested in speaking to you as they are hiring NOW.
- The company is fully remote, so wherever you are based they offer challenging work in an interesting field with some awesome technology.

---

![[rs-logo.png|200]]

[RazorSecure.com/careers](https://www.razorsecure.com/careers)

[razorsecure.noboilerplate.org](http://razorsecure.noboilerplate.org)

notes:

Find out more about jobs at RazorSecure at RazorSecure.com/careers, and if you want to apply, use the link, razorsecure.noboilerplate.org, so they know I sent you.

My thanks to RazorSecure for their support of this channel.

---

# Comprehensive Testing

notes:

You might be able to stop after testing just the happy path.

Not every project needs comprehensive testing, especially if you are just building on top of well-understood fundamentals.

A brochure website, for instance, if built in rust doesn't need to be tested through counter-example, the compiler has already told you that every page has valid html, no missing closing tags, no sql injection or memory bugs, and if using a rust frontend framework like Yew, the messages passed between your components are type checked and can't be misused as easily as in other language's string-based frameworks.

However, if you've got a complex database and you're building a big webapp, you will want test comprehensively.

---

## Black Box

```rust
#[test]
#[should_panic(expected = "InvalidDigit")]
fn bad_string() {
	"twenty".parse::<i32>().unwrap();
}
```

```sh
running 1 test
test bad_string - should panic ... ok  

test result: ok. 1 passed; 0 failed; finished in 0.15s
```

notes:
Unit tests can be divided neatly in two: Those that have no knowledge of library internals, and those that do.

Public interface testing, and private unit testing.

Black box tests in rust typically import a crate and use the same public API that end-users, or other modules of your app use.
Code examples use this method.

White box tests are defined in the same module as the code under test. We've already seen doctests use this method.

---

## White Box

```rust
pub fn my_function() {}
struct User {
	name: String
}

#[cfg(test)]
mod test {
    use super::{my_function, User};

    #[test]
    fn test_my_function() {}
    #[test]
    fn test_user() {}
}
```

notes:

While Black box tests reside simply inside the `test/` folder, white box tests can be defined alongside your code, in a submodule in the same file.

- [x] re-record this
Conditional compilation, the `#[cfg(test)]` line here, means

the whole module is stripped out of your release executable, only running in your debug test builds.

Here we have a public function, a private User struct, and then a submodule called test that contains our test code.

Though you are required to re-import any code you are testing into your test module, unlike in black box testing the functions and structs do not need to be labelled public.
Both this public function and private struct work fine.

---

# Probabilistic

```python
def hello(name):
	return "hello " + name
```

notes:
Probabalistic testing is a great way to shine a light into the dusty corners of our app that we may have forgotten about.

However, in other languages, it often requires boilerplate code.

If we want to generate random test input for this hello function in python, we still have work to do.
This is because we don't know what kind of data the input to the function is, the `name` parameter could be anything.
You and I might reasonably guess it's a string, but it could equally be an object, list, or even an integer.
In Python, more work is needed.

---

# Probabilistic

```rust
fn hello(name: String) -> String {
	format!("hello {name}")
}
```

notes:

With Rust, we know exactly what to do, and more importantly, the compiler also knows exactly what to do.

The name param is a String, which in Rust means a valid UTF-8 string, and not only that, we also notate the return value.
This is something we would have had to infer or annotate in the python example.

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

This is proptest, a property testing framework inspired by the [Hypothesis](http://hypothesis.works/) framework for Python, which was in turn inspired by QuickCheck for haskell.
It allows us to test that certain properties of our code hold for randomised inputs, and if a failure is found, it automatically finds the minimal test case to reproduce the problem.

Note that Proptest is taking advantage of two features of Rust that are not available in other popular languages:
1. The rich algebraic type system, and
2. Macros

---

```rust[1]
fn hello_with_strings(name: String) {
	hello(name);
}
```

[crates.io/crates/proptest](https://crates.io/crates/proptest)

```python[1]
@given(text())
def hello_with_strings(name):
    hello(name)
```

[pypi.org/project/hypothesis](https://pypi.org/project/hypothesis/)

notes:

Proptest, and Python's Hypothesis, are extremely similar in operation.
But Proptest requires no wrapping of the test function at runtime, due to Rust's type system already encoding that data.

In the python example, the Hypothesis framework must be told what kind of data `name` contains, text, for it to be generated.

In order to do this probabilistic testing, hypothesis had to overlay it's own propitiatory type system on top of python.

Note that even this simple example is kinda wrong, in Python's case.
`name` is supposedly text, but it could be any type at run time, python makes no guarantees.
We might not be testing the right thing!
In python, more work is needed.

The test *assumes* that `name` will quack like a string when it is in use. Rust, however, guarantees it.

---

## Fuzzing

```rust[]
#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate url;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = url::Url::parse(s);
    }
});
```

```js
$ cargo fuzz demo_fuzz
#58397 NEW    cov: 2069  corp: 111/4755b 
exec/s: 11679 rss: 176Mb L: 42 MS: 1 EraseBytes-
[...]
thread panicked at 'index out of bounds: [...]'
```

[rust-fuzz.github.io](https://rust-fuzz.github.io/book/cargo-fuzz.html)

notes:

The most comprehensive, though heavyweight, tool for this kind of randomised testing in Rust is cargo fuzz.

Cargo-fuzz uses LLVM's libFuzzer runtime library to generate psudorandom data and keep track of what has been tested.

This means that, unlike with proptest, you can stop the test, and resume it later, and it won't re-test the same randomised input twice.

This is important in fuzz testing very large systems, as the state space could be effectively infinite - the testing will never complete.

Resuming it later, perhaps on a powerful server, is very handy.

---

# Integration Tests

```rust[]
struct Country { country: String, count: i64 }

let countries = sqlx::query_as!(
	Country,
	"SELECT country, COUNT(*) as count
	FROM users
	GROUP BY country
	WHERE organization = ?",
	organization
)
    .fetch_all(&pool) // -> Vec<Country>
    .await?;
```

notes:

While it deserves it's own entire video, I will briefly mention integration testing here, because there's some magic still to be found.

You may recognise my favourite SQL framework for Rust, here, SQLx.

Both better than an ORM and a DSL, SQLx runs all your queries against your actual dev database during rust compilation inside a rolled-back transaction.
Parametarised data, like the string, `organization`, here, because it's type is known at compile time, can have random valid data inserted into the query.

This is all very wonderful and magic.
However, you have just coupled your application's compilation to an external service, the database, and therefore have complicated your building and testing infrastructure.

Or have you.

---

# Test Double

0. `cargo install sqlx-cli`
1. `sqlx = { features = ["offline"] }`
2. `cargo sqlx prepare`

<br/>

More info on doubling:

[martinfowler.com/bliki/TestDouble.html](https://martinfowler.com/bliki/TestDouble.html)

notes:

SQLx has an offline schema feature which is a great example of doubling, or mocking, or stubbing. You will have heard many names for this technique, but Martin Fowler calls it a Double, so that is good enough for me.

To double your SQLx database validation, and decouple it from a real database you do two things:

1. Enable the SQLx's Cargo feature `offline`
2. Save query metadata for offline usage using `cargo sqlx prepare`

And now any cargo Build will use the schema double saved to `sqlx-data.json` for compile-time verification

For your interest, this schema double looks like this:

---

```json
"3a39bed220618e2f59edb65dc9f8": {
"describe": {
  "columns": [
	{
	  "name": "id",
	  "ordinal": 0,
	  "type_info": "Int4"
	},
  ],
  "nullable": [false],
  "parameters": {
	"Left": []
  }
},
```

[github.com/launchbadge/sqlx/](https://github.com/launchbadge/sqlx/)

notes:

- [x] retake this screenshot

It's simply a description of your database's schema, and all features, in machine-readable JSON.

You'd check this in when you modify the database, probably when creating a migration, and then your CI pipeline can compile-check the project's SQL without a SQL database

It's also MUCH faster.

---

# Further Reading

> Do you [set your house on fire to test your smoke alarm?](https://dius.com.au/2014/05/20/simplifying-microservice-testing-with-pacts/) No, you test the contract it holds with your ears by using the testing button.

[crates.io/crates/pact_consumer](https://crates.io/crates/pact_consumer)

notes:

I am writing a follow-up video expanding on Integration, Doubling, and Contract testing, but for now, I'd point you at Pact, which has a comprehensive Reqwest contract tooling for microservices in Rust.

---

# Testing Rust

- Clippy
- Assertions
- Doctests
- Examples
- Black Box
- White Box
- Proptest
- Fuzzing

notes:

You will know how far down this list you need to go for your project and your team.

Integration and end-to-end testing will be next, in future video.

Rust and the community is extremely focussed on correctness, and that shines through in the testing ecosystem.

---

![[rust-logo.png|300]]


```php
$ cargo build  
   Finished target in 0.42s
```
<!-- element style="font-size: 1.16em;" -->

notes:

# OUTTRO

If you would like to support my channel, get early ad-free and tracking-free videos and vip discord access head to patreon.com/noboilerplate.

If you're interested in transhumanism and hopepunk stories, please check out my sci-fi podcast, Lost Terminal.

Or if urban fantasy is more your bag, click the bottom video to listen to a strange and beautiful podcast I produce called Modem Prometheus.

Transcripts and compile-checked markdown sourcecode are available on github, links in the description, and corrections are in the pinned ERRATA comment.

Thank you so much for watching, talk to you on Discord.
