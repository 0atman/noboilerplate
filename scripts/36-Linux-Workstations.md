<style>
:root {--r-code-font: "FiraCode Nerd Font";}
.reveal .hljs {min-height: 50%;}
</style>

![[nzxt-case.png|400]]

# Fast & Cheap

notes:

Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

We live in an incredible age.

We all have the 90s equivalent of supercomputers in our pockets, computers have never been so fast! but it doesn't FEEL like it, right?!

It's not you, computing HAS been getting worse, in lots of subtle ways.

Note that I don't say computers are getting worse, it's computing, the act of using a computer that is getting worse.

Computer hardware is much, much faster and much, much cheaper than most people realise.

The problem is LAPTOPS.

---

<split even>

![[minisforum.webp|300]]

&nbsp;
&nbsp;
&nbsp;
&nbsp;


![[mbp-m1.png|300]]

</split>

_700 USD_ [UM790 Pro](https://store.minisforum.com/products/minisforum-um790-pro?variant=43865372819701) &mdash; *3500 USD* Macbook Pro Max M1

## Similar CPU Performance

notes:
There are STAGGERING discrepancies between laptops and desktops, even with very small desktops like this mini pc on the left here.

The problem is thermal limits,

A PROBLEM THAT IT IS IMPOSSIBLE TO SOLVE IN LAPTOPS.

%%
Geekbench results for illustration m1 a little better in multi, ryzen a little better in single:
- https://browser.geekbench.com/macs/macbook-pro-16-inch-2021-apple-m1-max
- https://browser.geekbench.com/processors/amd-ryzen-9-7940hs
%%

---

![[cc-logo.png]]

## Public Domain Videos

[https://github.com/0atman/noboilerplate/](https://github.com/0atman/noboilerplate/)

notes:
Everything you see in this video: script, links, and images are part of a markdown document available freely on github under a public domain licence.

---

# Part 1: Hardware

---

![[stanly-rtea.png]]

notes:

It was not too long ago, perhaps 15 years, about when I started my career in web development, that most people did their computing on workstations, desktop computers with a separate monitor, keyboard and mouse.
- Artists
- Programmers
- Designers
- Video editors
- Musicians
- Educational YouTubers, and
- Producers and makers of all kinds

Laptops were simply not powerful enough for the graphics design, video editing, and compiling of large programming projects that we needed to do.

Laptops were for simple, portable tasks, browsing the web and writing, good for lecture notes, but it was understood that Real Work often had to happen on a desktop.

---

# The Laptop Revolution

notes:

Then a calamity was visited upon the world: Laptops became JUST fast enough to work for nearly everything you might want to do.

If you're looking for a watershed moment, I would point to Intel Atom's 2008 release as the beginning of the end.

Laptop performance and battery life became just slightly better than mediocre, and that, combined with most apps being offloaded to someone else's computer, accessed by the browser, allowed laptops to flourish, and hardware manufacturers pivoted to focussing almost entirely on this popular form factor.

---

# Today, Desktops Still Beat Laptops

#### In Price, Performance, and Upgradability

notes:

Today, nothing has changed: Desktops, powerful workstation machines, still eclipse laptops in performance.
Ask any pc gamer!

Desktop performance has not stood still while laptop performance has increased, we just stopped paying attention to it.
They have both increased together, with Desktops maintaining their enormous lead.
With a desktop, you still get much more for your money despite innovations in portable computing power.

The problem is physics.

---

# You Can't

# Have Your Cake

# And Eat it Too

notes:

Miniaturisation of components to fit into a sleek, light laptop is not without compromise.

Heat is the problem.

A laptop has a very small fan, and a very small mass of copper for the heatsink.

There is only so many kilos of copper the consumer is willing to carry with them!

It is deeply compromised for the amount of heat a high-performance task will kick out of the CPU and GPU.

---

## $$ E \Longrightarrow CPU \times N + HEAT $$

(I am a real scientist, and this is a real equation /s)

notes:

It's simple physics:
Electrical power in
computing power and heat out

If you want more computing power, you must increase the electricity input, which creates more heat.
The n coefficient here represents miniaturisation and cooling of the processor.

Smaller processors mean less distance for the electrons to travel, which means less heat, less literal friction of the electrons moving in the wire.
But we're coming up against hard limits in how small we can make our processors if we don't switch away from Silicon

If you can't make them smaller, you must make them SLOWER.

---

#### Same Model Number, ~3/4 of the Performance

![[cyberpunk2077-rtx-4090-pc-vs-laptop.png|700]]
&mdash; Techspot.com

notes:

The manufacture's solution to the problem of heat is to make the laptop chips WORSE.

Same price as the desktop version, dramatically worse performance.
Same model numbers as their desktop counterparts, dramatically worse performance.

And even when the laptop chips have similar base performance, in real load testing at room temperatures, the chip heats up and starts throttling speed to save itself from meltdown.

---

![[cinebench stress test.png|900]]

"[M2 MacBook Air Review - Needs More Air](https://www.youtube.com/watch?v=gqYaqBVT5vM)" &mdash; Dave2D
(you can fix this with [external cooling](https://www.tomshardware.com/laptops/macbooks/fanless-airjet-cooler-experiment-boosts-macbook-air-to-match-macbook-pros-performance)!)

notes:

Take the Macbook Air M2 and the Macbook Pro M2.

These machines have exactly the same cpu inside them, and for the first moment of the benchmark have the same performance.

But look at the fanless MacBook Air's speed fall apart as time goes on, and it heats up.
That's thermal throttling, and while all CPUs do it, standard desktop hardware simply doesn't have the prison of thermal constraints that laptops do.

Unfortunately it's not just about the physical constraints of the form factor.

We're forced into slow hardware also by economic sleight of hand.

---

### The Apple Problem

![[36-Linux-Workstations 2023-12-21 11.55.47.excalidraw.svg]]

PC upgrade cost: $300. *11x cheaper.*

notes:

[[36-Linux-Workstations 2023-12-21 11.55.47.excalidraw]]

An example.

On the left is the cheapest Macbook Pro M3, on the right is the same machine spec'd with 64GB of ram, and 4TB of disk.

See my sourcecode for the cpu differences between the two, but I'll remind you it's unimpressive.

Upgrading a PC with this amount of ram and disk should cost about $300.
But this macbook upgrade costs `=4899 - 1599` dollars!
11x more!

Apple can charge these prices because you can't upgrade the machines yourself.
No-one can.
Not even Apple, the ram and disk are non-upgradable.

Whereas, my favourite PC memory company, Crucial, will sell you 64GB of RAM for $100,
and 4TB of m.2 SSD for $200

This is a REAL PROBLEM for people who want powerful hardware and don't know that Apple are not playing fairly.

```md
- macbook pro M3 base
	- 8GB memory
	- 512 disk
	- 11cpu 14gpu 16ne
    - $1,599.00
- upgraded macbook pro M3 
	- 64gb memory
	- 4tb disk
	- 16cpu 40gpu 16ne
    - $4,899.00
```

---

![[patreon-apple-rant.png]]

## [Patreon.com/NoBoilerplate](http://www.patreon.com/noboilerplate)

notes:

I have a great deal more to say about Apple, but I had to cut my 3-minute unhinged rant short for this video.
If you'd like to watch the full version, it's on my patreon here.

It's just me making these videos, and I'm so grateful to everyone for supporting me so far.
I have big plans for 2024!

As a patron, you can see and give feedback on my videos up to a week early, vote on future video topics, as well as access private discord areas, and even get your name in the credits.

I'm also offering a limited number of mentoring slots. If you'd like 1:1 tuition on Rust, creative production, web tech, personal organisation, or anything that I talk about in my videos, do sign up and let's chat!

Thank you so much!

---

### In Defence of Laptops

![[thinkpad-laptop-train.jpg|700]]

notes:

To be clear, I don't hate laptops for mobile work.
I wrote this script on a train using one!

I dislike the assumption that they must be used everywhere, even when portability isn't needed at all.

I don't hate MacBooks either, I'll take one over a Windows laptop any day of the week, but I'd always prefer a Linux laptop, or, even better than that, a Linux desktop.

What hardware do I recommend, then?

---

## Recommendation

# Buy Desktop PCs

notes:
DESKTOP PCs

Cheap Desktop machines from 5 years ago are much faster compared to even the latest high-powered laptops, for a quarter of the price.

How do they do this?

It's simple economics: choice is good for the consumer.

---

![[itx-standard-layout.png]]

notes:

# Desktop ITX Standard

Have you ever wondered why desktops all look the same?

Unlike laptops, where the manufacturer designs the layout of the internals differently for each model, making spare parts obsolete every year.
Desktops have a standard, interchangeable, layout, with standard, interchangeable parts.

This means that next year you can double your ram or disk storage trivially, and then AGAIN the year after that.

You don't have to throw away the whole machine.

---

# What About Laptops for Mobile Use or Meetings?

notes:

Firstly, you're having too many meetings, see my pinned video for details.
Secondly, you mostly work from home, or a fixed office location, right?

And lastly, if you are at the kind of company or school that requires lots of in-person meetings or lectures, plenty do, buy yourself a really inexpensive, lightweight, laptop or tablet IN ADDITION TO YOUR INEXPENSIVE POWERFUL DESKTOP WORKSTATION for this purpose.
We've got wifi everywhere for a reason!

Don't shackle yourself to a slow laptop just because you
need to type notes on the go from time to time.

Chromebooks are ideal for this purpose, they cost basically nothing, and are perfect for companies and schools as they can be easily shared, each meeting room could have a stack charged and ready to use.

They even run Linux!

---

![[laptop-ergo.png|300]]

## Laptop Ergonomics Hurt US

note:

Typing on a laptop is terrible for us.
Touchpads are terrible for us.
A small screen down at table height is terrible for us.

We're killing ourselves in small ways, and we must stop this instant, and if you're buying for your team, it's insanity to oblige them to injure themselves for the sake of these slow, expensive machines.

---


![[think.png]]

notes:

The laptop form factor of a screen built into the lid covering the keyboard and touchpad is GENIUS, it's the standard for a reason.
But it sacrifices so much on the alter of portability.

If you don't need portability, you will be best served by a desktop computer.

And you don't usually need portability.

So, you've got cheap, powerful hardware.

But you don't want to run Windows, right?
No!

The next step is to pair powerful hardware with the best operating system on the planet, Linux.

---

![[tux-linux-logo.png|500]]

# Part 2: Software

#### Or, "Linux Has Been Ready This Whole Time"

notes:

---

![[gnome-screenshot-tiled.png]]

notes:

Linux runs lightning fast on the desktop, isn't beholden to Microsoft for windows updates, or apple and Xcode for basic libraries, and can run on any computer you like.
If you use an AMD graphics card, you don't even need graphics drivers, THE LINUX KERNEL HAS THEM BUILT IN.

And if you ALREADY do most of your work inside a web browser, google chrome is available here too, as well as my favourite, Firefox.

---

![[bitwig-screenshot-pianoteq.png]]

[Bitwig Studio](http://bitwig.com) on Linux

notes:

# My 16 Years Of Desktop Linux

My day-to-day O/S had been Ubuntu Linux since 2007, and I switched my music production to Linux on the 26 March 2014, when Bitwig was released, a gig-ready fully-featured Ableton Live clone.
If you're a musician, check it out!

I have written multiple albums since then, purely on Linux.

---

![[reaper-lt-editing.png]]

[Reaper](http://reaper.fm) on Linux

notes:
I do all my podcast audio editing on Linux too, using the excellent editor Reaper, which I can't say enough good things about, and it's made by the guy who created WinAMP!

---

![[reaper-video-editing-adhd-video.png]]

[Reaper](http://reaper.fm) on Linux

notes:

Reaper can also edit video, with both hardware-accelerated FFmpeg and VLC backends providing speed and hardware support that final cut and premier can only dream of.
I'm editing this video RIGHT NOW in reaper.

All of this works on any linux machine.

My whole production pipeline could run from a solar panel on a Raspberry Pi.

---

![[vscode-and-terminals.png]]

VS Code (and NCSpot!) on Linux

notes:

And it goes without saying that a cheap machine with a really fast CPU and 32 GB of ram is just what I need for my day-to-day programming.

And playing games!

---

![[steam-laptop-and-deck.png]]

notes:

# Steam

The last reason to keep Windows around, Gaming, has been slowly eroded since Valve integrated Proton (a zero-config Linux compatibility layer) into Steam, I remember celebrating when I first trivially ran Skyrim on this system in 2018!

The last nail in the coffin for Windows gaming was when Valve released the Steam Deck, running Linux and Proton natively, at the start of last year. Now all my games play great on Linux, on Deck or on my PC, and I have no Windows devices in the entire house!

---

<i class="fas fa-quote-left fa-2x fa-pull-left"></i>
_"Windows 11 scores dead last in gaming performance tests against 3 Linux gaming distros."_

### &mdash; [NotebookCheck](https://www.notebookcheck.net/Windows-11-scores-dead-last-in-gaming-performance-tests-against-3-Linux-gaming-distros.778624.0.html)

notes:

Linux even has a noticeable speed improvement ON THE SAME HARDWARE compared to windows.

This makes sense, because I'm sure Microsoft have a good team working on making windows fast, but Linux has the entire world developing it.

---

## Linux Workstation Superpowers

![[computer-opdate-stickycomics.png]]
[stickycomics.com](https://www.stickycomics.com/computer-update/)

notes:

Linux is a democratically-built operating system.

This means the annoyances that Microsoft and Apple oblige you to accept don't exist, there's no:
- pop-ups offering add-ons you don't need
- forcing restarts after updates, or
- charging for updates and features that should be standard,

- no sending your usage date off to "trusted third parties" for processing
- and steering you towards their own products and away from better alternatives 
	- (Microsoft edge and iTunes I'm looking at you)

And if there are features you don't like, you turn them off, it's your computer, you know best, not some company.

Linux doesn't have a corporate agenda, with shareholders expecting value extraction, it's a democratic, open, community.

---

### Package Management > AppStore

```sh
apt install vscode
```

```sh 
apt install firefox
```

```sh 
apt install steam
```

```sh 
apt install vscode firefox steam
```

(the three apps I use all day!)

notes:

Let me quickly highlight the first thing that sold me on linux, years ago: the ubiquitous built-in package manager.
If you've used brew on osx or winget on window, they're cute toy imitations of a real linux package manager.

- You don't have to go hunting on the web or app store for a program to install, it's as easy as apt install vscode or firefox or steam 
- OR ALL THREE AT ONCE
- in a single command you could install all the programs you need onto a new machine, or your friend's machine
- the same process does systems updates, and hardware drivers.
- and all without requiring constant restarts or locking of your computer.
- The mac terminal and windows WSL are poor imitations of this liberating environment.

---

![[asahi-linux-my-macbook.jpg|400]]
(please excuse the potato-quality photo!)

notes:

- Linux is a Universal Operating System
    - And you can even install it on a mac.
    - (though I don't recommend it)
    - Here's the macbook air I tried out for 3 months!


---

| **GOOD FOR THEM**            | **GOOD FOR YOU**                          |
|:-------------------------------------- | ------------------------------------- |
| No upgradability                       | Upgradability                         |
| One kind of machine for everyone       | Customisable machines for everyone           |
| Closed control of software environment | Open options for software envinroment |
| Your data on their servers             | Your data on your computer            |

notes:

At the start of this year, I tested Asahi Linux on Apple Silicon, but the experiment only lasted 3 months:

Apple's hardware was not the revolution in performance that their marketing pretended it to be.

Better than most laptops? Sure.
But that's not where real performance has ever been.

If you want the performance that any cheap gaming PC has, on a laptop, prepare to pay much, much more for it.

Why not build your own PC?

---

![[cavil-rules.png]]

You will not regret watching [A Superman Gaming PC](https://www.youtube.com/watch?v=G2gYUVQrLzQ)

notes:

If Cavil himself can do it, so can you!

Search YouTube for "Budget PC Gaming builds" and you'll find loads of simple recommendations to get started.

Here's mine: Buy the mini pc from the start of this video, and call it a day!

---

![[ubuntu-desktop-website.png]]

#### https://ubuntu.com/desktop

notes:

Whatever you build or buy, try Ubuntu, it's easy to install on any machine!

Have fun!

---

![[tri-hex-moon-white-transparent.png|300]]

# Thank You

## [Patreon.com/NoBoilerplate](http://www.patreon.com/noboilerplate)

notes:

# OUTRO

If you would like to support my channel, get early ad-free and tracking-free videos, vip discord access or 1:1 mentoring, head to patreon.com/noboilerplate.

If you're interested in transhumanism and hopepunk stories, please check out my weekly sci-fi podcast, Lost Terminal.

Or if urban fantasy is more your bag, do listen to a strange and beautiful podcast I produce every full moon called Modem Prometheus.

Transcripts and compile-checked markdown sourcecode are available on github, links in the description, and corrections are in the pinned ERRATA comment.

Thank you so much for watching, talk to you on Discord.
