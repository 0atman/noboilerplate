# [Frontmatter]

notes:

# Styles

<style>
.reveal code.rust {
  font-size: 1em;
}
.reveal code.md {
  font-size: 1.5em;
  line-height: 1.5em;
}
.reveal code.python {
  font-size: 1.5em;
  line-height: 1.5em;
}
.reveal code.ruby {
  font-size: 1.5em;
  line-height: 1.5em;
}
.reveal code.sh {
  font-size: 0.7em;
  line-height: 1.2em;
}
</style>


# Lint tweaks
```rust
#![allow(dead_code)]
```

# extern crates

```rust

```

# imports
```rust
```

# setup

```rust
fn main() {

```

---

![[rust-logo.png|350]]

# RUST is not a faster horse

notes:

Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

Thesis:

Rust is something new, a new way to solve our problems.
whether you're coming from low-level languages, or high-level languages, there's groundbreaking methods here.


---

# misconceptions

notes:

Lets start with some misconceptions.


---


## Rust is not a better C

![[Zig.svg]]
notes:

Rust is not a better C, you're thinking of ZIG. %% ERRATA: or Dlang %%

Zig is an INCREDIBLE language that if you love Rust, you'll also like. 

It is a FANTASTIC drop-in replacement for C, and it's ready now.


But is that all you want? A better C?

---

## Rust is not a better C++

![[Go_Logo_Blue.svg.png]]


notes:

Rust also isn't a better C++, you're thinking of GO. %% ERRATA: Actually, Go is more like a better Java %%

Carbon might be, too, once we find out if Google's going to get bored of it it next year.

I really like Go. Ken Thompson and team have done such a good job at perfecting what they set out to do, which is (according to their blog posts) "What would we do differently if we designed C++ again today", and I don't mean that as an insult. It's nothing short of genius.

