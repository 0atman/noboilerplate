
![rust](attachments/rust-logo.png)
## Rust makes you feel
## like a genius

notes:

Hi friends my name is Tris and this is No Boilerplate, focussing on fast, technical videos.

When I write rust, I feel like a genius.

I'll tell you why in a moment, but first a simple question:

---

## Is this correct?
```javascript
function add_one(n) {
    return n + 1;
}
```

notes: 
Is this code correct?
Well...

---
## It depends...
## on the context

```javascript
function add_one(n) {
    return n + 1;
}
```

- Is `n` always a number?
- How large is `n`?
- Could `n` be modified while we're reading its value?
- and so on...
- and on...

notes:

If you or a colleague wanted to be totally sure this function could always work, you'd have to add lots of boilerplate code:
- You'd check the type of n, erroring if it wasn't a number
- You'd Make sure n wasn't so large that it caused some kind of overflow
- And what about negative numbers?
- Or floating point numbers?
- Perhaps we'd ensure that n was passed by value not by reference
- or that we capture the lock on the part of shared memory that n was in.

Sure most languages have sensible defaults for many of these, but do you have all these rules and edge cases in your memory as you're writing code?

ALL THIS UNCERTAINTY AND WE JUST ADDED ONE TO A NUMBER

How can we hope to keep a whole language's assumptions all in our heads?
We need a notation to concisely encapsulate all this. 

Rust provides this notation in the rich type system.

We'll look at it in a moment.

---


![rust](attachments/loved-languages.png)
<3

note:
As many of you know, Rust has topped the Stack Overflow developer survey for most loved language 6 years in a row.

Compiler feedback is the secret to the satisfaction of writing rust, I think.

It makes me feel like a genius, my code works FIRST TIME when I run it on production.

Before I tell you why rust makes you feel like a genius, let me show you how other languages makes you feel STUPID by forcing you into what I'll call Error-Driven-Development.



---

## NO ERRORS
```javascript
let spam = ['cat', 'dog', 'mouse']
console.log(spam[6])

// no output, no errors, spam[6] is `undefined`
```

notes:
The worse case is no errors.

You run the code and it doesn't do what you want, but there are also no errors.

You just have to sit there and imagine _why on Earth_ you can't get your error! Because javascript's certainly not going to tell you.

Without annotations, even typescript can't help you either.

---


## Bad Errors
```markdown
📄 segfaults.py
```
```python
spam = ['cat', 'dog', 'mouse']
print(spam[6])
```

Python ~~error~~ traceback

```python
$ python segfaults.py

Traceback (most recent call last):
  File "segfaults.py", line 2, in <module>
    print(spam[6])
IndexError: list index out of range
```

notes:

MUCH BETTER THAN NO ERRORS are bad errors.
Let's look at how Python handles this.

Not as well as you might think, it turns out:

python won't tell me exactly what's wrong here.
It's only giving me SOME of the information.
And this is typical with most languages.

Sure, once you know what to look for you can see that a list index is out of bounds.
But there's SO MUCH MISSING HERE.

Which list, what bound, where even in line 2 is it?!

Errors in dynamically typed languages are usually bad.
And it's not necessarily because the compiler authors didn't put in the work.
They're doing the best they can.
If you have a simple type system, the compiler simply can't catch many errors for you.

By the way, it might look like Python has caught this error, but it actually didn't.
Look again: This error happened at runtime, 
your user has caught this error.
If you're lucky, your tests might catch this error.

Here's how it should be:

---

## Good errors

```markdown
📄 no_segfaults.rs
```
```rust
let spam = ["cat", "dog", "mouse"];
println!("{}", spam[6])
```

rust ~~traceback~~ error

```rust
$ cargo build

error: this operation will panic at runtime
 --> no_segfaults.rs:2:15
  |
2 |     println!("{}", spam[6]);
  |                    ^^^^^^^ index out of bounds: 
        the length is 3 but the index is 6
```

note:

Rust's errors are the nicest I've ever seen.

Just look at it:
 - We have the error itself
 - What the value should be, no more than 3,
 - and what the value actually was, 6
 - We don't have to remember what the value is, the compiler has told us.

Rust's errors happen at compile time, which is a fancy way of saying they MUST happen on the developers machine, not on a server somewhere.

---

## The rust compiler
## is your wingmate


```rust
error: format argument must be a string literal
  --> src/main.rs:13:22
   |
13 |             println!(n);
   |                      ^
   |
help: you might be missing a string literal to format with
   |
13 |             println!("{}", n);
   |                      +++++
```

note: 

lets get a little deeper.
Say I want to print out a number inside a loop for debugging purposes. 
But woops, like in C, numbers need a format string to be printed.

Just look at this error message.
Have you ever seen something so beautiful?

It's not only telling us the error in plain language, but visually pointing to exactly where the problem is, and THEN suggests what we should do to fix it!

I think this is why rust has been voted most loved language.

The compiler is like a driving instructor, gently coaching you on how to navigate the dangerous highway.

