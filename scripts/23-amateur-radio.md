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

# Off-Grid communication

notes:
%%
- Tell them what you're going to tell them
- Tell them
- Tell them what you told them
%%
Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

Today I'm going to talk about a new world I've discovered, a world that predated the internet and that will be here after the internet as we know it is gone.

That world is called Amateur Radio.

---

## Public Domain Videos

[github.com/0atman/noboilerplate/](https://github.com/0atman/noboilerplate/)

notes:
Everything you see in this video from the script to the images are part of a markdown document available on github under a public domain license.

---

![[paris-skyline.jpg]]


notes:
There is an invisible society in every city in the world.
Hiding in plain sight.

---
![[berlin-skyline.png]]

notes:
You won't find them online, they don't have an ip address, not on the public internet anyway.

---


![[london-skyline.png]]

notes:

You can some times catch glimpses of this society on rooftops.
If you know what to look for, there is evidence on every street.

---


![[car-radios.png]]

notes:
Or on cars.

---

![[loft-antenna.png]]

notes:

Sometimes the equipment is hidden, either from prying eyes or protected from nature and lightning, but it's there.

---

![[building-antennas-on-roof.png]]

In every city in the world, there are amatuer radio operators.
Millions of us.

---

![[baofeng.png|700]]

notes:

The price of admission is very low.
$25 on amazon, you can have a simple radio delivered tomorrow.

---

![[cb-radio-car.png]]

notes:

If you've heard of CB radio, and you're thinking oh this sounds similar.
You're getting there, but Amateur radio is so much more.

---

# VHF 144-146MHz
# UHF 430-440MHz

notes:

We'll start with the two most common amateur radio bands, VHF and UHF.

They are simple to start with, only requiring inexpensive radios.
The signals don't tend to go over the horizon, but 

With just these two bands, you can do SO MUCH FUN STUFF.


---

![[diana-eng-make-magazine.png|600]]



[Listening to Satellites with a Homemade Yagi Antenna](https://makezine.com/projects/homemade-yagi-antenna/)
&mdash; Diana Eng, MAKE Magazine

notes:

On this band you can communicate with amateur radio satellites.

---

![[chris-hadfield-iss-radio.png]]

notes:

AND EVEN WITH THE INTERNATIONAL SPACE STATION
This is Chris Hadfield, superstar astronaut, at the ISS's amateur radio station.

https://www.amsat.org/

---

![[earth-moon-earth-antenna-with-moon.png]]
- [ ] moon bounce 2.5s

---

<split even>

![[pota-park-bench-t-witherspoon.png|400]]

&nbsp;
&nbsp;
&nbsp;
&nbsp;
&nbsp;

![[pota-field-report-t-witherspoon.png|400]]

</split>

<br>


[POTA Field Report, Thomas Witherspoon](https://qrper.com/2021/02/pota-field-report-pairing-the-icom-ic-705-with-the-elecraft-ax1-pocket-antenna/)

notes:

- [ ] POTA
---

![[pota-mountaineering-radio.jpg]]

notes:

- [ ] SOTA

---

# Repeaters

notes:
Cell phones don't have very long range.
In fact, you need towers in every neibourhood to support them.
It's a clever idea, and one that Amateur Radio operators were experimenting with even before the first commercial cell phone in 1983.

There are repeaters EVERYWHERE, no matter where you live.

In decending order of my youtube audience stats, here are the repeater maps for -

---


# USA

![[usa-repeaters.png]]

notes:

The United States.

---

# Germany

![[germany-repeaters.png]]

notes:

Germany.

---

# (rest of Europe)

![[eu-repeaters.png]]

notes:
Oh and the rest of Europe obviously.

---

# UK

![[uk-repeaters.png]]

notes:

My home country of the UK has a few.

---

# India
![[india-repeaters.png]]

India.

---

# Canada

![[canada-repeaters.png]]

And Canada.

---


![[global-repeaters.png]]

notes:

You might note that as usual, this kind of map approximates a population map.

Wherever there are towns and cities there are repeaters!

If you want access to this world, it's cheap and easy, but like driving a car, there's a test and a license.
And just like driving a car it augments any outdoor activity you might want to do.

I've not even talked about the lower frequencies, where you can get worldwide propogation without using repeaters!

More on that after I tell you about returning sponsor, Quadratic.

---


<!-- slide bg="rgb(37, 34, 43)" -->

![[quadratic-website.png]]

notes:

Quadratic are building an Open Source spreadsheet for engineers and data scientists, built in Rust, Webassembly and WebGL

---

<!-- slide bg="rgb(37, 34, 43)" -->
![[quadratic-formula-python-selection.png|700]]

notes:

Quadratic combines the functional data visualisation of a spreadsheet with the power of full programming languages, starting with Python

---


<!-- slide bg="rgb(37, 34, 43)" -->


![[quadratic-dataframe.png|700]]

notes:
Standard Python data science libraries are built-in.

---

<!-- slide bg="rgb(37, 34, 43)" -->
![[quadratic-micropip-demo.png|700]]

<!-- slide bg="rgb(37, 34, 43)" -->
notes:
In fact, because quadratic are using Pyodide inside webassembly, any pure python dependency can be installed, like this example of the faker library.

---

<!-- slide bg="rgb(37, 34, 43)" -->

![[quadratic-api-demo.png|700]]

notes:
Because all of python is running locally inside webassembly, complex work, such as here pulling data from an api, is possible.

---

<!-- slide bg="rgb(37, 34, 43)" -->

![[quadratic-fps.png|700]]


notes:

This is all running at 60fps using webgl, with smooth scrolling and pinch to zoom.
All inside your browser.


---


<!-- slide bg="rgb(37, 34, 43)" -->

![[quadratic-section-zoom-in-out.gif]]

notes:
Quadratic built their infinite canvas on webgl, allowing for smooth scrolling and pinch to zoom.

---


<!-- slide bg="rgb(37, 34, 43)" -->

![[quadratic-github.png|800]]

https://github.com/quadratichq/quadratic

notes:

Quadratic is open source and free to use.

---

<!-- slide bg="rgb(37, 34, 43)" -->

![[Quadratic Logo.png]]

The infinite canvas spreadsheet with code

https://QuadraticHQ.com

notes:

-   Signup today!
-   Head to QuadraticHQ.com to try it out.

My thank to quadratic for their support of this channel.

---

![[kx3-morse-headset-desk.png]]

notes:

- [ ] morse code - no longer a requirement for most low-liscences but gives 100% power efficiency and cuts through noise very well



---

- [ ] DX


- [ ] frequencies across the bands
- [ ] Compare to wifi and bluetooth power and range
- [ ] where to start? VHF
	- [ ] baofeng
- [ ] disaster recovery

# 📻
# 2E1OAT
(that's me!)

---

![[hackers-blades.png]]

---

---

# CONTENT HERE

```
〉why would normal people get excited for amateur radio in a world where cell phones are ubiquitous
1. Emergency Communication: Amateur radio operators can communicate in times of natural disasters, emergencies, and other situations where traditional communication methods may fail. During such instances, amateur radio offers an alternative communication method and can potentially save lives.

2. Hobby: Amateur radio operation can be a fun and engaging hobby for people, especially those interested in electronics and communication technologies. Operators can contact people from all over the world, participate in contests, and explore different communication modes.

3. Local Community: Amateur radio clubs can bring together people with shared interests, and foster relationships among local communities. This can lead to social interactions, friendships and a sense of community that is not possible with cell phone use.

4. Exploration and Innovation: Amateur radio operators can experiment and come up with new ways of communication, modes of transmission, and receive certification for their achievements.

5. Low Cost: While cell phones require paid subscriptions, amateur radio only requires a very low amount of money for equipment purchase, and no ongoing fees.

In conclusion, while cell phones have become ubiquitous and offer instant communication, amateur radio provides a distinct offering for those interested in communication as a hobby, emergency communications, or seeking a unique social alternative to using cell phones.
```

- [ ] outline amateur radio with core concept
- [ ] dream of the 90s
- [ ] amateur radio license is like a driving license, you don't need it, but your options are much better if you have it

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

---


---


---





---



Let's go back to school.

---

![[rf-spectrum.png]]

|        |                  |
| ------ | ---------------- |
| 2.4GHz | WiFi & Bluetooth |
| 27MHz  | CB Radio         |
| ?      | Amateur Radio?   |


notes:
You might have seen something like this in a science textbook at some point.

Low frequency radio on the left, high frequency radio on the right.

The frequency you'll be most familiar with is 2.4GHz, which is what both WiFi and Bluetooth run on.

CB radio operates at about 27MHz, at the bottom of the VHF band.
CB is much better in an urban environment than WiFi, and can travel up to about 4km.

Where does Amateur radio operate? 

EVERYWHERE

---
![[tri-hex-moon-white-transparent.png|300]]

# Thank you
## [Patreon.com/NoBoilerplate](http://www.patreon.com/noboilerplate)
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
