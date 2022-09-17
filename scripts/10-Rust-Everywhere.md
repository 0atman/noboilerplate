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
yew = "0.19"
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
	println!("Rust Everywhere");

```

%%

![[rust-logo.png]]

# RUST Everywhere

### The Real Web 3.0

notes:

Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

Today I'm going to talk about building native apps you can deploy anywhere with no installation by using Rust and web assembly.

---

|           |                                        |
| --------- | -------------------------------------- |
| Docker    | <https://hub.docker.com/_/rust>          |
| Game      | <https://bevyengine.org>               |
| backend   | <https://rocket.rs>                    |
| AWS Cloud | <https://aws.amazon.com/sdk-for-rust/> |
| Windows   | <https://crates.io/crates/windows>     |
| Webasm    | <https://www.rust-lang.org/what/wasm>  |

notes:

Rust runs naively everywhere,
- on servers and in containers
- game engines running at native C speed
- backend web frameworks, allowing safe, fast web development
- In the AWS cloud with first-class support from amazon and is the cheapest language to run on aws lambda
- on windows with first class support from Microsoft
- And naively in the browser with Webasm

---

webasm
- no native graphics?
	- what about opengl
- not multi threaded?

```rust[]
pub use wasm_bindgen_rayon::init_thread_pool;

#[wasm_bindgen]
pub fn sum_of_squares(numbers: &[i32]) -> i32 {
  numbers
  .par_iter()
  .map(|x| x * x)
  .sum()
}
```

- no garbage collection
-

---

## Predictable Performance

## Small Code Size

## Modern Ergonomics

</br>

<https://www.rust-lang.org/what/wasm>

notes:

Rust is an excellent language to write webasm in:

No unpredictable garbage collection pauses. No JIT compiler performance cliffs. Just low-level control coupled with high-level ergonomics.

Small code size means faster page loads. Rust-generated wasm doesn’t include extra bloat, like a garbage collector. Advanced optimisations and tree shaking remove dead code.

And Rust has a lively ecosystem of libraries to help you hit the ground running. Expressive, zero-cost abstractions, and a welcoming community to help you learn.

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

wavacity
A port of the popular audio editor, Audacity, to webassembly.
The author even ported the C++ interface library, wxWidgets, along with the app.

---

![[wavacity-mobile.png]]

Literally my phone

notes:

When you write webassembly you get the mobile app for free.

---

![[doom-wasm.png|700]]

Literally DOOM

notes:

And with webgl being standard in all modern browsers, you can run anything, including doom.

---

![[doom-wasm-mobile.png]]

Literally DOOM on my phone

notes:

which of course runs on your phone too.

---

- C/C++
- 🦀 Rust
- AssemblyScript
- C#
- F#
- Go
- Kotlin
- Swift
- D
- Pascal
- Zig

notes:

But there is a problem with webassembly
- Here are the languages from webassembly.org that you write webasm in. Notice a problem?
	- not python
	- not ruby
	- not java
	- not even javascript
- This has hampered adoption, these webasm languages are so drastically different from the high-level languages of the web.
- but I spot a language in there that I DO like, and Rust was one of the earliest adopters of webassembly, more than 5 years ago.

Rust's take on webassembly after I talk about this video's sponsor, Quadratic.

---

![[Quadratic Logo.png]]

notes:

- Quadratic are building an Open Source spreadsheet for engineers and data scientists, built in Rust, Webassembly and WebGL
- This might be the coolest spreadsheet I've ever used, and I've used emacs.
- You can choose your formula language,
  - Either simple excel style statements - or - SQL and Python, both standards in the field of Data Science
- Because all data is evaluated in Webassembly, Quadratic is fast
- The UI is in WebGL, with hardware acceleration in all modern browsers, allowing 60fps scrolling, complex graphics, and smooth pinch-to-zoom

This is a really exciting project that I'm delighted to say are hiring.

---

Quadratic are looking for:
- Rust developers
- People with WebGL experience, even if that's only with JavaScript
- People with Apache Arrow experience for processing Quadratic's high-performance data sets, and,
- Senior engineers used to working at the pace of a startup.

---

Check out and star the project on github at github.com/quadrichq

and view their open jobs at careers.quadratic.to

My thanks to Quadratic for their support of this channel.

Let's see what Rust webassembly looks like.

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

Here's the lowest level webassembly code you can write in Rust.
It's so low level it's grabbing the window, document and then generating nodes and appending them to the DOM manually.

do you Recognise this kind of code?

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

oh, it looks like javascript!

This is of course not how we'd write browser code today, in JS or in Rust, I'm just showing you the fundamentals.

[MDN Document/createElement](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElement)

just as you don't directly manipulate the dom with js, we won't do so with vanilla rust

Today we're going to look at the most popular frontend webasm framework in Rust, which is called Yew.

---

# Yew.rs

![[yew.png]]

notes:

Yew is a framework for creating multi-threaded front-end web apps using WebAssembly.

It is a component-based framework, like React and Elm.

Though webassembly is slower than javascript at the moment, yew is faster than react.

I suspect that as webassembly is optimised in browsers more and more, we will see frameworks like Yew take the lead in performance.

But it's already faster than react, so I say ship it!

Yew is designed with webassembly's bidirectional javascript support in mind, supporting npm libraries.

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

This is the simplest component, but look at that html!() macro! Yes, that's html inside your Rust.

This isn't achieved by some kind of preprocessor like reqact requires with JSX, this is native Rust.

At compile-time, the html!() macro re-writes that html into yew function calls that build the html and interpolate the props for us.

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

There's no mapping of source files to built files and the loss of context you get with preprocessors.

As I've said before, it's Rust all the way down.

You build a Yew app, like react, with a tree of components:

---

[yew::html::Component](https://docs.rs/yew/latest/yew/prelude/trait.Component.html)

```rust[]
trait Component {
    type Message: 'static;
    type Properties: Properties;
    
