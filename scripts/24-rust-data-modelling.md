<style>
:root {--r-code-font: "FiraCode Nerd Font";}
.reveal .hljs {min-height: 50%;}
</style>
%%

f7f7f7 background slide colour

# Cargo.toml

```toml
[package]
name = "rust-data-modelling"
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
#![allow(unused_imports)]
```

# Extern Crates

```rust

```

# Imports

```rust
```

# Setup

```rust


```

%%

![[rust-logo.png]]

## Make invalid states
# unrepresentable

notes:

Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

one of my favourite features in rust is the rich type system
it doesn't just check you've used the right type of a a variable but allows you to build your own complex model with deep, rich integration with you problem domain 

Today I'm going to show you how to make invalid states unrepresentable using three techniques that align perfectly with Rust's powerful types and compiler.

We're going to talk about

- Algebraic Type Systems
- Data Normalisation
- State Machines

---

## Public Domain Videos

[github.com/0atman/noboilerplate/](https://github.com/0atman/noboilerplate/)

notes:
Everything you see in this video from the script to the images are part of a markdown document available on github under a public domain license.

---

```rust
struct Point {
    x: u32,
    y: u32,
}
```

```rust
enum WebEvent {
    Click(Point),
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
}
```

(adapted from [_Rust By Example_](https://doc.rust-lang.org/stable/rust-by-example/custom_types/structs.html))

notes:

Most popular languages have product types, the first example here of a Point is a product.
This is like a class or a c-style struct. It's a container for a number of attributes.

The second type here, WebEvent an enum, is NOT typically a core part of popular languages.
So-called Enums, in other language, are typically an enumeration over a set of integers, assigning them a name.
Bit flags or types of a message.

They're rubbish.

Rust enums are different. They are proper algebraic Sum types, sometimes called Tagged Unions.

---


```rust
struct FakeCat {
	alive: bool,
    hungry: bool,
}
```

```rust
enum RealCat {
    Alive { hungry: bool },
	Dead,
}
```

notes:

The difference between sum and product types is simple.

a Fake cat, here, could be written in any popular language with a normal class, or a struct in Rust.

It's fine, but requires runtime verification that a dead cat can't be hungry.

I'd be worried if it were.

It's the most natural thing to model this with Rust's fat enums, only the alive variant has the extra hungry attribute.

There's nothing to check with the real cat - the only two valid states are Alive and Dead, and because you've enforced this with the type system, you gain safety, guarantees, and superpowers.

---

```c
enum RealCat {
    Alive { hungry: bool },
	Dead,
}
```

```rust[]
let realcat = RealCat::Alive{ hungry: true };
match realcat {
	RealCat::Alive{..} => println!("good good")
}
```

notes:
for example, if you use enums, then the Rust compiler stops you making mistakes, this is not valid Rust here, I have made a mistake.

Can you see it?

---

```rust[]
let realcat = RealCat::Alive{ hungry: true };
match realcat {
	RealCat::Alive{..} => println!("good good")
}
```

```js
$ cargo build
non-exhaustive patterns: `RealCat::Dead` not covered
  --> src/main.rs:32:7
   |
32 | match realcat {
   |       ^^^^^^^ pattern `RealCat::Dead` not covered
   |
   = note, the matched value is of type `RealCat`
help: ensure that all possible cases are being handled by
adding a match arm with a wildcard pattern or an
explicit pattern as shown
   |
33 ~     RealCat::Alive{..} => println!("good good"),
34 ~     RealCat::Dead => todo!()
```

notes:
Unlike an if statement, which can do anything and is not bound to the type system, a match expression works with the type system.

I forgot to handle one of the two states. If I had used an if statement, or primitive types, I'd be none the wiser. 
Because I used an enum and match, the compiler knows what I am talking about, and kept me safe.

The compiler even told me how to fix my error, at the bottom here!

We're going to build our state machine using the match expression today, so let's look at what it can give us

---

```rust[]
let number = 13;

println!("Tell me about {}", number);
match number {
	1                  => println!("One!"),
	2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
	13..=19            => println!("A teen"),
	_                  => println!("Ain't special"),
}
```

(adapted from [_Rust By Example_](https://doc.rust-lang.org/stable/rust-by-example/custom_types/structs.html))

notes:

Rust provides pattern matching via the `match` keyword, which can be used like a C style `switch`. The first matching arm is evaluated, it short-circuits, and all possible values must be covered by the branches, this is enforced by the compiler.

Here we are matching on, in order:
- a value
- a set of values
- a range of values
- or wildcards

Note that we are using the runtime value inside the integer here. we can match on BOTH the compile-time type, AND the run-time value.

To get the extreme safety I talk about, you must enrich your model with the type system, not just use anonymous numbers and strings.

There's no runtime overhead in doing so, in rust, at runtime they are indeed just numbers and strings the cpu will be none the wiser.
It's one of Rust's many zero-cost abstractions.

But at compile time, where you and I exclusively live, we enrich the data with types.

---

## Part 1:
# Algebraic Type Systems

notes:

- OO is weird if you think about it

---


> The problem with object-oriented languages is they’ve got all this implicit environment that they carry around with them. You wanted a banana but what you got was a gorilla holding the banana and the entire jungle.

&mdash;**_Joe Armstrong_**, creator of Erlang

notes:

if you want a class, you'd better bring in ALL the parents of that class!
Reuse in object oriented systems is near zero.


---


![[24-rust-data-modelling 2023-04-21 12.20.56.excalidraw]]

notes:

Most object oriented languages solve the diamond problem by just not allowing multiple inheritance.
Oh great, what a clever solution.

WHO IS FLYING THIS THING.

we don't design database tables like this
we don't build companies like this

We've created through athrocentrism a system that doesn't model the world it just models cats and dogs and GENETICS, for some reason.

---

![[oo-told-benefits.png]]


[Goodbye, Object Oriented Programming](https://cscalfani.medium.com/goodbye-object-oriented-programming-a59cda4c0e53) 
</br>&mdash; Charles Scalfani

notes:

And shoe-horning our real world into an inheritance model causes real problems all the time.

Shrug off object orientation as Rust does, and you are left with a simpler, better world.


---

![[wait-its-all-enums.png]]


notes:

If you have an algebraic type system, like Rust has with enums, you just don't need any of the rubbish that object orientation loads you down with!


---

## OO features that can be replaced with enums

- Inheritance
- Hierarchies
- Containment
- Encapsulation
- Polymorphism
- ...is there anything else?


notes:

Smalltalk might have worked, I suppose, but the way OO is implemented in Python, Ruby, Javascript, Java and C++ is full of compromises to allow escape hatches out to the actual world of data and functions in order to work.

Why not just do Data and Functions REALLY REALLY well to start with?

Guess what, Rust does.

But how do we model our data, if not through classes and inheritance?
How can we replace a whole industry's experience designing data

My answer after a word from returning sponsor, Quadratic.


---


<!-- slide bg="rgb(37, 34, 43)" -->

![[quadratic-website.png]]

notes:

Quadratic are building an Open Source spreadsheet for engineers and data scientists, built in Rust, Webassembly and WebGL

---

<!-- slide bg="rgb(37, 34, 43)" -->
![[cell.png]]
notes:

Quadratic combines the functional data visualisation of a spreadsheet with the power of full programming languages, starting with Python

---


<!-- slide bg="rgb(37, 34, 43)" -->


![[quadratic-dataframe.png|700]]

notes:
Standard Python data science libraries are built-in.

---

<!-- slide bg="rgb(37, 34, 43)" -->
![[quadratic-micropip-demo.png|700]]

<!-- slide bg="rgb(37, 34, 43)" -->
notes:
In fact, because quadratic are using Pyodide inside webassembly, any pure python dependency can be installed, like this example of the faker library.

---

<!-- slide bg="rgb(37, 34, 43)" -->

![[quadratic-api-demo.png|700]]

notes:
Because all of python is running locally inside webassembly, complex work, such as here pulling data from an api, is possible.

---

<!-- slide bg="rgb(37, 34, 43)" -->

![[quadratic-fps.png|700]]


notes:

This is all running at 60fps on the gpu using webgl, all inside your browser.


---


<!-- slide bg="rgb(37, 34, 43)" -->

![[quadratic-section-zoom-in-out.gif]]

notes:
Quadratic built their infinite canvas on webgl, allowing for smooth scrolling and pinch to zoom.

---

<!-- slide bg="rgb(37, 34, 43)" -->
![[quadratic-gpt.png|500]]

notes:

They have recently released GPT integration, giving you a copilot or pair programmer while you're writing. Fantastic!

---


<!-- slide bg="rgb(37, 34, 43)" -->

![[quadratic-github.png|800]]

https://github.com/quadratichq/quadratic

notes:

it's open source and free to use.

---

<!-- slide bg="rgb(37, 34, 43)" -->

![[Quadratic Logo.png]]

The infinite canvas spreadsheet with code

https://QuadraticHQ.com

notes:

-   Signup today!
-   Head to QuadraticHQ.com to try it out.

My thanks to quadratic for their support of this channel.

---

## Part 2: 
# Data normalisation 

notes:

The answer is that we're not replacing a whole industry's experience modelling data.
I'm not going to teach you anything new today.

I'm going to recommend thinking about your data like we do in a database, and applying the ancient principles of normalisation to design it.

Tables existed long before object orientation, and will exist long after OO is a distant memory.

---


# ORM
## Vs
# DAL

notes:

You know how our industry is divided, between people who love ORMs because they bring databases into the object oriented world of our language, say java or Ruby, and people who love DALs, bringing the world of structured data into our object oriented languages.

This schism shouldn't exist. it's artificial.
OO has messed it all up.

Tables model the real world much better than inheritance models the real world.

Let's just use tables in our language model.

---

```sql
CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL 
  author TEXT NOT NULL,
)
```

```rust
struct Post {
    id: i32,
    title: String,
    body: String,
    published: bool,
    author: String,
}
```

notes:

Rust structs can model tables in a one to one manner, this example is adapted from the Diesel query builder for Rust, my second-favourite database connection tool behind SQLx.

Let's step through this example and see how we can improve it through normalisation.

---

## Useful
1.  First Normal Form
2.  Second Normal Form
3.  Third Normal Form
<br/>
## Less useful
4.  BCNF
5.  Fourth Normal Form

notes:

For some reason there are lots of different normal forms, you know how computer scientists love to make up stuff.

We're going to talk about 3rd normal form, and just call it a day, like the rest of the world does.

---

## 3rd Normal Form (3NF)

1. Eliminate repeating groups in tables.
2. Create a separate table for sets of related data.
3. Identify each set of related data with a key.
4. Create linking tables for many-to-many links.
5. Relate these tables with a foreign key.
6. Eliminate fields that do not depend on the key.

notes:

These rules are honestly common sense, and you'd do nearly all of them by yourself, HOWEVER knowing the rules of 3rd normal form will save you from data design mistakes later.

You can forget the Gang of Four patterns, but you must remember Normalisation.

Here's the post struct from earlier:

---

```rust
struct NormalPost {
    id: i32,
    title: String,
    body: String,
    published: bool,
    author: NormalUser,
}
```

```rust
struct NormalUser {
	id: i32,
	name: String
}
```
```rust
struct ImagePost {
	id: i32,
	image: Image,
	post: NormalPost
}
struct Image(String);
```

notes:

We've now got the structs in normal form, the data is split up into sensible structs, with data like author and image loosely coupled so multiple authors can write multiple posts, and images can be re-used.

depending on how relational the data is in your program, you may want to drop the ids, or inline a linking table.

That's all good, we're not strictly optimising for querying, we're just trying to get a handle on our data design.

Also note the Image tuple struct here, this allows us to create a new type that wraps a built in type.
ImagePosts need an Image, not just any old string.

Now you've refreshed your memory on data design, let's talk about making invalid states unrepresentable with state machines.

%%6:46%%

---

## Part 3: State machines
![[Turnstile_state_machine_colored.png]]
[Computers Without Memory - Computerphile](https://www.youtube.com/watch?v=vhiiia1_hC4)
notes:

I searched for a non-boring example of a state machine for DAYS when researching this video.
State machines are GOOD but they're not very interesting.

Circles are states, and arrows are transitions between the states.

If you like the science behind this, Computerphile have a GREAT video series on the subject, links in the source code 

Their video is good, but their examples of a turnstile or vending machine were rather dull.

I had lost all hope, but then.

---

![[super-mario-world-manual.webp]]



notes:
I was looking through the Super Mario World manual the other day, and found this amazing graphic on page 11.

Let's zoom in.

---


![[Smw_powerup_chart.jpg|700]]

["Finite State Machines in Rust"](https://www.youtube.com/watch?v=KdLTqyblbo4) &mdash; CodeScope

notes:

what's this?
We've got states and transitions!
IT'S A STATE MACHINE!

I think everyone in the world might be familiar with at least two of the states in this graph.

Let's build this state machine in Rust, and see how to make invalid marios unrepresentable.

---

```rust
#[derive(Debug, PartialEq)]
enum State {
    Mario,
    SuperMario,
    FireMario,
    CapeMario,
}
use State::*;
```

notes:
Build the valid states of your system with Rust's powerful enums.

Every mario on the graph is a new state, we've got mario, supermario, firemario, and capemario.

---

```rust
struct UserData {
	name: String,
	age: u8, // personally I aim to live to std::u8::MAX
	gender: String
}

enum User {
	One(UserData),
	Two(UserData)
}
```

notes:

Though we will keep things simple today, Rust's fat enums can contain as much data as you like, as you see here in this unrelated example.
Enums can contain structs and structs can contain enums.

Let's model our transitions.

---

```rust
#[derive(Debug)]
enum Transition {
    Feather,
    Flower,
    Mushroom,
}
use Transition::*;
```

```rust[]
dbg!(Transition::Flower);
//prints: [src/main.rs:84] Transition::Flower = Flower
```

notes:

Again we've modelled our transitions as an enum, and again if we wanted to add more data to the transition, we could add it here.

The first line, adding the debug attribute should be familiar to you if you've written your own structs before, they simply allow us to debug print our new type. 

The last line is a little hack to allow me to refer to the transitions by their unqualified name. I also did this for the states.

This will make our examples shorter today, but for non-trivial programs you should use the fully-qualified name, like I have here with Transition::Flower.

We're nearly done, we have the states and the transitions, now we must map the valid transitions between the states. 


---
```rust
#[derive(Debug)]
struct Player {
    state: State,
}
```
```rust
impl Player {
    fn new() -> Self {
        Self { state: Mario }
    }
    fn collect(&mut self, power: Transition) {
        match (&self.state, power) {
            (Mario, Mushroom) => self.state = SuperMario,
            (_,     Flower)   => self.state = FireMario,
            (_,     Feather)  => self.state = CapeMario,
            (_,     Mushroom) => {} // no change, 1up!
        }
    }
}
```

notes:

Here are the rules of our state machine.

First we build a player struct to hold our state.
This is the long-lived top level struct of your app, if you're building a ui, this is the window, if you're building a web server, this is the server app, which contains the db connection, etc.

We implement the new method constructor for our player, setting the default state as mario.
World 1-1 here we come!

The collect method is the only valid way to move between states.
The input to this function is what kind of powerup mario has collected, and this is passed into our match expression.


This expression is where our rules for valid state changes happen.

There are a few ways of doing this, you could use nested match expressions for more complex systems, perhaps using helper functions to split up the match into functions or even entire modules.

With a simple state machine, I recommend putting the state and transition event into a tuple and matching on THAT.

While developing your state, the type system and match expression keeps you safe, and because this compiles, I know I have covered all cases.


---

![[Smw_powerup_chart.jpg|500]]

```rust
fn main() {
	let mut itsame = Player::new();
	itsame.collect(Mushroom);
	itsame.collect(Flower);
	itsame.collect(Feather);
	itsame.collect(Mushroom);
	itsame.collect(Mushroom);
	assert!(itsame.state == CapeMario);
}
```


notes:

And here's our little state machine in action!

In other languages, we'd have to do a lot of testing to prove we'd written our arbitary if statements correctly, and not missed a case, or written unreachable cases.
Gross.

There are no edge cases here, I only have to scrutinise the match expression to ensure the business logic is implemented correctly, and when I need to change the states, perhaps in Super Mario 64, it's easy, and the compiler will tell us what match statements need updating.

---

## State Machines are EVERYWHERE

- regex
- cpus
- traffic lights
- pacemakers
- autopilot system
- reactive components!

notes:

- state machines are EVERYWHERE
- they're easy to reason about, easy to debug, and if you need to, the tightly defined scope allows them to be formally proved, for when human life depends on your system

Model your data with Rust's rich type system and you will make invalid states unrepresentable.

---

![[tri-hex-moon-white-transparent.png|300]]

# Thank you
## [Patreon.com/NoBoilerplate](http://www.patreon.com/noboilerplate)

notes:

# OUTRO

If you would like to support my channel, get early ad-free and tracking-free videos and vip discord access head to patreon.com/noboilerplate.

If you're interested in transhumanism and hopepunk stories, please check out my sci-fi audiofiction podcast, Lost Terminal.

Or if urban fantasy is more your bag, do listen to a strange and beautiful podcast I produce called Modem Prometheus.

Transcripts and compile-checked markdown sourcecode are available on github, links in the description, and corrections are in the pinned ERRATA comment.

Thank you so much for watching, talk to you on Discord.

