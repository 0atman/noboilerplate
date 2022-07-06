![rust](attachments/rust-logo.png)
# Rust

Your code can be perfect

%%
As developers we build critical infrastructure,
it's time to build it in a language designed to build critical infrastructure.
%%

---

How often have you seen this kind of code?

```python
int(item["ViewCount"]["N"])
```

What problems are there here?

%%
(wait for suggestions)
You're all wrong, the biggest problem is that this line will page you at 4am.
%%

---

```python
int(item["ViewCount"]["N"])
```

We're making a lot of unsafe assumptions here, and everywhere in production:

1.  The `item` is a `hashmap`,
2.  There is a `ViewCount` attribute,
3.  The `ViewCount` attribute is a `hashmap`,
4.  There is an `N` attribute in the `ViewCount` hashmap, and
5.  The value can be parsed as an integer with `int()`.

%%
Unsafe assumptions crash our programs at runtime and wake us up at 4am.
%%

---

For your interest, though I'll explain the syntax later on, here is the least safe rust solution

```rust
i32::from_str_radix(item["ViewCount"].unwrap().get("N").unwrap(), 10).unwrap()
```

Note the `unrwap()s`.

%%
This is exactly as unsafe as the Python solution, but the three places it can crash are now explicitly stated.
If you want it never crash, you find alternatives for those .unwrap()s

Here's a verbose version of what that might look like
%%

---

A comprehensive rust solution

```rust
if let Some(view_count_attr) = item.get("ViewCount") {
    match view_count_attr.get("N") {
        Ok(view_count) => {
            match i32::from_str_radix(view_count, 10) {
                Ok(n) => n,
                Err(_) => {
                    // We couldn't parse the string as an i32
                }
            }
        }
        Err(_) => {
            // The 'ViewCount' was not an 'N'
        }
    }
} else {
    // There is no 'ViewCount' attribute
}
```

Rust will not let you write unsafe code from the start.
You must handle all errors.

---

An idiomatic rust solution

```rust
let viewcount = item.get("ViewCount")?;
let n = viewcount.get("N")?;
let parsed_n = i32::from_str_radix(n, 10)?;
```

Each `?` will immediately return the `Result` to the calling function, on error.


%%
Don't worry too much about the specifics here.
We'll talk about basic rust sytax soon.

I just wanted to give you a taste.
%%

---

Problems today:
- Security breaches
- ransomware attacks
- operational failures
- safety critical failures

%%
Broken software hurts people, and slows down human progress

We now have a much better solution to these problems.

Rust is a really pleasant language to write code in.
But that's not what we're here to talk about.
%%

---

We're here to talk about making perfect software.

---
%%
I've been searching for many years for systems, frameworks and methods to make my code more reliable, or guarenteed.
%%

I'm TIRED of trawling error logs for it to tell me: 
- `"unexpected ; in query"`, or 
- `json decoding error on line 1`
- `NullPointerException` (or `NoneType has no atribute`)

%%
We as developers accept that our lives are governed by errors.
Often bullshit errors like these.

I am here to tell you that it doesn't have to be that way.
%% 

---

## "Rust provides memory safety without a garbage collector"

%%
You may have heard that "Rust provides memory safety without a garbage collector".

I don't care about memory safety, I've never had to.

Java, python, ruby, node, go - they all ensure memory safety by running a second program alongside your program, checking memory is freed when it's not longer used.

This is called the garbage collector, as you know.

The Rust team identified the need for memory safety without a garbage collector as a key problem to be fixed.

So they implemented a genius simple method to keep track of memory, called the borrow checker.

But here's the thing:

%%

---

In fixing memory safety the Rust team
ACCIDENTALLY FIXED EVERYTHING  
  
(that I do care about)

---

In making a compiler that understands your code in a very deep way, and a rich type system that supports that compiler, they gave us, the developers, all of that control, the potential to build the perfect language and ecosystem.

The rust community, over 16 years, have delivered.

---
![[Screenshot from 2022-03-16 20-06-11.png]]

Tag yourself while we have a mid-presentation break.

In a moment we'll see some basic rust code.

%%
This is for me, not for you!
%%

---

## Familiar

```rust
println!("Hello world");
```


%%
Rust has a familiar c-like syntax that javascript, go and java developers will be familiar with. Even delicate python developers such as myself won't be too confused.
%%

---

## High level features / Low level speed

```rust
let sum_of_squares: u32 = (1..6).map(|x| x * x).sum();
```

%%
Here are functional-style iterators, but this is just one of the many zero-cost abstractions that translate down to simple loops

No matter how clever your language is, the processor running your code only understands bits and a few operators.

Rust gets your high-level code RIGHT DOWN to that bare metal without sacrificing high-level developer ergonomics.
%%

---

## Think In Expressions

```rust
// statements
if x {
    y = a;
} else {
    y = b;
}

// expression
y = if x { a } else { b };
```

%%
Semicolons finally have MEANING.

By the way, who here knows the origin of line-oriented statements?
Where do they come from?

It's punched cards.
Punched cards are statements.

We can do better.
%%

---

Think in Iterators

```rust

(1..10).map(f).collect()

names.iter().filter(|x| x.starts_with("A"))

```

%%

%%

---

## Options Everywhere

```rust
let possibly_a_number = Some(1);

possibly_a_number.map(|n| n + 1).unwrap_or(0);
```

