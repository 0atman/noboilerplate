<style>
.reveal .mermaid svg {
  min-width: 100%;
  height: auto;
}
.reveal .mermaid svg .label-container {
  fill: var(--r-background-color) !important;
}
.reveal .mermaid svg .label foreignObject {
  overflow: visible !important;
}
.reveal .mermaid svg .nodeLabel {
  color: var(--r-main-color) !important;
}
.reveal .mermaid svg .edgePaths path {
  stroke: var(--r-main-color) !important;
}
.reveal .mermaid svg marker {
  stroke: var(--r-main-color) !important;
  fill: var(--r-main-color) !important;
}
.reveal .mermaid svg .edgeLabel {
  background-color: var(--r-background-color) !important;
  color: var(--r-main-color) !important;
  font-size: 13px;
}
</style>

<style>
:root {--r-code-font: "FiraCode Nerd Font";}
.reveal .hljs {min-height: 50%;}
</style>

![[attachments/tri-hex-moon-white-transparent.png|300]]
notes:

## Tasks

- [ ] colour code parts
- [ ] brainmade logo

---
![](attachments/apple%20intelligence%20presentation.png)

🙄

notes:
Hi friends my name is Tris and this is No Boilerplate, where I make fast, technical videos.

So, Apple Intelligence has been out for a month now, but like a lot of tech today, it's a bit underwhelming, right?

---

![](attachments/blade%20runner%202017.png)

_Blade Runner_ (2017)

notes:

- The AI hype train is driven by the tantalising promise of AGI, general intelligence, like we see in the movies:
	- Maria, HAL, Marvin, Johnny Five, C-3PO, Rachel & Deckard, Data, Holly, JARVIS, WALL-E
- But, despite 4 years of promises, Apple Intelligence is the latest example of these products missing the mark.

---

![](attachments/almostuseless.jpg)

## _"Almost useless."_