    fn create(&Context<Self>) -> Self;
    fn update(self, &Context<Self>, Message) -> bool
    fn changed(self, &Context<Self>) -> bool
    fn view(self, &Context<Self>) -> Html;
    fn rendered(self, &Context<Self>, bool) 
    fn destroy(self, &Context<Self>) 
}
```

(some code omitted)

notes:

Here is the full Component trait, which you can think of as the interface we will attach to our components.

Only the first two methods, `create()` and `view()`, are required.

- `create` is called when the component is created.
- `update` is called when a new message is sent to the component via it's scope.
- `changed` is called when properties passed to the component change
- `view` defines the components's visual layout in html
- `rendered` is called after each time a Component is rendered but before the browser updates the page.
- `destroy` is called right before a Component is unmounted.

Here's how they come together for the classic demo of a click counter:

---

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

The only message we expect to send to our component is to increment the counter.

We don't send this as a string or a number or some anonymous object - this is Rust, we do things properly.
The Msg enum has only one variant, and that is `AddOne`.
If there were other valid messages that could be passed in, we could add them here, and even define payloads of those messages such as numbers or strings.
The compiler would then refuse to compile any of our code where we sent the wrong message.

Just imagine what that kind of certainty would be like in a 100k line codebase. That'd be worth waiting a few seconds to compile wouldn't it.

I think so.

---

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

```rust
fn create(_ctx: &Context<Self>) -> Self {
	Self {value: 0}
}
// block continues
```

notes:

Then define the create method, where we set the initial state for our component.

Remember that Self, here, is the Model struct where we keep our component's state.

---

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

Here is the the update method, where we receive our message, and add one to our counter, mutating our state.

---

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

%%

```rust
}
```

%%

---

![[rust-logo.png]]

# GOOD NEWS EVERYBODY

notes:

# OUTTRO

If you'd like to see what you can write in rust, click the top video: I used it to make a fun retro computer visualisation for my hopepunk podcast, Lost Terminal.

Or if urban fantasy is more your bag, click the bottom video to listen to a strange and beautiful podcast I produce called Modem Prometheus.

If you would like to support my work, head to patreon.com/noboilerplate.
Transcripts and compile-checked markdown sourcecode are available on github, links in the description, and corrections are in the pinned ERRATA comment.

Thank you so much for watching, talk to you on Discord.

```rust
  println!("That's all folks!");
} 
```
