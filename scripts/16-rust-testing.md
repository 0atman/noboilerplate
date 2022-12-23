%%
<style>
:root {--r-code-font: "FiraCode Nerd Font";}
</style>

# Cargo.toml 
```toml
[package]
name = "rust-testing"
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
	println!("Rust Testing");

```
%%

![[rust-logo.png]]

# RUST: Testing Strategies

notes:

Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.


---

# CONTENT HERE


---
%% (this is commented out to keep the ad spot under 1 minute, sdaly!)
# SPONSOR QUIZ TIME
- Old hardware
- 100 MPH
- Dynamic Data Centre Combinations
- No grid power
- No fibre internet

notes:

- What data centre contains 100 decade-old servers
- re-configures its entire network when it merges or splits with another data centre
- must operate without a reliable power source or internet connection,
- And travels at over 100 MPH?

To answer this, I will introduce Today's sponsor, Razor Secure
%%

<!-- slide bg="[[rs-train.jpg]]" -->

![[rs-logo.png|400]]
notes:
_(disclosure: The company's CTO is my brother!)_

- As with previous sponsors, Razor Secure don't want your money, they actually want to pay you.
- This is because they've asked me to tell you about their open full-stack and Rust positions at their company.

---

<!-- slide bg="[[rs-train.jpg]]" -->

![[rs-logo.png|200]]

1. Cross-platform Rust Agent
2. Cloud microservices
3. Embedded hardware platform

notes:

- RazorSecure is a 50-person startup bringing cutting-edge security tech to the rapidly-advancing world of trains
- They do this through:
   - A Rust intrusion detection and monitoring agent running on whatever hardware is installed on-board.
   - A cloud environment running K8s, Python microservices, and event-based data processing, and
   - A yocto hardware platform running custom embedded linux.
- Their team members and customers are based across Europe and North America, so if you have taken a train journey in any of those areas you may have already been kept safe by their security systems.

---

<!-- slide bg="[[rs-train2.jpg]]" -->
<split even>

![[Python_logo_icon.png|200]]
![[kubernetes-logo.png|200]]
![[rust-logo.png|200]]

</split>

notes:

- If you are a Python full-stack developer who is interested in Rust and are excited by this challenge and stack, then they are VERY interested in speaking to you as they are hiring NOW.
- The company is fully remote, so wherever you are based they offer challenging work in an interesting field with some awesome technology and a dynamic team.

---

![[rs-logo.png|200]]

[RazorSecure.com](https://www.razorsecure.com/)


[RazorSecure.com/careers](https://www.razorsecure.com/careers)

notes:

Find out more about RazorSecure at RazorSecure.com, and see their
open positions at RazorSecure.com/careers, and remember to mention No Boilerplate as your referrer so they know I sent you.

My thanks to RazorSecure for their support of this channel.


---


![[rust-logo.png]]

# Subtitle 


notes:

# OUTTRO

If you would like to support my channel, get early ad-free and tracking-free videos and vip discord access head to patreon.com/noboilerplate.

If you're interested in transhumanism and hopepunk stories, please check out my sci-fi podcast, Lost Terminal.

Or if urban fantasy is more your bag, click the bottom video to listen to a strange and beautiful podcast I produce called Modem Prometheus.

Transcripts and compile-checked markdown sourcecode are available on github, links in the description, and corrections are in the pinned ERRATA comment.

Thank you so much for watching, talk to you on Discord.

```rust
  println!("That's all folks!");
} 
```