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
```

# Lint Tweaks

```rust
#![allow(dead_code)]
#![allow(unused_variables)]
```

# Extern Crates

```rust

```

# Imports

```rust
use bevy::prelude::*;
use bracket_terminal::prelude::{
	GameState,
	BTerm,
	BError,
	BTermBuilder,
	main_loop
};
```

# Setup

```rust
fn main() {
	println!("Rust talk");

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );

```

%%

![[rust-logo.png|300]]

# Web-Native Rust Apps

notes:

- [x] put a pause after each slide

Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

This is my follow-up video on rust and wasm.
Do check out part one, if you haven't already.

A quiet revolution happened in `2017`, the revolution of wasm.
But it has been slow to be adopted by popular programming languages.

As I outlined in the first video, Rust adopted webassembly instantly, and thanks to good language design, good community attitudes, and some good luck, Rust is the best language to write wasm apps in today.

Let's take a step back.

---

# Why is the Web so Popular?

notes:

Web programming used to be a frustrating place, I was there, not so long ago.

But even in the early days of html embeds and bad javascript, it was obvious the world had changed overnight.

It's easy to see in hindsight, but at the time, on the ground, the technologies we used so BAD.

Javascript no longer is a bad programming language. It's aggressively fine, but at the TIME it was terrible!
The web was full of frames and errors and plugins.

---
![[msn-web-browser.png]]

notes:

The interface, which for most sites was black on white with blue links, looked terrible too, and was different on every computer.

BEST VIEWED ON IE5, sites would tell you.

Remember that?

---

# The Web is About Distribution

notes:

The web wasn't a hit because it looked pretty.
Nor was it easy to build apps for.
Or fast.
Or efficient on the client or server.

What made it SO good, and still does, despite all these years past, is the ease of distribution.

---

```html
<html>
	<body>  
	<h1>Welcome to my website</h1>  
	<p>
		Click
		<a href="cat.html">
			here
		</a>
		for photos of my cat!
	</p>  
	</body>  
</html>
```

notes:

Anyone with access to a computer can hand-write a text file, host it somewhere, often for free, and then BOOM your words are accessible worldwide.

That's nothing short of incredible.

---

![[update-please-mariokart.png|500]]
notes:

And for application developers, there's no installation on your customer's computers, it's the thin client dream of the 70s.

There's no patching and no difficulty in supporting old versions of your app, which still plagues mobile app developers to this day.

We put up with the annoyances of the web, happily, to get this incredible distribution advantage.

No mailing customers disks, or CDs, or even giving Google or Apple a large portion of your app store revenue.
Just direct access between apps and users.

The web heralded the end of many of the old gatekeepers.
But it's not perfect.

---

![[dom-inspector.png]]

notes:

I am a web developer, and have been for 15 years, and I love the UI model inside the browser, which is called the DOM, the Document Object Model.

It's flexible, dynamic, and can do nearly anything.

Nearly anything.

What if you want to write you own UI, unconstrained by the browser's idea of how you should build it?

For that you need WebGL.

---

![[wavacity.png]]

notes:
Here's an example of an app, webacity, built with a native UI framework and ported to webassembly.

The framework here is wxWidgets, but many others have been ported, including giants such as GTK and QT

---

## Fastest Web Code is Wasm

## Fastest Web Ui is WebGL

## Fastest Storage is on the User's Computer

notes:
Let's get some simple facts out of the way.

Webassembly is MUCH faster than javascript for pure computation. Current browser implementations bottleneck at DOM manipulations,
but that's not a problem if you use WebGL, with gpu access, via opengl.
And when we stop thinking about 'web apps' as 'web pages' and start thinking of them as real apps, operating on real data, it's natural to store the data right where the user is, for low-latency processing. You can sync too, if the user wants that.

---

# Unconstrained by

## Javascript

## DOM Quirks

## Remote Storage

notes:

Every week we hear about new, faster javascript frameworks coming out, offering more and more dom manipulations per second. Svelt compared to React, for instance.

They're useful for a DOM-constrained app, but we have access to native UI speeds, no latency, and 60fps with webgl.

A great example of how to do this right is returning sponsor, Quadratic.

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
- Rust developers for their rewrite (the existing code is in typescript)
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

Let's look at another example of a large-scale webgl app that hasn't paid me to talk about them.

---

# Figma

![[Figma-web.png]]

notes:
Figma is an collaborative interface design app, powered by WebGL and Wasm.

Though much of the UI is html, the core product, the interface designer runs on the GPU in Wasm.

The complexity of multiple nested UI projects would not be possible to emulate in the DOM, especially for the pixel-perfect mockups that the app is designed to build.

Enough examples, what can we write in RUST today?

---

![[bracket-terminal-topology-demo.png|800]]

```toml
bracket-terminal = "0.8.7"
```

notes:

let's start simple, with some webgl-rendered text, by the project Bracket.

---

```rust
struct State {}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.print(1, 1, "Hello Bracket World");
    }
}
fn main_bracket() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Hello Minimal Bracket World")
        .build()?;
    let gs: State = State {};
    main_loop(context, gs)
}
```

notes:

Bracket provides a virtual ASCII terminal, and a game loop. This frees you up from implementation difficulties, making it easy to write grid-based games and apps. It also provides assistance with keyboard and mouse input.

Let's see what this example looks like:

---

![[Pasted image 20221015205628.png]]

notes:

Marvellous! Now we're coding like it's 1989.

But Bracket is doing an deceptively large amount of work for you here, in a deployed size of just 500KB:
- It's WebGL native
- 60fps
- wasm deployment ready to go.
- And if you've ever tried to build a terminal-based interface or game, you'll know how obnoxious the terminal can be, with poorly-documented control codes and terminal emulators having their own quirks!
Not so with Bracket!
It looks like a terminal, but it isn't.

Next Let's build a modern interface

---

# EGUI

```toml
eframe="0.19.0"
```

%%

```rust
use eframe::egui::{
	Context,
	CentralPanel,
	Slider
};
use eframe::{
	Frame,
	App
};
```

%%

```rust
struct MyApp {
    name: String,
    age: u32,
}
impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}
```

notes:
EGUI is one of the biggest native UI toolkits built from the ground up for Rust.
Yes, you could use GTK and others, but they link to the C libraries, and you'll have an interesting time using the webgl ports.
As usual, I recommend keeping things pure Rust.

To build a whole egui app, use eframe the EGUI Framework.

We start, as ever, with defining the valid states of our system.

Then implement the Default trait for our app struct.

The default trait, which defines a single method, also called `default()`, returns an instance of our struct with pre-filled data.
You will find this default method in many places in the standard library and 3rd party crates, where it makes sense to create a default configuration of a structure.
It's like the new method pattern, in that regard.

---

![[Egui-hello.png]]

notes:
Here's the UI we are going to build, by the way.
We've got a heading, a label and text box, a slider, a button and another label.

Here's the code:

---

```rust
impl App for MyApp {
 fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
```

```rust
 CentralPanel::default().show(ctx, |ui| {
	ui.heading("My egui Application");
	ui.horizontal(|ui| {
		ui.label("Your name: ");
		ui.text_edit_singleline(&mut self.name);
	});
	ui.add(Slider::new(&mut self.age, 0..=120)
		.text("age"));
	if ui.button("Click each year").clicked() {
		self.age += 1;
	}
	ui.label(format!(
		"Hello '{}', age {}", self.name, self.age));
 });}} //ew
