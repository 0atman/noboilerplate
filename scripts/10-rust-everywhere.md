%%
<style>
:root {--r-code-font: "FiraCode Nerd Font";}
</style>

# Cargo.toml

```toml
[package]
name = "template"
version = "0.1.0"
edition = "2021"

[build-dependencies]

[dev-dependencies]

[dependencies]
console_error_panic_hook = "0.1"
web-sys = { version = "0.3", features = ["Window", "Document", "HtmlElement", "Node", "Text"] }
wasm-bindgen = "=0.2.74"
wee_alloc = "0.4"
```

# Lint Tweaks

```rust
#![allow(dead_code)]
#![allow(unused_variables)]
#![recursion_limit = "1024"]
```

# Extern Crates

```rust

```

# Imports

```rust
```

# Setup

```rust
fn main() {
	println!("Rust & Wasm");

```

%%

![[rust-logo.png|300]]

# Rust & Wasm

### Safe & Fast Web Development

notes:

Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

Today I'm going to talk about building apps you can deploy anywhere with no installation by using Rust and web assembly.

---

## Open Source Videos

```sh
$ git clone git@github.com:0atman/noboilerplate.git
$ cd noboilerplate/scripts

$ make deps  # to install literate and cargo-watch
$ make build # produce a valid cargo project
$ cargo build
```

notes:

As ever, all Rust code you see in this video is part of a literate programming document that can be extracted and compiled with native Rust tooling.

---

