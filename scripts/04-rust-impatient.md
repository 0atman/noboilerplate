<style>
.reveal code.rust {
  font-size: 1.5em;
  line-height: 1.5em;
}
</style>

![[rust-logo.png]]
## Learn Rust
## in 10 minutes

notes:

Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

Today I'm going to take you from zero to rust in 10 minutes.

---

![](https://fasterthanli.me/img/logo-round-2.png) 

[fasterthanli.me/articles/a-half-hour-to-learn-rust](https://fasterthanli.me/articles/a-half-hour-to-learn-rust)

-- [@fasterthanlime](https://twitter.com/fasterthanlime)


%% add cool bear %%

notes:

The content of this presentation is based on the excellent blog post "A Half Hour To Learn Rust", by the preeminent [Amos Wenger](http://amos.me/).
Amos's incredible deep-dive tutorials were a big part of my Rust journey, and you should read everything he's ever written.

I find Amos's fast articles to be exactly compatible with how my brain works. And so, after telling dozens of people to go read this article, I decided to just make a video on it.

I reached out to Amos on twitter, and he generously said go for it.

Any errors in this video are because I have deviated from his original post.

---

> In order to increase fluency in a programming language, one has to read a lot of it. But how can you read a lot of it if you don't know what it means?

notes:

As Amos says, instead of focusing on one or two concepts, we are going to go through as many Rust snippets as possible, and explain what the keywords and symbols they contain mean.

This is going to be fast.
I need you to read and listen at the same time.
Ride the pause button if you need to.
I believe in you.

---

#### `curl https://sh.rustup.rs | sh`

(or visit [rustup.rs](https://www.rustup.rs))

notes:

After you have installed rust from [rustup.rs](https://www.rustup.rs), you are ready to begin.

---

`let` introduces a variable binding:

```rust
let x;  // declare "x"
x = 42; // assign 42 to "x"
```

This can be written as a single line

```rust
let x = 42;
```


---

Types can be annotated

```rust
let x: i32;
x = 42;
```

This can also be written as a single line

```rust
let x: i32 = 42;
```


notes:
`i32` is a signed 32-bit integer.
You can specify the variable's type explicitly after a colon :, which is called a type annotation:

---

![[dtype_promotion_lattice.png]]

https://data-apis.org/array-api/latest/API_specification/type_promotion.html

notes:

Behind the scenes, languages such as Python hide the implementation from you, and quietly promote the length of the integer at runtime when needed. This is clever, but inefficient when you know how big your numbers are going to be.

Rust of course has crates that provide this dynamic behaviour if you need it.

But how do you choose which integer type to use in Rust?

---

# JUST USE i32 for everything

notes:

In general, if you just need to represent some number, use an `i32`: 32 bits has a wide enough range that it's relatively unlikely to overflow, and signed numbers have fewer surprises than unsigned numbers. `i32` is equivalent to a `long` in C, or an `int` in Java. If you use a numeric literal and don't explicitly annotate the type, Rust will default to `i32`.

---
You can't access uninitialised variables

```rust
let x;
foobar(x); 
// borrow of possibly-uninitialized `x`
x = 42;
```
However, doing this is completely fine:

```rust
let x;
x = 42;
foobar(x); 
```

notes:

If you declare a name and initialise it later, the compiler will prevent you from using it before it's initialised.

I know this sounds obvious, but C crashes at runtime if you try to do this.

---


this does *nothing* because 42 is a constant

```rust
let _ = 42;
```

this calls `get_thing` but throws away its result

```rust
let _ = get_thing();
```

notes:

The underscore _ is a special name - or rather, a "lack of name". It basically means to throw away something, and not warn about it not being used.

This pattern of strict defaults with escape hatches to not be overly annoying is one you will see a lot with rust.

---

```rust
let pair = ('a', 17);
pair.0; // this is 'a'
pair.1; // this is 17
```

Or, with explicit type annotation:
```rust
let pair: (char, i32) = ('a', 17);
```

notes:

Rust has tuples, which you can think of as "fixed-length collections of values of different types".

Note that the Rust compiler can nearly always infer the types that you are using, and only rarely do you need to clarify ambiguous cases.

How many times have you thought you were working with one type of variable, only to realise through an error it was actually another? Rust helps you out here.

---

```rust
let (some_char, some_int) = ('a', 17);
assert!(some_char, 'a');
assert!(some_int, 17);
```

```rust
let (l, r) = slice.split_at(middle);
```

```rust
let (_, right) = slice.split_at(middle);
```

notes:

Tuples can be destructured when doing an assignment, which means they're broken down into their individual fields

This is especially useful when a function returns a tuple, such as in the second example with `split_at(middle)

When destructuring a tuple, an underscore can be used to throw away part of it.

---

```rust
let x = 3;
let y = 5;
let z = y + x;
```

The semi-colon marks the end of a statement.

notes:

The semi-colon marks the end of a statement.
Unlike in other langauges, semicolons are not just mandatory whitespace.

---

Statements semicolons can span multiple lines

```rust
let x = vec![1, 2, 3, 4, 5, 6, 7, 8]
    .iter()
    .map(|x| x + 3)
    .fold(0, |x, y| x + y);
```

notes:
This is an example of Rust's powerful, elegant iterators.
We'll talk about them later.

In Rust, like in Lisp, nearly everything is an expression 

---

`f -> void`
```rust
fn greet() {
    println!("Hi there!");
}
```
`f -> i32`
```rust
fn fair_dice_roll() -> i32 {
    4
}
```

notes:
fn declares a function.

At the top we have a void function, a function that returns nothing.

And below, a function that returns an integer. The arrow indicates a function's return type.

---

```rust[4]
let x = "out";
{
    // this is a different `x`
    let x = "in";
    println!("{}", x);
}
println!("{}", x);
```
This prints "in", then "out"

notes:

A pair of brackets declares a block, which has its own scope, sort of like an immediate function in javascript.

This interior variable `x` only lives as long as the block does, and does not modify the external `x`

---

```rust
// this:
let x = 42;

// is equivalent to this:
let x = { 42 };
```

notes:
Blocks are also expressions, which mean they evaluate to a value.
If you've written lisp or ruby, this should start to feel very familiar.

---

```rust
let x = {
    let y = 1; // first statement
    let z = 2; // second statement
    y + z // this is the *tail* 
};
```

notes:
Inside a block, there can be multiple statements
We call the final expression of a block the "tail": This is what the whole block will evaluate to.

---

```rust
fn fair_dice_roll() -> i32 {
    return 4;
}

fn fair_dice_roll() -> i32 {
    4
}
```
these are equivalent

notes:
these are equivalent
This is why "omitting the semicolon at the end of a function" is the same as returning.

---

```rust
fn fair_dice_roll() -> i32 {
    if feeling_lucky {
        6
    } else {
        4
    }
}
```
if conditionals are also expressions

notes:

if conditionals are also expressions
Note that this whole function returns an integer

---

```rust
fn fair_dice_roll() -> i32 {
    match feeling_lucky {
        true  => 6,
        false => 4,
    }
}
```
A match is also an expression, not a statement

---

Dots are typically used to access fields of a value:
```rust
let a = (10, 20);
a.0; // == 10

let amos = get_some_struct();
amos.nickname; // == "fasterthanlime"
```
Or call a method on a value:
```rust
let nick = "fasterthanlime";
nick.len(); // this is 14
```

notes:

Dots are typically used to access fields of a value:
Or call a method on a value:
Just like in most c-style language, dots are used for attributes or methods.

---

```rust
let least = std::cmp::min(3, 8);
```


Approximately:
```rust
crate::file::function
```

notes:

The double-colon, ::, is similar but it operates on namespaces.
This distinction provides great clarity between if you're using a property or a namespace.

In the first example, std is a crate (~ a library), cmp is a module (~ a source file), and min is a function:

---

```rust
use std::cmp::min;

let least = min(7, 1); // this is 1
```

notes:
the `use` directive can bring names from other namespace into scope.

Rust has strict scoping rules, if you don't see it in your source code, it's not available.

---

```rust
let x = "amos".len(); // 4
let x = str::len("amos"); // also 4
```

notes:

Types are namespaces too, and methods can be called as regular functions:
str is a primitive type, but many non-primitive types are also in scope by default.

---

# DEEP BREATH

notes:

Let's get into the type system

---

Structs are declared with the struct keyword:
```rust
struct Number {
    odd: bool,
    value: i32,
}
```
They can be initialised using literals:
```rust
let x = Number { odd: false, value: 2 };
let y = Number { value: 3, odd: true};
// the order does not matter
```

notes:
Structs are the backbone of Rust's excellent rich type system, 
Think of Structs as lightweight new types, encapsulating the valid states of your system.

---

```rust
fn print_number(n: Number) {
    match n.value {
        1 => println!("One"),
        2 => println!("Two"),
        _ => println!("{}", n.value),
    }
}
```

notes:
match arms are patterns, a match has to be exhaustive: at least one arm needs to match, and an underscore can be used as a "catch-all" pattern

In addition to these primitive integer matches, you also can match deeply nested data, and destructure it for ease of use.

---

```rust
struct Number {
    odd: bool,
    value: i32,
}
```
```rust
impl Number {
    fn is_positive(self) -> bool {
        self.value > 0
    }
}
```

notes:
You can declare methods on your own types, like here where we're adding an `is_positive()` method to our new Number struct.

---

```rust
let minus_two = Number {
    odd: false,
    value: -2,
};
println!("{}", minus_two.is_positive());
```

notes:
And then we can use our new methods like usual

---

```rust
let n = Number {
    odd: true,
    value: 17,
};
n.odd = false; 
```

ERROR
```markdown
error: cannot assign to `n.odd`,
as `n` is not declared to be mutable
```

notes:

Variable bindings are immutable by default, which means their interior can't be mutated, and also that they cannot be assigned to

---

```rust[2]
fn main() {
    let mut n = Number {
        odd: true,
        value: 17,
    }
    n.value = 19; // all good
}
```
`mut` makes a variable binding mutable:

notes:

Those of you coming from functional languages like Haskell will be very pleased to see immutability by default, rather than other c-like languages, which typically have immutability added on later through keywords like const.

Though Rust isn't strictly a functional language, it is clear that the language design has balanced practicality with purity from the functional world.

---

Functions can be generic:
```rust
fn foobar<T>(arg: T) {
    // work with `arg`
}
```
```rust
fn foobar<L, R>(left: L, right: R) {
    // work with `left` and `right`
}
```

notes:
Functions can be generic
They can have multiple type parameters, which can then be used in the function's declaration and its body, instead of concrete types
Think of them like a template string.

---

Structs can be generic too:
```rust
struct Pair<T> {
    a: T,
    b: T,
}
```

```rust
let p1 = Pair { a: 3, b: 9 };
// = Pair<i32>
let p2 = Pair { a: true, b: false };
// = Pair<bool>
```

notes:
Note that in this case, both a and b must be of the same type, T.


---

```rust
let mut v1 = Vec::new();
v1.push(1);
let mut v2 = Vec::new();
v2.push(false);
// v1 == Vec<i32>
// v2 == Vec<bool>
```

notes:

The standard library type Vec (Which is a heap-allocated array), is generic.

v1 is a vector of integers
v2 is a vector of booleans

Behind the scenes, Vectors use an array and swap it out for a larger array at runtime when it reaches full capacity.

---

```rust
fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![true, false, true];
}
```

notes: 
Speaking of Vec, it comes with a macro that gives us more or less "vec literals"
v1 is a vector of integers (i32 as usual)
and v2 is a vector of booleans

---

- `name!()`
- `name![]`
- `name!{}` 

notes:

All of these invoke a macro. Macros just expand to regular code.
You can recognise them by the bang at the end of the name.

---
`println` is a macro

```rust
fn main() {
    println!("{}", "Hello there!");
}
```
```rust
fn main() {
    use std::io::{self, Write};
    io::stdout()
        .lock()
        .write_all(b"Hello there!\n")
        .unwrap();
}
```

notes:
In fact, println is a macro:
This expands to something that has the same effect as the second block here.

---

```rust
fn main() {
    panic!("This panics");
}
```
error
```markdown
thread 'main' panicked at 'This panics', src/main.rs:3:5
```
notes:

panic is also a macro. It violently stops execution with an error message, and the file name and line number of the error.

---

```rust
fn main() {
    let o1: Option<i32> = Some(128);
    o1.unwrap(); // this is fine

    let o2: Option<i32> = None;
    o2.unwrap(); // this panics!
}
```
error
```markdown
thread 'main' panicked at
'called `Option::unwrap()` on a `None` value', 
src/libcore/option.rs:378:21
```

notes:

Some methods also panic. For example, the Option type can contain something, or it can contain nothing. If .unwrap() is called on it, and it contains nothing, it panics:

---

```rust
enum Option<T> {
    None,
    Some(T),
}
```
```rust
impl<T> Option<T> {
    fn unwrap(self) -> T {
        match self {
            Self::Some(t) => t,
            Self::None => panic!(...),
}}}
```

notes:
Option is not a struct - it's an enum, with two variants.
Note the generic type parameter, an option's Some can be of any type.
enums' variants can be used in patterns

---
`RESULT`
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

notes:
Result is also an enum, it can either contain something, or an error:
It also panics when unwrapped and containing an error.

---

```rust
let s1 = str::from_utf8(
    &[240, 159, 141, 137]
);
println!("{:?}", s1);
```
```markdown
`Ok("🍉")`
```
```rust
let s2 = str::from_utf8(&[195, 40]);
println!("{:?}", s2);
```

```markdown
`OErr(Utf8Error { valid_up_to: 0, error_len: Some(1) })`
```

notes:
Functions that can fail typically return a Result, like here, where we're creating a UTF8 string from bytes.
Not all bytes represent a valid string, (talk to anyone who is still using python 2!)
In the first example, we can see that s1 is the OK variant of the Result enum.
But S2 is the Error variant of the Result enum.
This pattern of errors-as-values keeps us in the functional world, where other languages would have exceptions which break us out.

---

```rust
let s = str::from_utf8(
    &[240, 159, 141, 137]).unwrap();
println!("{:?}", s);
```
```markdown
"🍉"
```
whereas...
```rust
str::from_utf8(&[195, 40]).unwrap();
```
```markdown
thread 'main' panicked at
'called `Result::unwrap()` on an `Err` value: 
`Utf8Error { valid_up_to: 0, error_len: Some(1) }`'
```

notes:
If you want to panic in case of failure, you can .unwrap():
Here we're deliberately quitting our program on a bad UTF8 string.

---

```rust
let s = str::from_utf8(&[195, 40])
.expect("valid utf-8");
```
error
```markdown
thread 'main' panicked at 'valid utf-8:
`Utf8Error{ valid_up_to: 0, error_len: Some(1) }`'
```

notes:
Or you can use .expect(), for a custom error message before panicking. 
It's called expect because it telegraphs to people reading both the code and the errors what you were expecting when you unwrapped the result.
As with all things, we don't always get what we want.

---

```rust
let melon = &[240, 159, 141, 137];
match str::from_utf8(melon) {
    Ok(s) => println!("{}", s),
    Err(e) => panic!(e),
}
// prints 🍉
```

notes:
You can also match and handle the error (or in this example, panic anyway!)

---

```rust
let melon = &[240, 159, 141, 137];
if let Ok(s) = str::from_utf8(melon) {
    println!("{}", s);
}
// prints 🍉
```

notes:
Or you can use if let to safely destructure the inner value, if it is OK.

---

```rust
let melon = &[240, 159, 141, 137];
match std::str::from_utf8(melon) {
    Ok(s) => println!("{}", s),
    Err(e) => return Err(e),
}
Ok(())
```

```markdown
(assuming inside `fn -> Result<(), std::str::Utf8Error>`)
```

notes:
Or you can bubble up the error, returning it to the calling function, which then handles it.
This pattern of unwrapping the value inside a result if it's OK, or returning it if it's an error is so common that rust has dedicated syntax to do it.
- [x] fix final t on 'do it'
---

```rust[2]
let melon = &[240, 159, 141, 137];
let s = str::from_utf8(melon)?;
println!("{}", s);
Ok(())
```

notes:

The question mark operator, at the end of Line 2, does the exact same thing as the larger match statement on the previous slide.

This is the normal rust error pattern in application code, where you're trying to just write the happy path, though the previous options are available to you when you need them.

Finally, let's talk about iterators.

---

```rust
let natural_numbers = 1..;
```

notes:
This is my favourite iterator. 
It's an iterator that represents all natural numbers from one to infinity.
This is possible to store in ram because iterators are computed lazily, on demand.

---

```rust
// 0 or greater
(0..).contains(&100); // true
// 20 or less than 20
(..=20).contains(&20)); // true
// only 3, 4, 5
(3..6).contains(&4)); // true
```

notes:
This iterator notation is called a range.
The most basic iterators are ranges.
The can be open at the bottom, or top, or you can specify both exactly.
Computation only happens when the iterator is called.

---
```rust
fn main() {
    for i in vec![52, 49, 21] {
        println!("I like number {}", i);
    }
}
```
output
```markdown
I like number 52
I like number 49
I like number 21
```

notes:
Anything that is iterable can be used in a for loop.
We've just seen a range being used, but it also works with a Vec.

---

```rust[2]
fn main() {
  for i in &[52, 49, 21] {
    println!("I like number {}", i);
  }
}
```
output
```markdown
I like number 52
I like number 49
I like number 21
```

notes:
Or a slice

---

```rust
fn main() {
    for c in "rust".chars() {
        println!("Give me a {}", c);
    }
}
```

output

```markdown
Give me a r
Give me a u
Give me a s
Give me a t
```


notes:
Or an actual iterator
nb string literals also have a `.bytes()` iterator if you want the raw bytes.
Rust's `char` type is a "Unicode scalar value" that is always a valid character.

---

```rust
fn main() {
  for c in "SuRPRISE INbOUND".chars()
    .filter(|c| c.is_lowercase())
    .flat_map(|c| c.to_uppercase()) {
    print!("{}", c);
  }
}
```
```rust
// output: UB
```

notes:

You can use an iterator in a for loop even if the iterator items are filtered and mapped and flattened.
This fluent interface pattern you will find everywhere in Rust.
Not classes, not mutating shared data.
Modelling your program's state as structs, and then writing functions to move between these valid states makes invalid states unrepresentable. 

---


```python
$ cargo build

error: this operation will panic at runtime
 --> no_segfaults.rs:2:15
  |
2 |     println!("{}", spam[6]);
  |                    ^^^^^^^ index out of bounds: 
        the length is 3 but the index is 6
```

notes:

Writing Rust is a very different experience to reading Rust. On one hand it's more difficult, you're not reading the solution to a problem, you're actually solving it. But the other hand, the Rust compiler helps out a lot.

For all of the intentional errors made throughout this video, the compiler always has very good error messages and insightful suggestions.

And when there's a hint missing, the compiler team is not afraid to add it.

---


   - www.fasterthanli.me
   - The Rust Book
   - Rust By Example
   - Read Rust
   - This Week In Rust


notes:

We are out of time!

I cut out lifetimes, closures and traits to keep this video fast. Read Amos's original blog post, which is linked in the description.

For more Rust material, I recommend the Rust Book or Rust By Example.

///

If you'd like to see what you can write in rust, click the top video: I used it to make a fun retro computer visualisation for my hopepunk podcast, Lost Terminal.

And if you'd like to watch more of my fast, technical videos, click the bottom video.

Transcripts and markdown sourcecode are available on github, links in the description.

Thank so much for watching, see you next time.