```

notes:
We implement EGUI's App trait, and define a single `update()` method.

you can see the heading, label and text box, slider, button and the other label, here.

Persistence is through updating Self, which is passed in as the first parameter of the update method.

Any data you can fit in a struct could be persisted in this way, which is to say, anything you like.

---

## EGUI Widgets

```rust[]
ui.label("This is a label");
ui.hyperlink("https://github.com/emilk/egui");
ui.text_edit_singleline(&mut my_string);
ui.button("Click me");
ui.add(egui::Slider::new(&mut my_f32, 0.0..=100.0));
ui.add(egui::DragValue::new(&mut my_f32));
ui.checkbox(&mut my_boolean, "Checkbox");
ui.radio_value(&mut my_enum, Enum::Third, "Third");
ui.separator();
ui.image(my_image, [640.0, 480.0]);
ui.collapsing("Click to see what is hidden!", |ui| {
    ui.label("Not much, as it turns out");
});
```

notes:

Here are some of the widgets available to you in EGUI, out of the box.

And on their website, which of course is written in webgl and wasm,

---

![[egui-kitchen-sink.png]]

https://www.egui.rs

notes:

they demo code editors, a curl client, and loads of other widgets!

---

```rust[]
if ui.button("click me").clicked() {
	take_action()
}
```

This button is re-rendered every frame

notes:
EGUI is an Immediate Mode framework.
This makes it simple to reason with, and simple to integrate with game or other opengl frameworks that expect to re-render every frame.

Game frameworks LIKE:

---

![[bevy-website.png]]

notes:

Bevy, which is is a featureful, 2d or 3d game engine built from the ground up in Rust.

There are many examples of 2d and 3d rendering on their website, all running smoothly in webgl.

---

```toml
bevy = "0.8.1"
```

```rust
fn bevy(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 100.0)),
            ..default()
        },
        ..default()});
}
```

notes:

Here's a simple scene in Bevy.

This shows how to render simple primitive shapes with a single colour.

In this case, a blue rectangle.

---

![[bevy-rectangle.png]]

(game development is my passion)

notes:

Here's how it looks on my machine.

Nothing groundbreaking, but it's hardware accelerated and webgl ready.

---

![[bevy-model-exploded.png]]

notes:

Bevy features:
- real-time 2D graphics for games and apps
- A modern and flexible 3D renderer
- lights, shadows, cameras, meshes, textures, materials,
- You can load audio files and play them on demand
- Asset changes are immediately reflected in running Bevy apps: You can hot-reload scenes, textures, and meshes
- And all this can be recompiled in 1-3s for instant feedback.

And bevy doesn't just support WebGL:

---

# The Future of Wasm: WebGPU

notes:
Looking forward, A new standard based on Vulkan, Metal, and Direct3d is being developed at the moment called WebGPU.

It remains to be seen if it will replace webgl. If it does, and you're writing a low-level webgl app, then yes, rewriting will be needed.

However, that's not how we build our apps and games.

If you use a UI library, like the ones I demoed today, then you are insulated somewhat against migration pain, you simply wait for the UI library to be ported over to the new underlying system.

Bevy already supports webgpu.

Until then, WebGL is ready in all browsers NOW.

---

> "Think about the ideal way to write a web app.
  Write the code to make it happen.”

&mdash; Aaron Schwartz (on web.py)

notes:

To paraphrase the late and great Aaron Schwartz. Think about the ideal way to write a web app. Write the code to make it happen.

Any app, game, or experience you can dream of is yours to make with Rust and WebGL.

What will you build?

---

![[rust-logo.png|300]]

# What Will YOU Build?

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