They were primarily motivated by their shared [dislike of C++](https://en.wikipedia.org/wiki/Criticism_of_C%2B%2B "Criticism of C++").[](https://en.wikipedia.org/wiki/Go_(programming_language)#cite_note-24) 

Like C++, Go has a simple type system and relies on object orientation. If you know C++ or Java, you'll feel right at home.

But is that all you want? A better C++?

---

![[ford-horses.jpeg]]

notes:

# Faster Horses

It is vanishingly unlikely that Henry Ford ever said this, the earliest written evidence I could find is in the 1980s.

It's a shame, because this IDEA is pervasive, and I think about it a lot when well-meaning people ask me why we can't use Go, or Zig, or Kotlin, or Typescript, instead of Rust.

None of these languages are doing anything dramatically new.
Some of them, especially Go, are doing things REALLY WELL, and have incredibly useful features to build TODAY's distributed systems.

But for me, That. Is. Not. Enough.


---

## Rust is FAST

notes:

If you want the very fastest executing language in the world, it's still C (just about) (https://just.billywhizz.io/blog/on-javascript-performance-01/)

But a fast-executing language that crashes all the time is like a supercar... That crashes all the time.

Let's put this into perspective:

Go is about 2x slower than C, Java and Javascript are about 3 or 4 times slower, depending on whom you ask.

Getting into dynamic languages:
Ruby is about an order of magnitude slower than C,
and Python is at least two orders of magnitude slower.

(https://github.com/kostya/benchmarks)

Rust however is within 10% of C's speed.
It often will compile to identical assembly.

Speed is a feature.

So if Rust isn't just a better C or C++, what is it?

---


## Rust is a new way of programming


notes:

Rust feels to me like a new way of programming
I'll show you how in a moment, but first I want to showcase the unique features of rust that, though you can get some elsewhere, you can't get them all together in a popular language you can hire a team for today.

---

## Weird Feature #1:
# Algebraic type system

notes:
Rust has an algebraic type system, and if you know what that is you already know you want it in your language.

Specifically, Rust has both product and sum types.
You already know what a product type is, you've used it forever.


---

```rust
struct User {
    name: String,
    power: i32
}
let tris = User {
    name: "Tris".into(),
    power:9000
};
```

notes:
- [x] retake this slide
A product type is one that is a composite of two or more types, like here, where a user is a composite of a string and an integer.

You can think of old object oriented classes as a product type.

Basically every language with a type system has product types.

---


```rust
enum Animal {
    Dog(String),
    Cat(String)
}
```

```rust
fn pet_name(pet: Animal) -> Option<String> {
    match pet {
        Animal::Dog(name) => Some(name),
        Animal::Cat(_)    => None
    }
}
```


notes:
A sum type is one you can create with a finite list of valid variants, like an enum.
As you see here, we have modelled two kinds of animals, and each animal has a name.
The `pet_name` function returns a dogs name if your pet is a dog, and it returns nothing if it is a cat because cats don't come when you call them so why would you learn their name.

You can think of enums as doing some of the old object oriented inheritance, where your functions accept any instance of animal.

```
enum Person {
    Engineer,
    Scientist,
    Height(i32),
    Weight(i32),
    Info {
        name: String,
        height: i32
    }
}
```


---


```rust
fn inspect(p: Person) {  
    match p {  
        Person::Engineer  => println!("Is an engineer!"),  
        Person::Scientist => println!("Is a scientist!"),  
        Person::Height(i) => println!("Height of {}.", i),  
        Person::Weight(i) => println!("Weight of {}.", i),  
        Person::Info { name, height } => {  
            println!("{} is {} tall!", name, height);  
        },  
    }  
}
```

notes:
Here is a bigger match expression from Rust By Example.

You can see that the match can destructure deeply nested data, and structs can be inside enums, and enums be inside structs.

https://doc.rust-lang.org/rust-by-example/custom_types/enum.html

---

## Weird feature #2:
# data ownership

notes:
As well as a rich type system, Core to this new way of programming is the concept of ownership.

And what do I mean by this? Let me show you:

---

## Javascript QUIZ TIME

Arrays

```js
> const a = [1, 2, 3] 
> const b = a
> b[0] = 1000000

> console.log(a)
?
```

Strings

```js
> let a = "tris"
> let b = a
> b = "newname" // in the video, this was mistakeninly `a = "newname"`

> console.log(a)
?
```

notes:
What does `a` equal in both these cases? Did we pass by reference or pass by value?
Was `a` mutated or not?

Even with these two simple data types, this is a non trivial question that you're probably going to have to look up.
First year computer scientists would be upset about this ambiguity, before they have the rules of pass by reference and pass by value for javascript beaten into them.

Are these rules the same for Ruby, or Python or java or your favourite language?
Maybe.

Rust does things differently.

---

```rust
let a = vec![1, 2, 3]; 
let b = a;
println!("{:?}", a);
```
error:
```js[3-4]
 1 | let a = vec![1, 2, 3]; 
   |     - move occurs because `a` has type `Vec<i32>`
 2 | let b = a;
   |         - value moved here
 3 | println!("{:?}", a);
   |                  ^ value borrowed here after move
```

notes:
By way of example, here's what happens if you try a similar thing in Rust

as usual, rust provides us with an excellent error explaining the problem.
Lets focus on the assignment on line 2.

Something special happened here.

Ownership of a's data, that vector we created on line 1, transferred from a to b.
You can think of it as passing by reference, and then deleting the original variable.
After that line, we say that `a` has been moved.
And after it's moved, it's GONE.

This is a clever way of getting the speed of passing by reference, while side-stepping a whole category of errors, and not to mention memory management problems, that would otherwise come with it.

---

# This is a trend
# in rust

notes:
This is a trend you will see throughout your Rust journey.
Rust just doesn't do the things that other popular languages do.
It's unfamiliar.

But that's the whole point.
The world is on fire, and Rust refuses to do things the bad old familiar ways.

---

> "Stop thinking of the unfamiliar as bad, and you'll be happier."

&mdash; No Boilerplate, 2020

notes:
Honestly, good life advice in general there.

Let me tell you about my new website and discord server.

---


<!-- slide bg="#1a5f7a" -->

## NoBoilerplate.org

![[discord.png]]

notes:

After the astonishing success of the channel, I've registered and added some links to noboilerplate.org, the most important being the invite link to the discord server.

Come and talk to me about my videos, rust, obsidian, or any fun tech topics, and if you're interested in my other creative work, there's channels for Lost Terminal and Modem Prometheus there too.

Thanks to everyone who has supported me on this journey so far, I'm very grateful, and I'm having a really great time.

Back to types.

---

## Weird feature #3:
# No runtime types

notes:
Though not the most rare feature, it will be unfamiliar for most of my audience who are coming from high-level languages like Javascript.

This feature is something that C and Rust share.
Not having a runtime is a superpower that other high-level languages give up too quickly.

Let me explain.

---

## python

```python
name = "Tris"
```

notes:

For me, a dynamically typed language is one that is too disconnected from where I live.

You and I, as developers, live exclusively at compile time.
We sit in front of our editor and look at the same source code the compiler does.

In a dynamic language like here with Python, we have to IMAGINE what the types will be at runtime.

This source code eventually is parsed by the python interpreter, and bytecode and what have you, and will probably be a string.
This example is trivial enough.
Sometimes we can interrogate the runtime to confirm this. But oftentimes we can't.

We didn't get that concrete type information at compile time, we have to know, through convention and experience, that this duck will quack like a string when the code runs on the server, or on the customer's laptop, or in our tests.

The supposed advantage of this indirection is that we don't have to define the type of variables in our source code.
But is this such a burden that we should accept this indirection and uncertainty so quickly?

C and Rust among many others say no.


---


## Weird Feature #4
# no Garbage Collector
notes:

While C and C++ don't have a garbage collector, they are not memory safe.

Rust is one of the few languages that solves memory safety without a garbage collector.

This is another superpower, and one robbed from us in languages that implement a GC like Java, Go, JS, Ruby, and Python.

Again, a level of indirection happens with a Garbage Collector.
Our data in our variables will be cleaned up when they are no longer used AT SOME POINT DURING EXECUTION.

This loss of control causes non-deterministic behaviour, because we can't quite predict when this data will be cleaned up.
It's better in some languages than others.

Worst of all is that for a GC to function, stop-the-world pauses are required in most popular languages.
Our code's execution stops while the GC cleans up the memory we're no longer using and then starts again.

In modern languages, it's common to find GC pauses below 1ms, which is a tremendous achievement.
But at 60fps, an entire frame needs to be rendered in just 16ms which is not a lot of margin for error.

Whole categories of real-time applications require sub ms accuracy. 
And not to mention platforms where a GC is not possible, like embedded development, realtime audio and video, and webassembly
What then?

Well, then it's time to tune your application if you're lucky, and if you're unlucky, your virtual machine or runtime.

Avoiding a GC sidesteps this whole category of problems.

---

## Weird feature #5
# data has lifetimes

notes:

Let's talk about lifetime annotations, which throw off some newcomers to Rust's syntax.

---

> a variable's lifetime begins when it is created and ends when it is destroyed.

&mdash; The Rust Book


notes:

The Rust Book says a variable's lifetime begins when it is created and ends when it is destroyed.

For instance:


---

```js
fn main() {
    let i = 3; // Lifetime for `i` starts. ────────────────┐
    { //                                                   │
        let borrow1 = &i; // `borrow1` lifetime starts. ──┐│
        //                                                ││
        println!("borrow1: {}", borrow1); //              ││
    } // `borrow1 ends. ──────────────────────────────────┘│
    //                                                     │
}   // Lifetime ends. ─────────────────────────────────────┘
```

notes:

This code is also taken from Rust By Example, but simplified

Lifetimes are annotated here with lines denoting the creation and destruction of each variable.
`i` has the longest lifetime because its scope entirely encloses `borrow1`.

While this example is simple because it exactly corresponds with the normal scoping rules you'll be familiar with in other languages, the power and complexity of lifetimes comes with it's interaction with the type system:

---

```rust
struct Account(String);
struct Order<'a>{
    lock: &'a Account
    // presumably other fields too
}
```
```rust
fn order_example<'a>() -> Order<'a> {
    let tris = Account("tris".into());
    Order{lock: &tris}
}
```


`Error: returns a value referencing data owned by the current function`

notes:
- [x] retake this slide
Here is how you bring all of this together to make compile-checked logic.

I have started here by modelling the relationship 'Orders must not be placed if the account is invalid'.
This is the first thing you should do when starting a new rust application, think about your valid states.
Here we have an order that contains a reference to an Account. The lifetime annotation 'a indicates that the referenced account must not go out of scope or be moved for the order to remain valid.

It is then a simple matter to write the function `order_example` that can not break this contract we have written into our type system.

It doesn't matter if we use concurrent code, threads, channels, or anything else, the compiler will keep us safe, and stop us or anyone who uses our code from violating these rules.

An order must have a currently valid account, always.

---

## Weird feature #6
# Macros

notes:

Rust also has access to compile-time execution using the macro system.
C has a templating system that is hot garbage and a nightmare to use, a string manipulation language at best.

What most people learn from C is that compile-time metaprograming is scary, and perhaps we shouldn't do it.

Go learned that.
Python and Ruby learned that, along with Javascript.

Rust is not so timid.
Rust's macro system is based on lisp's, the golden standard for metaprogramming for 50 years.

---

## Weird feature #7
# Unsafe System

notes:

Rust's unsafe system learns lessons from C's direct pointer access.

---

```rust
let mut num = 5;
let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;
unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}
```

notes:

Instead of locking away this vital interaction with the real machine, as other twice-shy languages do, the rust compiler insists you clearly signpost the small parts of your system that require pointer access.
You don't throw away all the safety of rust inside an Unsafe block, but you are now allowed to dereference a raw pointer, as well as 4 other superpowers, links in the sourcecode:

https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html

If you have a memory bug, you know exactly where to look.

---

## Rust is not a faster horse
# It's a rocket

notes:

Rust doesn't throw away what we have learned about low-level hardware access, nor does it repeat the mistakes of the past.

It's unfamiliar, there's no classes, you have to think much more carefully about types and state.
But that is good.

Rust might not be the language you wanted, but rust might be the language you need.


---

# OUTTRO

notes:

If you'd like to see what you can write in rust, click the top video: I used it to make a fun retro computer visualisation for my hopepunk podcast, Lost Terminal.

Or if urban fantasy is more your bag, click the bottom video to listen to a strange and beautiful podcast I produce called Modem Prometheus.

Transcripts and compile-checked markdown sourcecode are available on github, links in the description, and corrections are in the pinned ERRATA comment.

Thank you so much for watching, see you next time.

---

%% Thumbnail %%


![[rust-logo.png]]

# It's a rocket 

notes:

```rust
} // that's all folks!
```