|             |                                                                     |
| ----------- | ------------------------------------------------------------------- |
| Docker      | [hub.docker.com/\_/rust](https://hub.docker.com/_/rust)             |
| Games       | [bevyengine.org](https://bevyengine.org) etc                        |
| Backend     | [rocket.rs](https://rocket.rs) etc                                  | 
| AWS Cloud   | [aws.amazon.com/sdk-for-rust](https://aws.amazon.com/sdk-for-rust/) |
| Windows     | [crates.io/crates/windows](https://crates.io/crates/windows)        |
| Webassembly | [rust-lang.org/what/wasm](https://www.rust-lang.org/what/wasm)      |

notes:

Rust runs everywhere,
- on servers and in containers
- game engines running at native C speed
- backend web frameworks, allowing safe, fast web development
- In the AWS cloud with first-class support from amazon and is the cheapest language to run on aws lambda
- on windows with first class support from Microsoft
- And naively in the browser with Webasm

---

## Native Performance Apps

### With

## Web Distribution

notes:

Webassembly gives you native performing apps with the ease of web distribution.

For example:

---

![[wavacity.png|800]]
<https://wavacity.com>

notes:

Wavacity is a port of the popular audio editor, Audacity, to webassembly.
The author even ported the C++ interface library, wxWidgets, along with the app.

---

![[wavacity-mobile.png]]

Literally my phone

notes:

When you deploy to webassembly you get the mobile app for free.

---

![[doom-wasm.png|700]]

Literally DOOM

notes:

And with webgl being standard in all modern browsers, you can run anything, including DOOM!

---

![[doom-wasm-mobile.png]]

Literally DOOM on my phone

notes:

which of course runs on your phone too.

---

## Supported Languages

|                |        |
| -------------- | ------ |
| C/C++          | Kotlin |
| Go             | Swift  |
| AssemblyScript | D      |
| C#             | Pascal |
| F#             | Zig    |
| 🦀 Rust        |        |

notes:

But there is a snag with webassembly
- Here are the languages from webassembly.org that have first class support.
- Notice a problem?
	- no python
	- no ruby
	- no java
	- not even javascript

That's not to say that these languages aren't in the process of being ported to webassembly. With emscripten, any C++ program can be ported, but these languages aren't a good fit for webassembly due to their heavyweight runtimes.

- This has hampered wasm adoption, these wasm languages are so drastically different from the popular high-level languages of the web.
- but I spot a language in there that I DO like, and Rust was one of the earliest adopters of webassembly, more than 5 years ago.

---
## Wasm usage

![[webasm-usage-scottlogic.png|400]]

[State of Wasm 2022, ScottLogic.com](https://blog.scottlogic.com/2022/06/20/state-of-wasm-2022.html)

notes:

Colin Eberhardt has run a small survey of webassembly usage for two years now, and in all metrics, Rust crushes the alternatives.

This makes sense, despite these other language's popularity, they are not good fit for a webassembly environment.

Rust is an excellent language to write webasm in:

---

## Predictable Performance

## Small Code Size

## Modern Ergonomics

</br>

<https://www.rust-lang.org/what/wasm>

notes:

There's no unpredictable garbage collection pauses. No JIT compiler performance cliffs. Just low-level control coupled with high-level ergonomics.

Small code size means faster page loads. Rust-generated webassembly doesn’t include extra bloat, like a garbage collector. Advanced optimisations and tree shaking remove dead code.

And Rust has a lively ecosystem of compatible libraries to help you hit the ground running, with expressive, zero-cost abstractions, and a welcoming community to help you learn.

Let's have a look at Rust's take on webassembly.

---

Simple DOM manipulation

%%

```rust

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use console_error_panic_hook::set_once as set_panic_hook;
use wasm_bindgen::prelude::*;
use web_sys::window;

```

%%

```rust
fn start_app() {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    
    let text_node = document.create_text_node("Hello");
    let h1 = document.create_element("h1").unwrap();
    
    h1.append_child(&text_node).unwrap();
    body.append_child(&h1).unwrap();
}
```

recognise this?

%%

```rust
#[wasm_bindgen(inline_js = "export function snippetTest() { console.log('Hello from JS FFI!'); }")]
extern "C" {
    fn snippetTest();
}

fn main() {
    set_panic_hook();
    snippetTest();
    start_app();
}
```

%%

notes:

Here's the lowest-level DOM-manipulating webassembly code you can write in Rust.
It's so low level it's grabbing the window and document, then generating nodes and appending them to the DOM manually.

But, maybe you recognise this kind of code?
Where have you seen this sort of thing before?

---

## Rust

```rust[]
let h1 = document.create_element("h1").unwrap();
let text_node = document.create_text_node("Hello");
h1.append_child(&text_node).unwrap();
```

## JS

```js[]
const h1 = document.createElement("h1");  
const textNode = document.createTextNode("Hello");  
h1.appendChild(textNode);
```

notes:

It looks a lot like javascript doesn't it!
Only, with no nulls, a rich type system, and an ecosystem that doesn't make you want to pull your hair out!

This is of course not how we'd write browser code today, in JS or in Rust, I'm just showing you the fundamentals.

[MDN Document/createElement](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElement)

just as you don't directly manipulate the dom with js, we won't do so with vanilla rust

Today we're going to look at the most popular high-level frontend webasm framework in Rust, which is called Yew, right after this word from this video's sponsor, Quadratic.

---

<!-- slide bg="rgb(37, 34, 43)" -->

![[Quadratic Logo.png]]

notes:

- Quadratic are building an Open Source spreadsheet for engineers and data scientists, built in Rust, Webassembly and WebGL

---

<!-- slide bg="rgb(37, 34, 43)" -->
![[quadratic choose cell type.png]]
notes:

- This might be the coolest spreadsheet I've ever used, and I've used emacs!
- You can choose your formula language,
  - Either simple excel-style statements, or
  - SQL and Python, both standards in the field of Data Science
- Because all data is evaluated in Webassembly, Quadratic is fast
- The UI is in WebGL, with hardware acceleration in all modern browsers, allowing 60fps scrolling, complex graphics, and smooth pinch-to-zoom

This is a really exciting project that I'm delighted to say is hiring.

---


<!-- slide bg="rgb(37, 34, 43)" -->
# Hiring for

<split even>

![[rust-logo.png|100]]

![[WebGL_Logo.png]]

![[arrow-inverse.png|100]]

![[scripts/attachments/WASM.png|90]]

</split>

notes:

Quadratic are looking for:
- Rust developers
- People with WebGL experience, even if that's only with JavaScript
- People with Apache Arrow experience for processing Quadratic's high-performance datasets, and,
- Senior engineers used to working at the pace of a startup.

---

<!-- slide bg="rgb(37, 34, 43)" -->

![[Quadratic Logo.png]]

<https://github.com/quadratichq>

<https://careers.quadratic.to>

notes:

Check out and star the project on github at github.com/quadratichq

and view their open jobs at careers.quadratic.to

My thanks to Quadratic for their support of this channel.

Let's see what high-level webassembly looks like in Yew.

---

# Yew.rs

![[yew.png]]

```toml
yew = "0.19"
```

notes:

Yew is a rust framework for creating multi-threaded front-end web apps using WebAssembly.

It is a component-based framework, like React and Elm.
Though perhaps it is more correct to say that they're ALL functional programming - based, and React is simply the most well-known.

Though webassembly is slower than javascript at the time of recording, yew is faster than react.

I suspect that as webassembly is optimised in browsers more and more, we will see frameworks like Yew take the lead in performance.

But it's already faster than react, so I say ship it!

Yew is designed with webassembly's bidirectional javascript support in mind, allowing access to the npm ecosystem.

If you're in to that.

Let's see some code

%%

```rust
#[derive(Properties, PartialEq)]
struct Props {
    text: String,
}
```

%%

---

## Hello Yew

```rust
#[function_component(HelloComponent)]
fn Hello(props: &Props) -> Html {
    html! {
        <p>{ &props.text }</p>
    }
}
```

notes:

No logic here, just a paragraph created in our DOM.

This is the simplest component, but look at that html!() macro! Yes, that's actual html inside your Rust code.

This isn't achieved by some kind of external preprocessor like react requires with JSX, this is native Rust.

At compile-time, the html!() macro re-writes that html into a yew data structure that builds valid html and interpolates the props for us.

---

## [yew](https://docs.rs/yew/0.19.3/yew/index.html)::[html](https://docs.rs/yew/0.19.3/yew/macro.html.html#)

```html
 error: this closing tag has no corresponding opening tag
   --> src/main.rs:4:27
  |
  |         <p>{ &props.text }</BADCLOSINGTAG>
  |                           ^^^^^^^^^^^^^^^^
```

notes:
The Yew HTML macro even understands HTML tags and attributes, and will give you syntax errors within Rust's normal compiler errors because they ARE normal rust compiler errors.

This means that whatever editor you are using, or just cargo build, you will get these html errors as normal for free.
You don't have to configure IDE sub-languages or whatever.

There's no mapping of source files to built files and the loss of context you get with preprocessors.

As I've said before, it's Rust all the way down.

We build a Yew app, like react, with a tree of components, where state flows from trunk to leaf:

---

[yew::html::Component](https://docs.rs/yew/latest/yew/prelude/trait.Component.html)

```rust[]
trait Component {
    type Message: 'static;
    type Properties: Properties;
    
    fn create(&Context<Self>) -> Self;
    fn view(self, &Context<Self>) -> Html;
    fn update(self, &Context<Self>, Message) -> bool
    fn changed(self, &Context<Self>) -> bool
    fn rendered(self, &Context<Self>, bool) 
    fn destroy(self, &Context<Self>) 
}
```

(some code omitted)

notes:

Here is Yew's Component trait, which you can think of as the interface of our components.

Only the first two methods, `create()` and `view()`, are required.

- `create` is called when the component is created.
- `view` defines the components's visual layout in html
- `update` is called when a new message is sent to the component.
- `changed` is called when properties passed to the component change
- `rendered` is called after each time a Component is rendered but before the browser updates the page, and,
- `destroy` is called right before a Component is unmounted.

Here's how they come together for the classic demo of a click counter:

---

## Model

```rust
use yew::prelude::*;

enum Msg {
    AddOne,
}
struct Model {
    value: i64,
}
// block continues
```

notes:

Firstly, as ever, we define the valid states of our system.

I'll pause here to point out that from this core model you, the viewer, could probably infer the functionality of my app, couldn't you?

The compiler certainly could.

Let's move on:

The only message we expect to send to our component is to increment the counter.

We don't send this as a string or a number or some anonymous object - this is Rust, we do things properly.
The Msg enum has only one variant, `AddOne`.
If there were other valid messages that could be sent to this component, we could add them here, and even define payloads of those messages such as numbers or strings or any type.

The compiler would then refuse to compile any of our code where we sent the wrong message or forgot to handle a message variant.

Just imagine what that kind of certainty would be like in a 100k line codebase. That'd be worth waiting a few seconds to compile wouldn't it.

I've built large react apps.

I certainly think so.

---

## Model

```rust
impl Component for Model {
    type Message = Msg;
    type Properties = ();
// block continues
```

notes:

The Model struct holds our state, which for this example is just the counter value, an extremely large integer.

We then build our component, implementing the Component trait for our Model, with the Message and Properties that all Components are required to have.

---

## Model.create()

```rust
fn create(_ctx: &Context<Self>) -> Self {
	Self {value: 0}
}
// block continues
```

notes:

Then we define the create method, where we set the initial state for our component.

Remember that `Self`, here, is the Model struct where we keep our component's state.

---

## Model.update()

```rust
fn update(&mut self, _ctx: &Context<Self>,
	msg: Self::Message) -> bool {
		match msg {
			Msg::AddOne => {
				self.value += 1;
				true
			}
		}
}
// block continues
```

notes:

Here is the the update method, where we receive our message, and add one to our counter, mutating our component's state.

---

## Model.view()

```rust
fn view(&self, ctx: &Context<Self>) -> Html {
	let link = ctx.link();
	html! {
	  <div>
		<button onclick={link.callback(|_| Msg::AddOne)}>{
		  "+1"
		}
		</button>
		<p>{ self.value }</p>
	  </div>
	}
}
```

notes:

And all of this is rendered by the view method in html, using the brilliant html macro again.

We have created a button and wired the onclick event to send our valid message to our update method.

---

```html
--> src/main.rs:78:43
 |
 | enum Msg {
 | -------- variant `AddTwo` not found for this enum
 
 ...
 
 | <button onclick={link.callback(|_| Msg::AddTwo)}>{
 |                                         ^^^^^^
   variant or associated item not found in `main::Msg`
```

notes:

Note that even inside the html!() macro, the rust compiler keeps us safe - our whole app is strongly typed and errors are beautiful.


---

# Webassembly Challenges

notes:

Now, building apps with Webassembly is not without challenges.

You will often read about three in particular:

---

## Native Graphics?

Yes, with WebGL

![[WebGL_Logo.png]]

notes:

As you can probably guess, WebGL is the missing piece of the puzzle here.
Many Rust ui libraries work inside webassembly, allowing custom native interfaces running at 60fps.

Subscribe for part two where I demo this.

---

## Multithreading?

Yes, with Web Workers

```rust[]
pub use wasm_bindgen_rayon::init_thread_pool;

#[wasm_bindgen]
pub fn sum_of_squares(numbers: &[i32]) -> i32 {
  numbers
  .par_iter() // <- computation in parallel from here
  .map(|x| x * x)
  .sum()
}
```

notes:
Webassembly has access to web workers, which you may know from javascript.

This is difficult to integrate into most languages' runtimes, as the end developer, ie you and I, don't have access to the runtime, it's written in C/C++. You either use O/S threads, not available in the browser, or your languages green threads or goroutines or whatever, or you're out of luck and have to wait for the language designers to implement web workers.

Rust doesn't come with an async runtime, though we mostly use the crate Tokio for this.

And here we see the benefit of not building in an async runtime into the standard library.

this means that excellent packages like my favourite here, Rayon, are backend agnostic, which means it's trivial to swap it out for one that supports webworkers, like we are doing here.

---

## Garbage Collection?

No (it's a feature!)

notes:

Wasm not having a built-in garbage collector really harms porting efforts for other languages that expect one, meaning that the whole runtime of, say, python must be ported into webassembly, in order to run it in the browser.

But with Rust, there's no gc, meaning it works trivially inside webassembly, just as it does on every o/s, containers, and on bare metal.

If you know rust, congratulations, you're now a frontend web developer, and you can easily write safe and fast web apps.

---

%%

```rust
}
```

%%

---

![[rust-logo.png|300]]

# Safe & Fast Web Development

notes:

# OUTTRO


In the top video, I used Rust to make a fun retro computer visualisation for my hopepunk podcast, Lost Terminal.

If urban fantasy is more your bag, click the bottom video to listen to a strange and beautiful podcast I produce called Modem Prometheus.

Transcripts and compile-checked markdown sourcecode are available on github, links in the description, and corrections are in the pinned ERRATA comment.

Thank you so much for watching, talk to you on Discord.

```rust
  println!("That's all folks!");
} 
```
