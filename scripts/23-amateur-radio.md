<style>
:root {--r-code-font: "FiraCode Nerd Font";}
.reveal .hljs {min-height: 50%;}
</style>
%%

f7f7f7 background slide colour

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
```

# Setup

```rust
fn main() {
	println!("Rust talk");

```

%%

![[rust-logo.png]]

# Off-Grid Radio

notes:
%%
- Tell them what you're going to tell them
- Tell them
- Tell them what you told them
%%
Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

---

## Public Domain Videos

[github.com/0atman/noboilerplate/](https://github.com/0atman/noboilerplate/)

notes:
Everything you see in this video from the script to the images are part of a markdown document available on github under a public domain license.

---

# 📻
# 2E1OAT
(that's me!)

---

![[hackers-blades.png]]

---

![[hoff-car.png]]

---

# CONTENT HERE

- [ ] outline amateur radio with core concept
- [ ] dream of the 90s
- [ ] amateur radio license is like a driving license, you don't need it, but your options are much better if you have it
- [ ] ISS
- [ ] moon bounce 2.5s
- [ ] POTA
- [ ] SOTA
- [ ] morse code - no longer a requirement for most low-liscences but gives 100% power efficiency and cuts through noise very well
- [ ] DX
- [ ] frequencies across the bands
- [ ] where to start? VHF
	- [ ] baofeng
- [ ] disaster recovery
- [ ] radio repeaters in your city
	- [ ] USA
	- [ ] UK
	- [ ] India
	- [ ] Canada
	- [ ] Germany
- [ ] DIGITAL
	- [ ] no crypto
	- [ ] low power modes
	- [ ] TCP fine
	- [ ] UDP fine
	- [ ] lora
	- [ ] 

---



![[paris-skyline.jpg]]


notes:
There is an invisible force in every city

---


![[london-skyline.png]]



---
![[berlin-skyline.png]]

---

![[car-radios.png]]

---

![[loft-antenna.png]]


---

![[building-antennas-on-roof.png]]

---

![[baofeng.png]]

---


---


---








# USA

![[usa-repeaters.png]]

---

# Germany

![[germany-repeaters.png]]

---

# (rest of Europe)

![[eu-repeaters.png]]

---

# UK

![[uk-repeaters.png]]

---

# India
![[india-repeaters.png]]

---

# Canada

![[canada-repeaters.png]]

---


<!-- slide bg="rgb(37, 34, 43)" -->

![[quadratic-website.png]]

notes:

This time we want to focus on user signups, so walking through the high-level features again and show a simple example.

---

<!-- slide bg="rgb(37, 34, 43)" -->
![[quadratic-formula-python-selection.png|700]]

notes:

Quadratic combines the functional data visualisation of a spreadsheet with power programming languages, starting with Python, the standard in data science.

---

<!-- slide bg="rgb(37, 34, 43)" -->
![[quadratic-micropip-demo.png]]

<!-- slide bg="rgb(37, 34, 43)" -->
notes:
Using Pyodide inside webassembly, any pure python dependency can be installed, and a few non-pure packages like nummpy and pandas that have been ported from C++.

---

<!-- slide bg="rgb(37, 34, 43)" -->

![[quadratic-api-demo.png]]

notes:
Because all of python is running locally inside webassembly, complex work, such as here pulling data from an api, is possible.

---

<!-- slide bg="rgb(37, 34, 43)" -->

![[quadratic-fps.png]]


notes:


This is all running at 60fps using webgl, with smooth scrolling and pinch to zoom.
All inside your browser.


---


<!-- slide bg="rgb(37, 34, 43)" -->

![[quadratic-fullscreen-python.png]]

-   Show one of our example files with data and python code
---


<!-- slide bg="rgb(37, 34, 43)" -->

![[quadratic-section-zoom-in-out.gif]]

-   Infinite Canvas Spreadsheet

---


<!-- slide bg="rgb(37, 34, 43)" -->


![[quadratic-dataframe.png|700]]

-   Python spreadsheet

---


<!-- slide bg="rgb(37, 34, 43)" -->

![[quadratic-github.png|800]]

-   Open source, free

---

<!-- slide bg="rgb(37, 34, 43)" -->

![[Quadratic Logo.png]]

The infinite canvas spreadsheet with code

https://QuadraticHQ.com

notes:

-   Signup today!
-   Link in description and on screen. QuadraticHQ.com

---



(TODO: animated demo)

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



---
![[rust-logo.png]]

# Subtitle

notes:

# OUTRO

If you would like to support my channel, get early ad-free and tracking-free videos and vip discord access head to patreon.com/noboilerplate.

If you're interested in transhumanism and hopepunk stories, please check out my sci-fi podcast, Lost Terminal.

Or if urban fantasy is more your bag, do listen to a strange and beautiful podcast I produce called Modem Prometheus.

Transcripts and compile-checked markdown sourcecode are available on github, links in the description, and corrections are in the pinned ERRATA comment.

Thank you so much for watching, talk to you on Discord.

```rust
  println!("That's all folks!");
} 
```
