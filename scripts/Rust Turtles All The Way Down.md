
![rust](attachments/rust-logo.png)
# Turtles All The Way Down
notes:

Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

I'm going to talk to you about two features of rust that no other popular language has.

It's these two features that make rust both as low-level as C, and as high-level as lisp, whereas most other popular languages are stuck somewhere in the middle.

These two super powers are Rust's macros and the unsafe system.

---

## Build tools
↕️
## Typical Programming languages
↕️
## C/C++: Direct Hardware access

notes:

There are high, mid, and low level languages.

Macros give you access to high-level language features.
This category of techniques has a lot of different names, metaprogramming, templating, macros or preprocessors.

I am going to call them build tools, because many languages have incomplete metaprogramming implementations that provide a very narrow set of features that I don't want you to mix up, whereas in Rust, the macro system is so powerful you can create your entire build tooling using it, inside the compiler.

Python's metaprogramming allows classes to build classes.
Rusts's metaprogramming is turing complete and can do anything including accessing the disk and network.

Conversely, down at the low level, the unsafe system allows you the direct hardware access that every programming language uses, but only a few privileged C developers (ie the ones who first wrote the language) can use directly.

In both high and low level, other languages say "here is the line, you may never cross it". Rust says, cross here if you know what you're doing.


---

# Part 1:
# UNSAFETY

notes:
Let's talk about the unsafe system first.

---

JavaScript
```js[3]
> getSelection

function getSelection() { [native code] }
```

Python
```python[4]
>>> sorted??

Signature: sorted(iterable, /, *, key=None, reverse=False)
Type:      builtin_function_or_method
```


notes:

In most languages, you have a floor to your abstraction that you can't get lower than, without throwing away the language itself.

These “javascript” and “python” functions are actually written in C, and you’ll be writing your own C code if you want to tweak them.


---


![[nobody-sees-the-wizard.png]]
NOBODY SEES THE CODE!

notes:

This is how high-level languages are typically written. Even Go, a language that is often compared to Rust, is written partly in C++ and assembly. 

Fundamental functions and language features previously have only been written in C.

High-level developer ergonomics and low-level hardware control are thought of as mutually exclusive.

They perhaps were, until now.

---
std::collections::LinkedList
```rust[2]
fn push_front_node(&mut self, mut node: Box<Node<T>>) {
    unsafe {
        node.next = self.head;
        node.prev = None;
        let node = Some(Box::leak(node).into());
        match self.head {
            None => self.tail = node,
            Some(head) => (*head.as_ptr()).prev = node,
        }
        self.head = node;
        self.len += 1;
    }
}
```
(some code omitted)

notes:

Rust guarantees that you don't need to worry about memory management, the compiler will help you, as long as you obey the rules that I outlined in my last video.

You may have heard the buzzword that linked lists are impossible to make in Rust.

This is not true, the confusion arises because they're only impossible in SAFE rust, which is where we, as application developers spend all our time.

If you want to make a linked list, as they did in here in the standard library, you first step down into unsafe rust, where you promise to the compiler that you know what you are doing. 
Then, in your safe code, you can now use the safe abstraction you have just built.

This is analogous to writing C extensions for Python or Ruby, or writing JNI to access native libraries in Java.

You're PROMISING that you've checked the safety of the code.

If you write a C Python extension, you must throw out all of Python's ergonomics and safety, and learn a whole new language with vastly different rules and constraints.

In Rust, you need learn hardly anything more to write code in an unsafe block.

---