&mdash; ["An honest review of Apple Intelligence", MKBHD](https://www.youtube.com/watch?v=haDjmBT9tu4)

notes:

## Apple Intelligence

The best two things that Marques, here, has to say about Apple Intelligence are:
1. The background eraser tool is pretty good, and,
2. it has bumped up the base ram across all of Apple's hardware line-up
    - This is well overdue, as I mentioned in my video on [[The Unreasonable Effectiveness of Linux Workstations]]

Apple Intelligence was announced at WWDC in June 2024, but didn't ship with the brand new iphones and the other hardware that was announced then, and only after months were these mediocre features released to us.

The _really good_ stuff is coming, we are promised.

Where have we heard that before?

---

![[attachments/cc-zero-2k.png|300]]

## Public Domain Videos

[https://github.com/0atman/noboilerplate/](https://github.com/0atman/noboilerplate/)

(for all [blue links]() read my scripts here ⬆)

notes:
My video scripts are dedicated to the public domain.

Everything you see here: script, links, and images are part of a markdown document available freely on GitHub at the above address.

---

# Part 1:

## The Magic Beans don't Work

notes:
- [ ] add a new colour
To understand what is happening with AI, it's useful to look at two other examples of tech that had HUGE hype but fell short of expectations.

With AI and all this tech we're bombarded with these days, it feel like they are all kindof falling short, doesn't it?

There's a reason for that:

---

# Part 2:

#### The Magic Beans don't Work

## Because they don't have to

notes:
- [ ] add a new colour
Back to AI.
Large Language Models Can only Learn Topics where there is a Large Amount of Language Available to train them.

---

## LLMs can't math

$$1+1=2$$

vs

$$2e^2+5j=0$$

notes:

For example, the reason LLMs can't do maths is because, apart from trivial examples, the state space of all numbers is too great to expect much existing training data

Contrast how often `1+1=2` is written in textbooks, easy for chatgput to do, with how often, say, `2e^2+5j=0` is.

One of these has a large amount of existing natural language data available for training, one does not.

---

![](attachments/chatgpt%20mistakes.jpg)

ChatGPT can make mistakes.

notes:

## Chatgpt

This is why tools like chatgpt seem good at first, when you ask it simple questions, but as you dig deeper, they fall apart and get increasingly inaccurate, or hit artificial guard rails and only provide surface level responses.

---

## LLMS CAN'T LEARN SPECIFICS

<split even>
![[attachments/matt-might-phd-infographic-elementary.png|300]]

&nbsp;

![[attachments/matt-might-phd-infographic-postgrad-research.png|300]]

&nbsp;

![[attachments/matt-might-phd-infographic-phd.png|300]]

</split>

School  &nbsp;  &nbsp;  &nbsp;  &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; University &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; Ph.D.

[matt.might.net/articles/phd-school-in-pictures/](https://matt.might.net/articles/phd-school-in-pictures/)

notes:

It's not that the technology is new, and will eventually get better, it's that this incredible language ability CAN ONLY WORK after being trained on large amounts of data.

By definition, there might be only a single PhD written about a very niche topic.
So GPT will never learn that information because a single PhD paper is not a large amount of language.

And if you don't have a large amount of language, you can't train a large language model.

AI companies can't fulfil their promises.

---

# Part 3:

## We are not the audience

notes:

- [ ] add a new colour
This dissonence between hype and reality seem confusing, until You Realise We're not the Audience for Them

If you scratch the surface, these technologies don't work.
And things that don't work can't solve problems.
And you can't sell someone something that doesn't solve their problem, not twice anyway.

So why do these companies keep making these promises?
~~(it's capitalism)~~

---

# CAPITALISM IS WORKING FINE

notes:

- It's not demand from the customers
- Nor direction from their engineers,
- not even, really, by choices made by their CEOs.

It's because the real decision-makers in these companies are their wealthy investors.

- mediocre AI tools (that we all hate) have been crammed into everything we use - now even our notifications - because the companies have to impress investors with AI features, even if they don't work well

---

## The Startup Runway

![[Excalidraw/startup runway.excalidraw.svg]]

notes:

When you work in tech startups, as I have over the past 15 years, you get to know the startup runway very well.

I wasn't always, as a lowly engineer, privy to the actual amount of funding coming in and salaries going out each month, but we would all be able to feel when the end of the runway was in sight.

You can typically extend your runway in two main ways:
1. Selling products and services to users for money
2. Persuading investors to part with more of their money

Selling products is HARD! They have to work!

But selling a promise? That's EASY.

---

# Plus ça change?

_The more things change the more they stay the same._

notes:

Plus ça change, perhaps? I'm not so sure.

---

![](attachments/shooting%20fish%20light.png)

_Shooting Fish_ (1997)

notes:

- GenAI is this perfect tool for tricking investors out of their money because, often enough, the people asking for the money think it works too!
- LLMs are great at basic stuff, and in the past when a computer could automate basic tasks to a good degree, it only required time, improvements, and of course money, to perfect.
	- As an investor, surely you'd better get in on the ground floor of this!
	- Just as you have before.

---

# Hypothosis:

## Maybe wealthy investors shouldn't be able to fsck up my notifications

notes:

Conclusion: From colonies on Mars to democratising money, it's always easier to promise a bright future than build a better present.

What I remind myself to do, whenever I see these bazaar products that no-one needs is to try to pay less attention to what these corps _SAY_ they will do in the future, and far more to what they actually _DO_ today.

---

# Thank You

To all my patrons, you make this possible!

%%

```rust
fn credits() {
```

%%

```rust
let producers: [&str; 0] = [];
let sponsors = [
	"Jaycee", "Gregory Taylor", "Ything LLC"
];
let patrons: [&str; 478];
```

%%

```rust
}
```

%%

### [Patreon.com/NoBoilerplate](http://www.patreon.com/noboilerplate)

### [ko-fi.com/noboilerplate](https://ko-fi.com/noboilerplate)

notes:

# OUTRO

Thank you.

If you would like to support my channel, get early ad-free and tracking-free videos, your name in the credits or 1:1 mentoring, head to my patreon or ko-fi.

If you're interested in transhumanism and hopepunk, please check out my weekly sci-fi audiofiction podcast, Lost Terminal.

Season 2 of The Phosphene Catalogue is out now, if you like mysteries and art, check it out!

Transcripts and compile-checked markdown sourcecode are available on github, links in the description, and corrections are in the pinned ERRATA comment.

Thank you so much for watching, talk to you on Discord.

```rust
  println!("That's all folks!");
} 
```