This is another simple example, but the rust compiler holds your hand in this way right through async network programming, multi-processing, and through channels and locks, and all this is available to anyone via the macro system, so that third party web libraries and frameworks have these rich errors too.

The generous, kind, thoughtful Rust developers told us that it's dangerous out there, take this.

---

## Your code won't crash

With Rust it is easily possible to write code that has no execution paths that crash at runtime.

notes:

The Rust team set out to solve the most difficult problem we face, in c, which is how to handle memory safely.
The fixed it with the Borrow Checker, which I'll explain in a moment.
Because they solved the most difficult problem the hard way, not cheating with a garbage collector, or leaving it up to the developer, it was easy to use this solution to solve all the other problems, by hooking into this new way of programming.

If you make a compiler that understands your program's memory exhaustively, then you have made a compiler than understands your code exhaustively.

And Rust isn't some theoretical language that is only used in universities or by esoteric wizards.
All the components are already here TODAY to make a language that stops you making mistakes, and helps you if you break the rules.

I've talked as long as I can without telling you about The Borrow Checker.
The good news is that it's extremely simple.
It's the effects that are profound.

---

## The Borrow Checker

Has two rules:
- Data has one owner
- Data may have multiple readers or one writer

notes:

That's IT. Those are the two rules of the borrow checker.
All behaviour can be explained by these two.

You can think of Data as a variable, though it's really the data that variable points to.

An example will make this clear.

---

## THE NAME EXAMPLE
_Demonstrating ownership_

```rust
let name = "tris";
let capitalized_name = capitalize(name);

println!(capitalized_name); // OK
println!(name); //error: `name` moved
```

notes:

In rust, you don't pass a variable into a function.
You give ownership of the variable to that function.
Here, when you've given name to the capitalize function, it's GONE.


---

## The Library EXAMPLE
_Demonstrating ownership and borrowing_


```rust []
let neuromancer = Book {};

loan_to_lucy(&neuromancer); // borrow
loan_to_nia(&neuromancer); // borrow

// we are still owner of neuromancer, it is just borrowed

withdraw_book(neuromancer);  // give up ownership to fn

// we are not allowed to use `neuromancer` anymore

loan_to_priya(&neuromancer); // err: neuromancer moved
```

notes: 

Imagine we run a library.
That sounds nice doesn't it!

At our library we have essentially infinite copies of our books we can loan out.

When we loan out our book on lines 3 and 4, we are guaranteed a few things.
- The person who is borrowing our book is strictly forbidden from changing the text.
- They can only read the book until they give it back (after the loan function has finished)
- No doodling on the cover.
- And STRICTLY no dog-earing the pages.
That's just rude.

If we, the Library, remove the book from circulation, line 8, perhaps because the publisher has asked us to, we can no longer loan out the book.

When passing a variable into a function, you are giving up ownership of the variable. ie, it will be cleaned up when the function is done with it.

---

## The Author Example
_Demonstrating ownership and mutable borrowing_

```rust []
let neuromancer = Manuscript {};

let ace_books = Editor {};
let molly = Editor {};

edit(&mut neuromancer, ace_books); // mutable borrow
edit(&mut neuromancer, dave); // err: only one mut borrow

// we are still owner of neuromancer, it is just borrowed

sell(neuromancer);  // pass ownership

// we are not allowed to use `neuromancer` anymore
loan(&neuromancer); // err: neuromancer moved
```


notes:

In this example we are the author of Neuromancer, William Gibson.
Imagine that!

It's 1983, and we have an unfinished manuscript, not a finished book.

This typewritten first draft we want to get in front of our editor, for them to make changes, draw with their red pen all over it, and hand it back to us, improved.

We have only one manuscript.

We could photocopy it, but then we'd have to reconcile multiple edits on paper.

No thank you.

We give Ace Books, our editor, a mutable borrow to our manuscript.

They and only they can change it.
They must give it back when they are done.

We can't let it be mutably borrowed again, we have to wait for the first function, our editor, to return it.

`TODO IS THIS RIGHT?`

This is how ownership and borrowing work at a very low level.

This system was designed to keep track of memory, and to be able to free it when it is no longer used.

But you can use this system to design MUCH more complex invariants for your programs!

Let's look at a practical, high-level example of the repercussions of this simple system.

---

> Do not communicate by sharing memory; instead, share memory by communicating.
 
&mdash;Effective Go

note:

It's really nice that the Go team designed their language this way. 
It's the right way to handle shared memory.

But they don't stop you sharing memory, which leads to problems.

**Rust's ownership made it easy to turn this recommendation into a compiler-checked rule**.

---

```rust
// Suppose channel: Channel<Vec<String>>

let mut users = Vec::new();
// do some computation, maybe append some usernames

channel.send(users);
print_vec(&users);
```

Results in:

```
Error: use of moved value `vec`
```

note:

Now you understand ownership, I think you can read this.
I have very high standards for you.
Pause the video if you need to.

