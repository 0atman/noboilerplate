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

# Lint tweaks
```rust
#![allow(dead_code)]
#![allow(unused_variables)]
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

# Web-Native Rust apps

notes:

Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

A revolution happened in `2017`, the wasm revolution.
But it has been slow to be adopted by popular programming languages.

Rust adopted webassembly instantly, and thanks to good language design, good community attitudes, and some good luck, is the best language to write wasm apps today.

Let's take a step back.

---

# Why is the web so popular?

notes:

Web programming used to be a frustrating place, I was there, not so long ago.

Even in the early days of html embeds and bad javascript, it was obvious business had changed overnight.

It's easy to see in hindsight, but at the time, on the ground, the technologies we fought with were were so BAD.

Javascript is no longer is a bad programming language. It's fine, but at the TIME it was terrible!
The web was full of frames and errors and plugins.

The interface, which for most sites was times new roman, black on white with blue links, looked terrible, and was different on every computer.

BEST VIEWED ON IE4, sites would tell you.

Remember that?

---

# The web is about distribution

The web wasn't a hit because it looked pretty.
Nor was it easy to build apps for.
Or fast.
Or efficient on the client or server.

What made it SO good, and still does, despite all these years past, is the ease of distrobution.

---

`my cat blog image`

Anyone with a text editor can hand-write a text file, host it somewhere, often for free, and then BOOM your words are accessible worldwide.

---

`please update to continue using this app image`

And for application developers, there's no installation on your customer's computers, it's the thin client dream.

No patching, no difficulty in supporting old version of your app, which still plagues mobile app developers

---

We put up with all the annoyances of the web, happily, to get this incredible distribution advantage.

No mailing customers disks, or CDs, or even giving google or apple a portion of your app store revenue.
Just direct access between business and consumers.

The web heralded the end of many of the old gatekeepers.

---

But it's not perfect.

I am a web developer, and have been for 15 years, and I love the UI model inside the browser, which is called the DOM, the Document Object Model.

It's flexible, dynamic, and can do nearly anything.

Nearly anything.

What if you want to write you own UI, unconstrained by the browser's idea of how you should build it?

For that you need WebGL.

---


# Real apps inside web browser

notes:

Webgl allows you to build native apps using regular UI frameworks within a browser.

---

![[wavacity.png]]

notes:
Here's an example of an app built with a native UI framework, specifically wxWidgets, But all the usual suspects have been ported:
- GTK
- QT
- `etc`



---


## Fastest web code is Wasm
## Fastest web ui is WebGL
## Fastest storage is on the user's computer

notes:
Let's get some simple facts out of the way.

Webassembly is MUCH faster than javascript for pure computation. Current browser implementations bottleneck at DOM interactions.
But that's not a problem if you use WebGL, with direct gpu access, via opengl. 
And when we stop thinking about 'web apps' as 'web pages' and start thinking of them as real apps user, operating on real data, it's natural to store data right where the user is, for low-latency processing. You can sync too, if the user wants that.

---

## Unconstrained by the speed of javascript
## Unconstrained by DOM quirks
## Unconstrained by remote storage 

notes:


There are often new, faster javascript frameworks coming out offering more and more dom manipulations per second. Svelt compared to React, for instance.
They're useful for a DOM-constrained app, but we have access to native UI speeds, no latency, and 60fps with webgl.

A great example is today's sponsor, Quadratic.

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
- [ ] check this with David
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

# figma example

Enough examples, what can we write in RUST?

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
Yes, there's GTK and others, but they link to the C libraries, and you'll have an interesting time using the webgl ports.
As usual, I recommend keeping things pure Rust.

To build a whole egui app, use eframe the EGUI Framework.

We start, as ever, with defining the valid states of our system.

Then impliment the Default trait for our app struct.

The default trait, which defines a single method, also called `default()`,  that returns an instance of our struct. 
You will find this default method in many places in the standard library and 3rd party crates, where it makes sense to create a default configureation of a structure.
It's like the new method pattern, in that regard.

---


![[Egui-hello.png]]

notes:
Here's the UI we are going to build, by the way.
We've got a heading, a label and text box, a slider, a button and another label.

Here's how we implement that:

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

notes:
We implement EGUI's App trait, and define a single `update()` method.

And here's the code that builds the previous screenshot.
you can see the heading, label and text box, slider, button and the other label.

Persistance is through updating Self, which is passed in as the first paramater of the update method.

Any data you can fit in a struct could be persisted in this way, which is to say, anything you like.

---

## EGUI widgets

```rust
ui.label("This is a label");
ui.hyperlink("https://github.com/emilk/egui");
ui.text_edit_singleline(&mut my_string);
ui.button("Click me")
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

Here are some of the widgets available to you in EGUI, and on their website, which of course is written in webgl and wasm, they demo a markdown editor and a curl client, too. 

---

```rust
if ui.button("click me").clicked() { take_action() }
```

This button is re-rendered every frame

notes:
EGUI is an Immediate Mode framework.
This makes it simple to reason with, and simple to integrate with game or other opengl frameworks that expect to re-render every frame.

---

# brackets-terminal

---

# Bevy engine

---

---

```rust
fn bevy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Rectangle
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 100.0)),
            ..default()
        },
        ..default()
    });

    // Circle
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(-100., 0., 0.)),
        ..default()
    });

    // Hexagon
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
        material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
        transform: Transform::from_translation(Vec3::new(100., 0., 0.)),
        ..default()
    });
}
```

---


---




> "Think about the ideal way to write a web app.
  Write the code to make it happen.”

&mdash; Aaron Schwartz (on web.py)

notes:

To paraphrase the late great Aaron Schwartz. Think about the ideal way to write a web app. Write the code to make it happen.

Any app, game, or experience is yours to build with Rust and WebGL. 


---

# The Future of Wasm

notes:
A new standard based on Vulkan, Metal, and Direct3d is being developed at the moment called WebGPU.

It remains to be seen if it will replace webgl. If it does, and you're writing a low-level webgl app, then yes, rewriting will be needed.
However, that's now how we write our apps and games mostly.
If you use a UI library, like the ones I demoed earlier, then you are insulated somewhat against migration pain, you simply wait for the UI library to be ported over to the new underlying system.

Until then, WebGL is ready NOW. 


%%
```rust
}} // who left these here?
```
%%

---


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