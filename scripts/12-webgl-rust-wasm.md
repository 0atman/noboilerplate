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

Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

A quiet revolution happened in `2017`, the revolution of wasm.
But it has been slow to be adopted by popular programming languages.

Rust adopted webassembly instantly, and thanks to good language design, good community attitudes, and some good luck, is the best language to write wasm apps today.

Let's take a step back.

---

# Why is the Web so Popular?

notes:

Web programming used to be a frustrating place, I was there, not so long ago.

But Even in the early days of html embeds and bad javascript, it was obvious business had changed overnight.

It's easy to see in hindsight, but at the time, on the ground, the technologies were so BAD.

Javascript no longer is a bad programming language. It's fine, but at the TIME it was terrible!
The web was full of frames and errors and plugins.

The interface, which for most sites was times new roman, black on white with blue links, looked terrible too, and was different on every computer.

BEST VIEWED ON IE4, sites would tell you.

Remember that?

---

The web wasn't a hit because it looked pretty.
Nor was it easy to build apps for.
Or fast.
Or efficient on the client or server.

What made it SO good, and still does, despite all these years past, is the ease of distribution.

---

`my cat blog image`

Anyone with a text editor can hand-write a text file, host it somewhere, often for free, and then BOOM your words are accessible worldwide.

That's nothing short of incredible.

---

`please update to continue using this app image`

And for application developers, there's no installation on your customer's computers, it's the thin client dream of the 70s.

No patching, no difficulty in supporting old versions of your app, which still plagues mobile app developers

---

We put up with all the annoyances of the web, happily, to get this incredible distribution advantage.

No mailing customers disks, or CDs, or even giving google or apple a portion of your app store revenue.
Just direct access between business and consumers.

The web heralded the end of many gatekeepers.

---

But there's a problem.

It STILL kind of sucks.

I am a web developer, and have been for 15 years, and I love the UI model inside the browser, which is called the DOM, the Document Object Model.

It's flexible, dynamic, and can do nearly anything.

Nearly anything.

What if you want to write you own UI, unconstrained by the browser's idea of how you should build it?

For that you need WebGL.

---

# Real Apps Inside Web Browser

---

## Fastest Web Code is Wasm

## Fastest Web Ui is Webgl

## Fastest Storage is on the User's Computer

---

## You Are Unconstrained by the Speed of Javascript

## You Are Unconstrained by Learning the Subtleties of the Browser

## You Are Unconstrained by Posting User Date Across the Internet

---

---

# Figma Example

---

# Egui

---

# Brackets-terminal

---

# Bevy Engine

---

> "Think about the ideal way to write a web app.
  Write the code to make it happen.”

&mdash; Aaron Schwartz (on web.py)

notes:

To paraphrase the late great Aaron Schwartz. Think about the ideal way to write a web app. Write the code to make it happen.

Any app, game, or experience is yours to build with Rust and WebGL.

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

---

# EGUI

```toml
eframe="0.19.0"
```

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

To build a whole egui app, use eframe.

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
 });
```

%%

```rust
}} // who left these here?
```

%%

---

![[Egui-hello.png]]

---

![[rust-logo.png]]

# Subtitle

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
