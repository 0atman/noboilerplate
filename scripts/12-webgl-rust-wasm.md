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

# Native Rust webapps

notes:

Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

I think the main advantage is building real apps inside the web browser! It would be great to focus on libraries built on webgl, such as EGUI. I think the big benefit of WASM + WebGL is that you can run fast code with a completely custom UI. This is game changing for applications like Quadratic where our main interface is a spreadsheet. We don't have to use div or table predefined elements. We can build our own from scratch, that do exactly what we want with high performance! We can also store all the data in our spreadsheet and define the exact memory layout using something like Apache [Arrow](https://streaklinks.com/BO6n8e-VlQ3KmGfAPgGckNrF/https%3A%2F%2Fgithub.com%2Fjorgecarleitao%2Farrow2). This changes what is possible in the browser!  
  
  
Figma is a great example. There is no HTML element for vector editing!

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