%%
Use all the iterator operations on `Option`
No nulls in the whole language
%%

---

## Rich Types Make Invalid States Unrepresentable

```rust
enum Living { Alive, Dead }
enum Planet { Mercury, Venus, Earth, Mars, Jupiter, Saturn, Uranus, Neptune}
struct Human {name: String, state: Living, home: Planet}

let user = Human {
    name: "Tris",
    state: Living::Alive,
    home: Planet::Earth
}
```

%%
A human has to be alive or dead, and has to live on a planet. No nulls, no anonymous objects.

In Rust you tell the compiler how the world works, and it will hold you, and everyone who contributes to your code, accountable to the contract you have written.

This may be a new way of programming for you, but it's such a good pattern this is now how I try to write my python.
%%

---

### Modern Tooling

- **cargo** - packaging, building
- **cargo fmt** - standard formatting
- **cargo test** - doc and unit tests
- **cargo bench** - benchmarking
- **cargo clippy** - code linting
- **rustup** - rust version switching

%%
Rust has a best-in-class package manager, solving all the dependency nightmares we face day-to-day.

This is what you get when you have a community focussed on corectness.
%%

---

### Async syntax done RIGHT

```javascript
//js
const response = await fetch("http://example.com");
const data = await response.text();
console.log(data);
```

```rust
//rs
let data = fetch("http://example.com")
	.await
	.response
	.await
	.text;
println!(data);
```

%%
In JS if you remember, we had to wait YEARS for async/await to be standardised, and we still can't get it in all browsers.

In rust, it was a library.
%%

---

### No Rust 2.0

```rust
use html_macro::*;

fn main () {
    let name = "Tris";
    let page = html! { 
        <div id="component">hello { name }</div>
    };
    println("{}", page);
}
```

%%
The language is extended with macros, code that executes at compile time, which are installed as simple libraries, same as everything else.

Macros convert new syntax back to type-safe rust, which is then fed into the compiler.
You don't have to throw out the safety of rust to use new features TODAY

If you've use babel, webpack or the million other javascript precompilers, you've used a bad, error-prone, ill-defined, macro system.
%%

---

2nd act break.

Non-trivial code is coming.

I believe in you.

---


Let's look at some real-world rust.

```rust
#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
 return format!("Hello, {} year old named {}!", age, name)
}
```


%%
Here's how you write a simple get request in Rocket, a simple web framework.

You can probably read this without my help. It's a pattern we've all seen before.

The first line isn't a comment, it's a macro, which enriches and rewrites the enclosed function before the source code gets passed to the compiler.

It's a simple hello world http endpoint, with built-in compile-level validation of guaranteed-valid utf8 strings, and a rudimentary understanding that people shouldn't be negative years old, or over 255.

I'll admit, that's optimistic validation.

But we can do better, we have the technology.
%%

---

```rust
#[get("/published/<id>")]
async fn published_forms_by_id(id: Path<UUID>) -> FormResponse {
  return sqlx::query_as!(
      Form,
      "SELECT * FROM forms WHERE id=$1;",
      id
    ).fetch_one()
    .await
    .expect("");
```
%%
DON'T BE SCARED.
I promise we can get through this.
You're all the smartest people I've ever met, I believe in you.

Lets reason about this short piece of code.

It's still using the Rocket web framework, by the way, think of it as a sinatra or flask equivalent.

If our program compiles we know many things are guarenteed:
- `id` will be a valid UUID, from a valid http path
- The return json will ALWAYS be in the schema we designed,  named FormResponse, with defined values acting as the contract we can never break with our API clients.
- sqlx actually runs that query on my local dev database with a valid test input (based on the type) in a rolled-back transaction at compile time. If it is invalid, my code doesn't compile.
    - Yeah, this is magic. RUST magic.

So far so great, but there's more:
- No memory leaks
- No SQL injection - guaranteed at compile time
- And you get all of this with no heavyweight, slow abstractions - this all compiles down to for loops and if statements: close to C speed, running on bare metal.
%%
---

# #![forbid(unsafe_code)]

%%

But We can do even better:
- By adding the `#![forbid(unsafe_code)]` directive we forbid 
 the `unsafe` block, and therefore any linking to operating system libraries (ie, external c code), guaranteeing our app, and all its dependencies are pure rust, and therefore do not break any of our guarantees.
- But what about native libraries that you NEED
    - no Libpq-dev for postgres
    - no Pandas/Numpy
    - NOT EVEN openssl escapes oxidisation

%%

---

Rust is a language for the next 40 years

- High AND low level
- Code for web assembly, containers, or bare metal chips
- "Fast, reliable, productive — pick three"
- "Fearless concurrency"
- No Rust 2.0

%%
The last 40 years were written in c, the next 40 will be written in rust.
%%

---


In writing this, I wanted to invite you to get in on the ground floor with Rust, as it's going to be an industry-changing ride.

%%
However, I realised that actually you have some walking to do.

The Rust lift is currently on the 16th floor.
%%

---

![[lang-rank-0122-wm.png]]

%%
There are more rust projects on github than Scala, Kotlin, Swift, CoffeeScript and Perl.

It's time to take rust seriously, first in your personal projects and learning, and soon, at your work.
%%

---


![rust](attachments/rust-logo.png)
# Rust
Your code can be perfect

%%
Because, finally, our code can be perfect.
%%

---

