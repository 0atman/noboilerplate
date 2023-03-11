---
highlightTheme: css/vs2015.css
---

%%

#tasks
- [x] Make all graphics and text as large as possible
- [x] Flow correctly
- [x] Set out the thesis then prove it
- [x] rename 'Section' to 'section'
- [x] rename 'class' to name
- [x] SPELL CHECK YOU IDIOT
- [x] add a screenshot of the game running when talking about journelling games
- [x] add ERRATA with todo!();


<style>
.reveal code.rust {
  font-size: 1.5em;
  line-height: 1.5em;
}
</style>

<style>
.reveal code.md {
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
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
```

# extern crates

```rust
extern crate inquire;
```

# imports
```rust
use strum_macros::Display;
use strum_macros::EnumString;
use std::str::FromStr;
use inquire::Text;
```

# setup

```rust
fn main() {

```

%%

![[rust-logo.png|300]]

# Building a Space Station in RUST

notes:

Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

Today we're going to build the foundations of a single-player journalling game, based on my Scifi podcast, Lost Terminal, about a little AI alone on a space station.
%% pin card to trailer video %%

I'm going to show you what I think is the best way to design Rust programs, which is to:

---


# Make invalid states unrepresentable

notes:

Much of Rust's ergonomics comes from its rich type system.

The more detail you put in to your typed model, the more the compiler can help you, and in many cases, suggest correct code for you.

But how do we design programs in Rust? There's no classes!
As we go through this design process, I will show parallels with traditional OOP design patterns, and that starts with replacing Classes.

---

```rust
struct Dog {
    name: String,
    breed: String,
    age: u8
}
```

```rust
impl Dog {
    fn nametag(self) -> String {
        format!("{{self.name}}{{self.age}}")
    }
}
```

notes:

- [x] re-take this page

Rust loosely couples data and behaviour. but as you can see, the combination of structs for data and implement blocks for behaviour should be a very familiar analogue for a class, though it is more flexible, as we will see.

---

# let's build our space station

---
%%

```rust
//use std::string::ToString;
//use strum_macros::Display;
```
%%

## Use enums to constrain 

```rust
use rand_derive2::RandGen;
```
```rust
#[derive(Debug, RandGen, Display)]
enum Name {
   Akira,     Californa, Daedalus,
   Eisenberg, Intrepid,  Miranda,
   Nova,      Reliant,   Sagan 
}
```

```rust
let a_name: Name = rand::random();
// equals, possibly `Name::Nova`
```

notes:

We start off naming our satellite.

- [x] re-record this line

Starting at the top, the derive line implements the `Debug` trait for this enum.
- The Debug Trait allows for printing the struct in programmer-friendly format, which saves a lot of time while you're prototyping,
- RandGen is part of the `rand_derive2` crate, allowing us to randomise our structure's data,

Our satellite is going to be named one of these names.
I could have used a string here, but that doesn't allow the type system to give us any superpowers.

For instance, I could use the `rand` crate to generate a random string, but it would be a bunch of random characters. Not what We're looking for.

Let's plug our Name enum into the station.


---

## Use structs to contain

```rust
#[derive(Debug, RandGen)]
struct Station {
    name: Name,
    version: u8,
    sections: Vec<Section>
}
```

notes:

Here is our space Station struct.

It has a name, which is one of the limited collection of valid names which NASA, The Federation or whoever allows us to use,

A version number so we can tell the different satellites that are of the same name apart,
and a vector of sections.
As you will recall, a vector in Rust is like a list in other languages.
There can be any number of Sections, including none.

---

## Constructor Pattern

```rust
use rand::random;
```
```rust
impl Station {
    fn new() -> Self {
        Station {
            name: random(),
            version: random(),
            sections: (0..10)
                .map(|_| random())
                .collect()
        }
    }
}
```

notes:

Our first rust pattern is the constructor pattern.

Rust structs do not require a method like this to create them, you can just do it directly, however very often you want to control how a struct is created.

The convention is to implement the `new()` method, as I have done here. In my `new()` method, it randomises the Station's name, version and sections.

We are using the incredible `rand` crate to do this, you will notice that no matter what type we are randomising for, we don't need to call different functions. This is worth highlighting as it's another superpower you get with Rust's type system:

---

## Type Inference is magic

```rust
use rand::prelude::*;
```

```rust[]
let rand_int: u8  = random();

let rand_ch: char = random();

let rand_int: f32 = random();

let mut nums: Vec<i32> = (1..100).collect();
nums.shuffle(&mut rand::thread_rng());
```

notes:

The `rand` crate lets you generate all kinds of random values.
On line 1 we see a random int, line 3 a random character, line 5 a random float, and more complexly, on lines 7 and 8 we shuffle a vector.

Importing the rand prelude gets us the trait that adds the `.shuffle()` method to vecs and many other handy features.

But how does the compiler know what kind of random value to return from these `random()` functions?

This is called type inference, and unlike in java and many other languages, the compiler can infer based on BOTH SIDES of the equals sign, left as well as right.

This means typically, you only need to add type annotations to function signatures, and the compiler figures out the rest.

---

## Structs and enums

```rust
#[derive(Debug, RandGen, Eq, PartialEq)]
struct Section {
    name: SectionName,
    active: bool,
}
```


```rust
#[derive(Debug, RandGen, Display, EnumString)]
#[derive(Eq, PartialEq)]
enum SectionName {
    AstroScience,     Solar,       Antenna,
    RadiationMirrors, Sleeping,    NuclearGenerator,
    Galley,           Transponder, Tracking
}
```

notes:

back to the space station, here I've defined a Section.
A Section is a room on our station specialised for one task. 

Note that you can define your structs and enums in any order, here we are using the SectionName enum before defining it.

I like this because it feels a step closer to literate programming, which represents a move away from writing programs in the manner and order imposed by the computer, and instead enables us to develop programs in the order demanded by the logic and flow of our thoughts.

(Talk to Don Knuth for more on that)

For our simple game, a Section is either active or inactive. And once again, we'll define the valid Section names, as these are standard on satellites.
We'll randomise the configuration for our game when we implement it later.

---


![[lt space.png]]

notes:

As I said, the game we are building is based on my hopepunk podcast, Lost Terminal, which follows the journey of a little satellite trying to understand what has happened after Earth stops returning his calls.

It is set an unknown number of years after the global climate apocalypse, where we find out what has happened back on earth through the lens of our narrator, an AI named Seth.

---

![[lost-terminal-crt.png]]

notes:

I write and perform every episode, as well as having composed the music. 

I coded the first two seasons as a cli app and screenrecorded them though a CRT terminal emulator called Cool Retro Term.

(which is s a general-purpose posix terminal that runs on linux and osx, you should try it, it's amazing!)

Hopepunk is a nice genre to be writing stories in, it's a kind of antidote to grimdark, where things go from bad to worse and only the bad guys win.
In Lost Terminal, though the world as we know it has ended, rebuilding is possible.

---

![[Lost Terminal logo.png|600]]
# LostTerminal.com

notes:
Every ad-free episode is available on spotify, itunes, youtube or the dozens of other normal places you would expect to find a podcast.
Links are on LostTerminal.com, and I'll link to the first episode at the end of this video.

I might be biased, but I think you're going to love it.

Let's continue building our own little space station.

---

## Building a menu

```rust
use inquire::Select;

/// Build a simple menu based on `items`
fn menu(items: &[String]) -> String {
    Select::new("MENU", items.to_vec())
    .prompt()
    .unwrap()
}
```

notes:

Here we're using the `inquire` crate, a terrific command line interaction crate that allows us to build interactive menus, auto-completing lists, input lines with good readline support and more.

We'll be using just the menu and text input from it in our game.

Here I've wrapped inquire's select struct, which will create our menu system in-game.
The second argument to the new constructor of Select is the vector of string menu options.

(6:00)

---

![[inquire-menu.png]]

notes:

Here's what it will look like when we run our game, note the cursor, if you type, it fuzzy finds the menu item you are looking for.
Very cool!

---

## Adding methods to our station struct 

```rust
impl Station {
    fn days_left(&self) -> usize {
        self.sections
            .iter()
            .filter(|m| m.active)
            .count()
    }
}
```

notes:

Later on in our code, we're defining a `days_left()` method for our Station struct.

Our game ends when the number of working sections reaches zero.
Future versions of our game could have different categories of Section. Perhaps as long as the solar panels keep your batteries charged, you wake up in the morning and continue work?

That's how it is in Lost Terminal, anyway.

Note the first parameter of our method is called `self` - how familiar coming from object orientation!
For this method I chose to make `self` a read-only borrow, which is what the ampersand means.
As you can see in the body, I do not mutate the Station.

This method is a good example of Rust's Iterator pattern, most lists of things in Rust are either iterators, or can be made into iterators like here, using the `.iter()` method.

---

## rustc is your wingmate

```js[6]
 1  `std::vec::Vec<main::Section>` is not an iterator
    --> src/main.rs:77:14
     |
 77  |             .filter(|m| m.active)
     |              ^^^^^^ `std::vec::Vec<main::Section>`
is not an iterator try calling `.into_iter()` or `.iter()`
```

notes:
By the way, if you forget to use the `iter()` method on the sections vector, as I did initially, don't worry the compiler will tell you how to fix it!.

---

```rust
impl Station {
    fn working_sections(&self) -> Vec<String> {
        self.sections.iter()
            .filter(|m| m.active)
            .map(|m| m.name.to_string())
            .collect()
    }
    fn broken_sections(&self) -> Vec<String> {
        self.sections.iter()
            .filter(|m| !m.active)
            .map(|m| m.name.to_string())
            .collect()
    }
}
```

notes:
Let's write two more helper methods that give us a vector of strings of working station sections, and one of broken sections.

Note that we can make as many implement blocks for Station as we like.
They don't all have to be in the same place, though for organisation you might like to refactor them into one spot.

These two methods are quite similar to the last one, so we will continue.

---

## Markdown presentation available on GitHub
https://github.com/0atman/noboilerplate

notes:
It's worth mentioning at this point that all the codeblocks in my presentation are passed through the compiler.

How do I do this?

The codeblocks are stripped out of my markdown and concatenated into the main.rs live as I write my presentation.

Not all code is shown, imports and lint config and such are hidden for brevity, but they are all in the markdown for you to examine at you leisure.

Please be kind about my code, I'd love suggestions, and I'm still learning a lot about rust every day.

Onwards!

---

## Everything not saved
## will be lost

```rust
// fixes `broken_section` on a `station`
fn repair(broken_section: String, station: &mut Station) {
    let section = SectionName::from_str(
        broken_section.as_str()).unwrap();
        
    let broken_index = station.sections.iter()
        .position(|m| m.name == section)
        .expect("Section not found.");
        
    station.sections[broken_index].active = true;
}
```

notes:
We're now digging into the power of enums.

Back when I defined our `SectionName` enum, I snuck in a crate called `strum` which allows converting enum variants to strings,
and strings to enum variants.

That is what we are doing here in the `from_str()` method that was added to our enum by deriving the `EnumString` trait.

I'm not delighted by having to resort to indexing the vector, I wonder if there is something more clever I could be doing? Do let me know if so!

%%

```rust
fn science(working_section: String, station: &mut Station) {
    station.break_something();
}
```

```rust
impl Station {
    fn new_day(&mut self) {
        self.break_something();
    }
    fn break_something(&mut self) {
       let broken_index = thread_rng().gen_range(0..self.sections.len());
        let mut broken_section = &mut self.sections[broken_index];
        if broken_section.active {
            broken_section.active = false;
            println!("(Section-FAILURE {})", &broken_section.name);
        } else {
        println!("(sections OK)");
        }
    }
    fn status(&self) {
        dbg!(&self);
    }
}
```

%%

---

## Main loop

```rust
let mut station = Station::new();
let mut station_log = vec![];
```
```rust
loop { // main game loop!
    let days_left = station.days_left();
    if days_left < 1 { 
        println!("(END-TRANSMISSION)");
        break
    }
    println!("{days_left} UNTIL FINAL TRANSMISSION");
    station_log.push(Text::new("Enter your log:")
        .prompt().unwrap());
    match menu(&[
        "NEW DAY".into(), "STATUS".into(),
        "POWERDOWN".into()]).as_str() {
        "NEW DAY" => {
        // menu system defined after here
```

notes:

Finally, we have our main game loop.

Every day we wake up, find out what has broken on our space station, and decide if we want to spend our time fixing it, or spend our time doing science while we still can.

First, we start an infinite loop, we will break out of it only when the days left are 0, which when the game ends, or if the user volunteers to powerdown early, in the menu at the bottom.

---

![[space-station-demo.png]]

notes:

Each day we prompt the user for their station log, where they can journal how their last day went, what science experiments they had run, what sections had broken overnight and more importantly, how they feel about all that.

Though we've built a very simple game in terms of mechanics, the genius of journelling games are to provide a structure onto which the imagination of the user can be allowed to run free. 

%%
```rust
            station.new_day();
            match menu(&["REPAIR".into(), "SCIENCE".into()]).as_str() {
                "REPAIR" => {
                    repair(menu(&station.broken_sections()), &mut station);
                    continue;
                },
                "SCIENCE" => {
                    science(menu(&station.working_sections()), &mut station);
                    continue;
                }
                &_ => panic!(),
            }
        },
        "STATUS" => station.status(),
        "POWERDOWN" => break,
        &_ => panic!("test"),
    }    
}
dbg!(station_log);
```
%%

---

## To write rust is to model the valid states of a system

Then write functions to move between those states.

notes:

It seems to me that (read slide).



---

> Show me your flowcharts and conceal your tables, and I shall continue to be mystified. Show me your tables, and I won't usually need your flowcharts; they'll be obvious.

&mdash; Fred Brooks

notes:

This way of designing systems is the way [Fred Brooks](https://libquotes.com/fred-brooks) recommended in his 1975 book _The Mythical Man-Month_.

Fred's words are what the Rust compiler would say, I think, if it could talk.

Make the compiler's job easy, and you'll make your job, your team's job, and your future self's job, easy too.

---

%%%%

# OUTRO


If you'd like to see what you can write in rust, click the top video: I used it to make a fun retro computer visualisation for my hopepunk podcast, Lost Terminal.

Or if urban fantasy is more your bag, click the bottom video to listen to a strange and beautiful podcast I produce called Modem Prometheus.

Transcripts and compile-checked markdown sourcecode are available on github, links in the description, and corrections are in the pinned ERRATA comment.

Thank you so much for watching, see you on Discord!

---

%% Thumbnail %%


![[rust-logo.png]]

## make invalid states unrepresentable 
---

```rust
} // That's all folks!
```
