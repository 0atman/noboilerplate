<style>
.reveal code.rust {
  font-size: 1.5em;
  line-height: 1.5em;
}
</style>

![[rust-logo.png]]

# Stop Writing Rust
### And go outside and play

notes:

Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

When I build an application in Rust, I feel more confidant that it will work now, and for all time, than in any other language.

There's not just one reason why I feel like this, many parts of the language come together to make this feeling concrete.
Some of these features are just good practice in other languages, but Rust makes them mandatory.
Many of these features appear to be unique to Rust, among popular languages.

Today I'm going to talk about my observations of how in other languages it's easy to start projects, but in rust, it's easy to finish them

#tasks
- [x] build a bridge and it then gently rusts over decades
- [x] (about how there's no rust2.0 and how you can write code once and have it work forever)
- [x] Rust respects your time SO MUCH

---


# I want to be able to FINISH PROJECTS

notes:

When I chose programming languages, frameworks and ecosystems, I'm looking for one thing these days.

I want to be able to finish projects.

I want to:
Write a working app
quickly enough
correctly enough
and all while responding to the inevitability of change 

After I have deployed I want to KNOW FOR SURE:
- That it does what I want
- Won't crash, under any reasonable circumstances
- And is perennial - I or whoever maintains the project can come back and change some text in 4 years and it'll build fine

We all share this goal of aiming to finish projects, it's why we started building them to begin with.

---

# Fast 
# Reliable
# Productive
(pick THREE)

notes:

You'll perhaps have heard this.
These are the three core pillars of Rust, and they all let me finish projects.

Lets talk about the first pillar:

---

# Fast

| Language   | Time |
| ---------- | ---- |
| C          | 1x    |
| Rust       | 1.1x  |
| Java + JS       | 4x    |
| Ruby       | 20x   |
| Python     | 80x     |


notes:

Rust is the fastest high-level language on the planet.
Look at the sourcecode of this video for my source for that.
https://github.com/kostya/benchmarks

And due to rust's zero-cost abstraction principles, modules, generics, iterators etc do not cause additional runtime overhead.
This means as your application expands, it does not inevitably slow down.

This speed is life-changing.
Or at least, paradigm-changing.

Compared to a python app you require about 72x less compute resource.
You very realistically might not need a complex scaling plan if you write your application in rust.
It's like you've already scaled up 72x!

(multi region is still probably a good idea for when AWS has what they call an "unplanned thermal event")

---


# Reliable from
# the start

notes:

Code you write in rust is reliable from the start, due to the way the language is deigned.

All the things I've talked about in previous videos come together here.
- The unsafe system
- Macros
- The rich type system
- The cargo build system
- The community, focused on correctness

---

## `TypeError`

`Cannot read property of undefined`


notes:

When you program in rust, more than any other popular language, you can be sure that what you have built works, all the bullshit that we're conditioned to accept from other popular languages mostly just doesn't happen

---

| Name          | last updated | downloads   |
| ------------- | ------------ | ----------- |
| `rand_core`   | `1 year`       | `124,840,930` |
| `scopeguard`  | `2 years`      | `‎ 68,250,115`  |
| `cfg-if`      | `2 years`      | `120,569,601` |
| `lazy_static` | `3 years`      | `‎ 92,530,206`  |

notes:
As an illustration, here is a selection of top Rust crates.
Half of these are in the top 10 downloads on crates.io, they have been used millions and millions of times.

But look at their last updated dates.
They're not abandoned.
They're DONE.

This trend in Rust is caused, I believe, by two main forces.
Backwards compatibility and correctness.

---

# No Rust 2.0
## Is about backwards compatibility
notes:

In previous videos I said that there will be no Rust 2.0 due to the macro system. Now, I talked with a Rust maintainer, and they said this actually misses the point:
Code written today will be compileable in 5, 10, 40 years because of the Rust team's commitment to perfect backwards compatibility.
Code you write today, will always compile in all future versions of rust.
The side effect of this is that code you build today benefits from ALL FUTURE OPTIMISATIONS that the rust toolchain will receive.
With no modification by you, your build times and deployed code will get FASTER.

---

# Correct from 
# the start

notes:

The second big reason rust code doesn't need modification after it's published is the same reason the learning curve is steeper than for other popular languages.

The Rust compiler enforces many things that are just good practice in other languages.

A few simple examples of this are

---

## No unused variables

```js
warning: unused variable: `username`
  --> src/main.rs:11:7
   |
11 |   let username = "tris";
   |       ^^^^^^^^ help: if this is intentional,
   |       prefix it with an underscore: `_username`
   |

```

notes:

no unused variables, that you might have forgotten to use.

---

### Exhaustive Pattern Matching 

```js
  let boolean = true;
  let binary = match boolean {
      false => 0,
      //true => 1,   <- missing the `true` case
  };
```

```js
error[E0004]: non-exhaustive patterns: `true` not covered
  --> src/main.rs:12:22
   |
12 |   let binary = match boolean {
   |                      ^^^^^^^ pattern `true` not covered
   |
   = note the matched value is of type `bool`
```

notes:

Exhaustive pattern matching. In the same vein, Rust will tell you when you've forgotten to handle a match statement exhaustively.
This is actually a compiler error, not just a warning.

In this example it's easy to keep all branches in your mind with just two values for boolean.
But when you have modelled the entire valid state of your system with lightweight structs, using match will guarantee at compile time that you and everyone who contributes to your code, has not forgotten anything.

---

```rust
// Suppose channel: Channel<Vec<String>>

let mut users = Vec::new();
// append some usernames

channel.send(users);
print_vec(&users);  // used after move 
```

Results in:

```
Error: use of moved value `vec`
```

notes:

Correct concurrency.

Once you've sent a variable to another thread or channel, it's GONE. It is a compiler error to even read it.

We call this compiler-checked behaviour "Fearless Concurrency"

---

## Errors must be handled

Quickly

```js
let mut file = File::open(&path).unwrap()
```

```js
let mut file = File::open(&path)
                     .expect("this file MUST exist")
```

Comprehensively 
```js
let mut file = match File::open(&path) {
    Err(why) => panic!("can't open {path.display()}: {why}"),
    Ok(file) => file,
};

let mut file = File::open(&path)?; // to return the Err
```

notes:
You can either quickly tell the compiler that you KNOW the result might be a failure, and use .unwrap(), just to get something working now, optionally enriching the crash with an error message with .expect(), or you can handle the error comprehensively.

When writing rust, .unwrap() is for prototyping code, so we don't have the annoyance of heavyweight error handling when we just want to get going. 
This is why most code exmaples you will see use .unwrap().
They're not trying to teach you error handling, they're just showing you how to open a file.
In other languages these kinds of runtime pitfalls are hidden, at best by an exception that you have to catch, or worst, with no visible indication that the code may crash at all.

Rust makes it clear: If you want to write perfect code, find alternatives to those `.unwrap()`s

Rust doesn't just give you tools to write code that doesn't break at runtime.
It gives you an escape hatch to write quick code that is CLEARLY SIGNPOSTED as unreliable to anyone auditing the code.

This kind of balance of safety and pragmatism leads me to the final pillar of Rust: Productivity.

---

# Productive

notes:

This third pillar is where the magic lies, I think. 
Fast code is fast code.
Reliable code just works.
But productivity is nuanced.

Rust respects your time SO MUCH.
Many popular languages have great productivity.

I keep saying "popular languages"
Let me take a quick detour to address popularity first.

---

# Popularity


![[redmonk-language-rankings-jan-2022-bottom10.png]]
notes:

Here are the latest RedMonk popularity rankings, according to github projects and stackoverflow tags.
For space, I've cut off the top 10. 
Don't worry, you're not missing anything, it's just the usual suspects, like languages with java in their name.

No matter how theoretically good a language is, if I can't hire a team, can't buy books, can't train myself and others, it's a non-starter.

Languages such as Haskell, Elixir, Common Lisp, Julia and others, are VERY EXCITING, and have lots of really great features that I'm excited to learn.
But they don't help me with my goal of finishing projects for this reason.

Rust slipped into Redmonk's top 20 language rankings in mid 2020, right around the time I started learning Rust.

The timing is not a co-incidence.

Let's get back to productivity.

---

# Fast Prototyping
notes:

Many popular languages are great for fast prototyping, and rust is no exception, though you might be surprised to learn this.

Fast prototyping is about getting fast or instant feedback. You can do this in a number of ways.

I will talk about three.

---

```rust
   >>> "python"
```

```rust
  irb> "ruby"
```

```rust
     > "javscript"
```

```rust
user=> "clojure"
```

```rust
scala> "scala"
```

tag yourself

notes:

Firstly you can do this in a language with a REPL, or a shell.
You can do repl-driven development by loading data and functions into the runtime and modifying them iteratively, based on feedback from multiple experimental executions.
If this is a new technique for you, talk to your local Data Scientist to learn more.

Once you have a good idea of what you need, you persist your working code into your source files. 

The ultimate expression of this is languages where you can save the state of the runtime, Lisps often can do this for example

---

![[hoplon.png]]

notes:

Secondly, you can do it in-browser, with languages and frameworks that re-load source files every page load, or re-build files when they detect changes.
Think React, ruby on rails, django, express. Things like that.
Make a change, hit F5. Some like this clojure framework, even hot-load the code into the browser without a reload. quite a trick.

But there is a third way to get feedback: 
Compiler experimentation.

---

# Type system
## With superpowers

notes:

Rust's rich algebraic type system has lifetime annotations. 
This was designed so that memory can be cleaned up when a variable falls out of scope.
BUT IT ALSO GIVES YOU SUPERPOWERS

Superpowers I will have to explain in detail in a dedicated video.

The short version is that if your type system has lifetime annotations, you can describe not just what your data is, but WHEN.

Lets see an example of compiler feedback.

%%Rust's bootstrapped in C, and C's bootstrapped in assembly, and assembly was toggled in on switches by the The True Wizards Who Know Everything.

You know what makes me feel stupid? Synchronising gears on an old gearbox - everyone looks at me as my car makes terrible noises.
A modern gearbox makes me feel like a genius because the car just does what I want.

Your faith in the intelligence of C developers is not shared by Microsoft, Google, or the OpenSSL team. They experience the growing pains of the language's flexibility:
Once your program becomes non-trivial, it appears to be impossible to avoid memory nightmares, which manifest as security holes so terrible that we give them names:
Slammer worm, WannaCry, Triden exploi, HeartBleed, Stagefrieght, Ghost%%


---

```js
  let boolean = true;
  let binary = match boolean {
      false => 0,
      //true => 1,   <- missing the `true` case
  };
```

```js
error[E0004]: non-exhaustive patterns: `true` not covered

17 |   let binary = match boolean {
   |                      ^^^^^^^ pattern `true` not covered
   |
   = note the matched value is of type `bool`
help: ensure that all possible cases are being handled
      by adding a match arm with a wildcard pattern 
      or an explicit pattern as shown
   |
18 ~       false => 0,
19 ~       true => todo!(),
   |

```

notes:

Remember this boolean match error from earlier?

This is an example of the conversation you can have with the compiler as you work.
You try something, and it might need tweaking to be safe or complete, but the compiler very often tells you exactly what to do to fix it.

It feels a lot like test driven development - red, green, refactor - but the tests have already been partially written for you.

In previous videos I likened it to a driving instructor, gently guiding you how to navigate the dangerous highway.

It's not just possible to write perfect safe code with Rust. It's actually easy.

---

## WERID ANALOGY TIME

notes:
Let me finish with a real-life analogy of what it feels like to me to deploy Rust 

---


![[saltash-steamtrain.png]]

notes:

This is the Royal Albert Bridge, the final rail link between London and Penzance, opened in 1859.

---
![[saltash-viaduct-D8961A.jpg]]

notes:

It's so old that it pre-dates cars.

---


![[brunel.png]]

notes:
It was designed by the original hipster, Isambard Kingdom Brunel.

---

![[Bridges_18.png]]

notes:

Planned carefully

---

![[1854_at_the_Royal_Albert_Bridge_-_sinking_the_central_pier.jpg]]

notes:
Built to specification

---

![[saltash_bridge.jpg]]

notes:

And now it is solid infrastructure, and is gently, safely, rusting.

---

![[Royal-Albert-Bridge-train-landscape-6-e1479300780790.jpg]]

notes: 
It brought my father to London in the 60s, and just a few weekends ago I used it to make another return journey to visit him.

---

![[saltash road.jpg]]

notes:

There's an ugly 70s concrete road bridge next to it these days.

But I know which one probably still be standing in ANOTHER 161 years.


---



![[rust-logo.png]]

# Stop Writing Rust
### And go outside and play

notes:

With Rust, when you code compiles, check it in: You're done.
And can finally finish your projects and go outside and play.

%%%%

If you'd like to see what you can write in rust, click the top video: I used it to make a fun retro computer visualisation for my scifi and mental health podcast, Lost Terminal.

And if you'd like to watch more of my fast, technical videos, click the bottom video.

Transcripts and markdown sourcecode are available on github, links in the description, and corrections are in the pinned comment.

Thank so much for watching, see you next time.