We've created a list of users, and sent it down a channel to some other thread or process or machine.
After we've sent it, who knows what will happen to the data in there.
It is unsafe to even READ the user list after we've sent it.
The borrow checker, despite being created to keep memory safe, can be used trivially to create compile-time-checked guaranteed safe channels.

The thread receiving users could modify it as this first thread continues running, so the call to `print_vec` could lead to race condition or, for that matter, a use-after-free bug.

You don't even need to TEST this code to find these race conditions. The rust compiler won't let you compile.

In the Rust community, we call this Fearless Concurrency.

Find out more:
https://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html

---


## Options Everywhere

notes: 
In rust, Options are everywhere.
Because, as in life, you can't always get what you want. 


---
## Ways to test null in js

```js
typeof null          // "object" (not "null")
typeof undefined     // "undefined"
null === undefined   // false
null  == undefined   // true
null === null        // true
null  == null        // true
!null                // true
isNaN(1 + null)      // false
isNaN(1 + undefined) // true
```

Who is flying this thing?

note:

No one remembers all this. It's byzantine and cruel.

Your life doesn't have to be this way.

---

```rust
enum Option<T> { // T can contain any type of value
    Some(T),
    None,
}

let possibly_a_number = Some(1);

possibly_a_number.map(|n| n + 1).unwrap_or(0); // = 2
```


notes:
Everyone agrees that nulls are bad until it's time to return nothing from a function.

Wrapping a type in an `option` signifies clearly to the programmer, the IDE, and the compiler, that this value might be something, or it might be nothing.
It's not that it's zero, or an empty string, it might just be nothing.

Nothing, the value, is useful, and the rust compiler will force you to deal with it. 

---
## Results everywhere

```
📄 go-errors.go
```
```go
f, err := os.Open("filename.ext")
if err != nil {
    log.Fatal(err)
}
```

notes:
Just as options transform nulls in rust, Results transform errors.

In Rust, errors are values.
You might have seen this in other languages, like here, in go.

Here err contains the error, and you must check it before accessing `f`.

But there's nothing stopping you ignoring the error, in fact this happens all the time.

---

A result can represent either success or failure

```rust
enum Result<T, E> { 
    Ok(T),
    Err(E),
}
```

`T` and `E` can contain any type of value,

`E` can be any error.

notes:

Rust captures this pattern in the `Result` type, so that the compiler can force you to handle the error.

---

## Zero cost abstractions

notes:

Nearly all of this that I've shown you doesn't exist at run time.

Your rich type hierarchy doesn't sneak on to your customers laptop.

The chain of optional functions resolving down nicely into a safe value with no errors doesn't run on your user's phone.

Even the borrow checker writes itself out of the code as you compile it into your gpu-accelerated doom clone running in webassembly.

---

```rust
pub fn sum_loops(n: i32) -> i32 {
    let mut sum = 0;
    for i in 1..n {
      sum += i;
    }
    sum
}
```

IS EQUIVALENT TO

```rust
pub fn sum_iterators(n: i32) -> i32 {
   (1..n).sum()
}
```

notes:


Rust's incredible richness and complexity only exists at compile time, after you have told the compiler how the world works by notating your code with all this markup, the compiler exhaustively proves that nothing could violate the contracts you have put into code.
If everything checks out, all this information is stripped out of the code as it is compiled into low-level assembly.
Types don't exist at runtime.
This is actually the way the world works. 
We invented types.
They're like a logical fiction.
They only exist in our minds.
CPUs don't know anything about types, they only know 1s and 0s and a few operators.

Both of these two code blocks compile down to EXACTLY THE SAME assembly.

---

```elixr
example::sum:
        xor     eax, eax
        cmp     edi, 2
        jl      .LBB0_2
        lea     eax, [rdi - 2]
        lea     ecx, [rdi - 3]
        imul    rcx, rax
        shr     rcx
        lea     eax, [rcx + 2*rdi]
        add     eax, -3
.LBB0_2:
        ret
```

Your code can be both HIGHER LEVEL and FASTER in rust than it could if you had hand-optimised it.

---

# FREE ~~BEER~~ STUFF
- All of AWS
- All of the MS Windows API
- Webassembly
- Iterators, Options, and Results
- Memory safety

notes:

Here is a non-exhaustive selection of some of the things the rust compiler can validate for us for free.

---



> In rust, you tell the compiler how the world works, and it will hold you and everyone who contributes to your code accountable to the contract you have written."

**—Rust: Your Code Can Be Perfect**

notes:

In my last video I told you that in Rust, you tell the compiler how the world works, and it holds you accountable to this contract.

The rich type system is how you write the contract, the compiler built on the borrow checker holds you to it.

This all means your code works perfectly, the first time you deploy it, and makes you feel like a genius.

---

notes: END SCREEN


If you'd like to see what you can write in rust, click the top video: I used it to make a fun retro computer visualisation for my scifi and mental health podcast, Lost Terminal.

And if you'd like to watch more of my fast, technical videos, click the bottom video.

Transcripts and markdown sourcecode are available on github, links in the description.

Thank so much for watching, see you next time.