## Unsafety
-   Dereferencing a [raw pointer](https://doc.rust-lang.org/reference/types/pointer.html).
-   Reading or writing a [mutable](https://doc.rust-lang.org/reference/items/static-items.html#mutable-statics) or [external](https://doc.rust-lang.org/reference/items/external-blocks.html) static variable.
-   Accessing a field of a [`union`](https://doc.rust-lang.org/reference/items/unions.html), other than to assign to it.
-   Calling an unsafe function (including foreign C functions).
-   Implementing an [unsafe trait](https://doc.rust-lang.org/reference/items/traits.html#unsafe-traits).

notes:

Unsafe operations are those that can potentially violate the memory-safety guarantees of Rust's static semantics.

This list of language-level features cannot be used in the safe subset of Rust.

Have you noticed the one thing that links all the items in this list of things you can only do in unsafe rust?

---

## Unsafe operations are for ~~nerds~~ framework authors

notes:

Unsafe operations are for framework authors.

You and I, as web developers, or game developers, or application developers will certainly never write unsafe code in our normal work.
Just as we never did before.

However, the frameworks we use, the crates and 3rd party rust libraries will be faster, more powerful, and most importantly be pure rust with no C dependencies because of the unsafe system.

The unsafe block is a brilliant feature that keeps us writing Rust, when other programming languages would have to reach for C.

The language designers incorporated it to avoid the unsafely of C.
I like it because I don't have to learn any more languages.

---

# Part 2:
# Macros
notes:

On to Part 2: Macros

---

![[reprap.jpg]]

notes:
Let’s talk about the RepRap.

To build a reprap, you first build your own bad 3d printer with homemade parts and plans from the internet.  
THEN YOU USE THAT PRINTER TO PRINT A BETTER PRINTER

This is like using macros.
With other programming languages, you adapt the problem to the language.
With rust, you also can adapt the language to the problem.  

This sounds terrifying in traditional languages. Even with 
metaprogramming-happy languages, like python and ruby, where you *CAN* overwrite the addition operator and write DSLs, it’s considered bad form, and confusing for the next person.  

---

![[macro-compile.png]]

_(Graphics design is my passion)_

notes:
Macros are rust code that do two things:

1. They run at compile time and
2. They can modify your source code

---

1. They run at compile time and
2. They can modify your source code

notes:

These two properties allow you to do incredible magic that isn't possible in nearly all other languages.

```clojure
(lisps "being the big exception")
```

Let's talk about running at compile time first:

---
```rust
let countries = sqlx::query!(
    "SELECT country, COUNT(*) as count
     FROM users
     WHERE organization = ?",
     organization
)
```

runs:

```sql
SELECT country, COUNT(*) as count
FROM users
WHERE organization = "01234567890ABCDEF";
```

notes:
Macros are clearly notated in Rust by the exclamation mark, or bang, at the end of the name.
The compiler enforces this convention of course.
This tells you, the developer, that some compile-time execution is going to happen.
This is in contrast to python or ruby, where it is not obvious what is happening.

In this case, the `query!` macro, from the excellent `sqlx` library, runs that sql query on your local dev database at compile time in a rolled-back transaction.
The macro doesn't know what the value of `organization` will be, it will only be populated at runtime, perhaps on your production server, or your customer's computer.
However, thanks to the rich type system, the macro DOES know EXACTLY what kind of value it will be.
If it's a number, it knows the upper and lower bounds of that number, if it's a string it knows it's valid UTF-8, and so on.
Because of this, the macro can generate a valid `organisation` at compile time, probably random data, and feed that into the database's transaction to test a real query.
If there is an error, the compiler feeds back to you using the same errors you've seen in non-macro rust code.
This means your IDE using LSP or the command line using cargo, has the same rich errors Rust is famous for.

---

## macros are a build tool inside your code

notes:

I think this is a good way to think about Macros.

What do build tools do?
- they collect build artefacts like images and config
- they change behaviour of the app depending on configuration files
- they bundle up loose files into more readily-deployable blobs
- and they rewrite source code from a form that is easier for the human, perhaps JSX or sass, into a form that is easier for the machine, like javascript or css.

Let's talk about how macros modify source code:

---

```rust
macro_rules! add{
    ($a:expr,$b:expr)=>{
        {
            $a+$b
        }
    }
}

fn main(){
    add!(1,2); // this line will be replaced with `1+2`
}
```

notes:

Macros, like unsafe code, are a little more difficult to read than normal rust.
But they're only used by very experienced crate and framework authors, not the end developer like me, who just wants to make my cool tetris clone in webassembly.

Note that the numerical result of 3 is not inserted into the code here. 
The addition must still be executed at runtime, it's the syntax that has been swapped around at compile time.

This distinction between functions that execute at runtime and macros that execute at compile time is essential.

Most languages have no access to compile time, and you are instead forced into building non-standard error-prone build tools that you run before executing your code as preprocessors.

Let's look at a practical example of a rust syntax macro.

---

```rust
cmd!(http get localhost);
```

is unpacked to:

```rust
let cmd = std::process::Command::new("http");
    cmd.arg("get");
    cmd.arg("localhost");
```

notes:
The CMD macro is from the command-macros crate.
https://crates.io/crates/command-macros

You could have done this with a function, but then you'd be calling a function at runtime, with the small overhead a function call incurs.

You also would have needed commas in there.
Note that the cmd! macro introduces new syntax, not just an abstraction.

Macros are an important part of Rust's zero cost abstractions.

Running them at compile time, on the developers machine allows new syntax to be added without waiting for a new edition of the language to be produced.
Clearly signposted and scoped inside the macro.

You also don't need permission.

---

```python
>>> command = 'Hello, World!'
>>> match command:
...     case 'Hello, World!':
...         print('Hello to you too!')
...     case 'Goodbye, World!':
...         print('See you later')
...     case other:
...         print('No match found')
```

Case statement in Python 3.10

notes:

Python finally has a case statement. But it took a long time to arrive didn't it?

The case for the case statement was [officially put forward](https://peps.python.org/pep-3103/) by Python's author, Guido, in 2006.
(right about the time I started writing python)

However, when he presented it at PyCon in 2007, the audience felt it was unnecessary, and so the proposal was rejected.

This new syntax, even one that is functionally identical to nested if statements, was so hard to build that they took 15 years to agree to do it.

In rust, new syntax is a library. For example:

---

```rust[]
let path = "/user/home";
let (value, _) = match_request!(&Method::GET, path, {
    "/login" => {
        GET => "serve login form",
    },
    "/user/home" => {
        GET => "serve home page",
    }
}).unwrap();

assert_eq!(value, "serve home page");
```

notes:

Here is a macro that looks like a native rust match statement, but is actually creating routes in the Hyper HTTP server 

You can clearly see where the macro is, on line 2, because of the bang at the end of `match_request`.

https://crates.io/crates/match_request
---

## Rust's build times are slow

```shell[10]
❯ cargo build
Compiling version_check v0.9.4
Compiling typenum v1.15.0
Compiling libc v0.2.126
[...]
Compiling pear v0.1.5
Compiling rocket_http v0.4.11
Compiling rustrocket v0.1.0 (~/projects/rustrocket)

Finished dev [debuginfo] target(s) in 10.92s
```

notes:

A common complaint from people who have never compiled Rust is that Rust's build times are slow.

They are indeed a little slower than Go or Java.
But that's because Rust is DOING WORK FOR YOU.

A lot more happens at rust build time than in most other languages.

Here, as I build the Rocket web framework's hello world example in 10 seconds, 100 libraries compiled in total, many things happen in Rust that do not happen in other languages:
- Macros are being executed, arbitrary code is running, and macros themselves can insert further macros in your code (such as the `println!()` macro) that will then require further expansion
- The compiler, in conjunction with the rich type system is proving that all code both yours and imported libraries does not break either your contracts or those built-in to the language or libraries.
- The borrow checker is exhaustively proving that no execution paths can break the memory guarantees of the language.

These few extra seconds of build time equate to HOURS, DAYS, WEEKS of gained productivity because you're not hunting bugs in log files.

---

```shell[2]
❯ cargo build
    Finished dev [debuginfo] target(s) in 0.02s
```

notes:
oh, and the second and all subsequent times you build this it takes no time at all.

---

# NO RUST 2.0

notes:
In previous videos I've talked about how there's no rust 2.0.

And you see why that is now, don't you?

There's no need to wait for a version 2.0 that may never arrive, you can write new language features today, and people have:
    - async!
    - serde
    - contracts
    - proof systems
    - literals (for list!, and map!s etc)

The combination of low-level hardware access and high-level macros has given us the perfect language, not just now, but for the next 40 years.  

- The rust developers have built this incredible hybrid language.
    - They didn't only make a complicated low-level language where you can do pointer arithmetic
    - Nor did they just make a high-level language where you have no easy access to the underlying system, forcing you to write C to get at it 
    - They gave us a language that has BOTH.


---


![rust](attachments/rust-logo.png)

## With Rust, it's turtles
## all the way down

notes:

If you'd like to see what you can write in rust, click the top video: I used it to make a fun retro computer visualisation for my scifi and mental health podcast, Lost Terminal.

And if you'd like to watch more of my fast, technical videos, click the bottom video.

Transcripts and markdown sourcecode are available on github, links in the description.

Thank so much for watching, see you next time.
