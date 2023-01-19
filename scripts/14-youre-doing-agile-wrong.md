%%
<style>
:root {--r-code-font: "FiraCode Nerd Font";}
.reveal .hljs {min-height: 50%;}
</style>
%%

---

![[tri-hex-moon-white-transparent.png|300]]


# You're Doing

# Agile Wrong

notes:

Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

Today we're going to talk about a key part of software development that doesn't involve writing software.

We're going to talk about why agile at your company sucks.

---

## 1. Read the [agilemanifesto.org](https://agilemanifesto.org)

## 2. Do What is Valuable

## 3. Don't Do What is Not

notes:

I'm going to make some recommendations in this video, but if you're short on time you can stop here.

It's not just enough to do only what is valuable, you must resist doing what is not.

---

![[Gerald Benischke profile.png]]

## [Less is more agile](https://beny23.github.io/posts/my_take_on_engineering_room_9/)

- [@beny23@infosec.exchange](https://mastodon.social/@beny23)
- [github.com/beny23](https://github.com/beny23)

notes:

Much of this video is inspired by the excellent blog post "Less is more agile", by Gerald Benischke.

%% pron: bay neesh kuh %%

Thanks, Gerald!

In it, He doesn't say this in so many words, but I will:

---

# Agile Sucks

notes:

specifically the way it is understood in most companies around the world today.

Probably in your company right now there are overheads that annoy you.

If you are a software engineer, you might spend more than half your time in meetings NOT WRITING SOFTWARE.

This madness must stop.

---

# Breathe

notes:

Let me take a step back.

This isn't going to be a pessimistic video - I don't know how to make one of those - I actually have really good news for you:

---

# Agile is Great

notes:

Agile is great.

It's not just great, it's better than that.

---

# Agile is the Only Thing That Works

notes:

It's the only thing that works.

The principles of agile software development are so simple.

---

**Individuals and interactions** over processes and tools

**Working software** over comprehensive documentation

**Customer collaboration** over contract negotiation

**Responding to change** over following a plan

notes:

The values of interaction, working software, collaboration and responding to change, are universally agreed-upon.

But where did these principles come from?

It was obvious to the experienced manifesto authors, in 2001, that building software has been nothing like previous construction projects, like building bridges, railroads or cars.

And if you treat it like it is, it goes wrong.

For all of history, construction projects were better if they were planned meticulously.

If you build a house, you can't adjust the size of the walls to suit the roof if you discover that it doesn't fit when you build it.

but in software, everything is malleable, and huge changes can be made in a single day, sometimes in a single hour.

But the memory of history throws a long shadow:

We paint ourselves into a corner, again and again

---

## If The Cost of Failure is Less Than the Cost of Planning

# YOU MUST NOT PLAN

notes:

This is the core difference in our industry, I think.

Instead of taking 2 hours to plan a new feature, each engineer could build their own prototype in those 2 hours, and we'll pick the best one from the 10 choices. (and tomorrow we'll do it all again based on the customer's feedback)

The cost of failure, in building software is nearly zero.

What does that tell you about how much planning we should do?

Nearly zero.

Not zero, but nearly zero.

---

> Agile has come to mean doing half of scrum poorly and using Jira

&mdash; Andy Hunt, `@PragmaticAndy`

notes:

Andy Hunt, here, sums up the problem clearly.

Too often when a company says 'we do agile', what they mean is they have implemented the parts of scrum that they like, and are tracking projects in Jira.

Or trello, I suppose. Now that the company that makes Jira owns trello.

---

> SAFe is Shitty Agile For Enterprises

&mdash; Martin Fowler

notes:

here's a quote by mf, and it's indicative of a sickness in our industry - a plague of frameworks sold to management to supposedly improve agile.

What is SAFe, by the way? I looked it up: it's a nightmare.

---

![[safe-lean-agile-principles-eww.png]]

notes:

Even in their 'lean' principles here, SAFe appears to me to crush the agile methodology with heavyweight processes and tools.

Which is INCREDIBLE, because let me remind you,

---

# Individuals and Interactions

## Over Processes and Tools

notes:

The agile manifesto directly suggests de-prioritising processes and tools, and instead focusing on individuals an interactions.

---

# This Is The Wrong Kind of Cargo Cult

notes:

I'm sure that you can find a company and team for every tool in the SAFe book, or the Lean book, or whatever course you would like to sell to gullible managers.

But the whole idea is that your team is supposed to figure it out themselves
Every person on this planet is unique, therefore every team of people is a permutation of those already unique attributes.

How can scrum fit everything?

My recommendations after I shout-out another Rust-focused youtube channel, Code To The Moon!

---

![[cttm-smart-pointers.png]]

_"Rust's Alien Data Types 👽 Box, Rc, Arc"_

notes:

Ken makes similar videos to me, lots of Rust as well as linked technical topics.  

When I was first trying to understand Rc and ARc, his video "Rust's alien data types" explained these concepts in such clear language, I grokked it immediately.


---

![[cttm-hx-demo.png]]

_"Helix 🧬 the Rust Powered Development Environment"_

notes:

and his video on the Helix editor was a great explanation of the new motions and config you need to know, especially coming from vim as I do.

---

![[errata-cttm.png]]
notes:

The Code To The Moon videos have great audio, great video, and Ken keeps the script TIGHT, which you know I appreciate.

He's even adopted the ERRATA pinned comment, which I think shows great journalistic integrity.

Check him out, search for Code To The Moon on youtube or head to cttm.io.

Back to how much I love Scrum -

---

![[allen-holub-dont-like-scrum.png]]

notes:

I don't love scrum. Allen Holub doesn't think it's the best either.

%% pron: hol-uhb %%

What agile does very, very well, is build the right thing, iteratively, whilst involving the customer.

What it does very badly is answer the question "When will it be done?".

---

# What Agile is Not

notes:

And this is a problem, because the people with the money very reasonably want the answer to this question.

Scrum evolved as a lightweight wrapper around Extreme Programming (XP), to sneak Agile into an organisation that demands deadlines and project tracking.

But once deployed, the ceremonies of scrum become the fixation of management, and the whole thing only gets worse when agile training companies are looking to sell courses, qualifications, and books.

Soon even engineers think they hate agile.

---

# What's the Solution?

notes:

So what's the solution?

I have a few ideas for where you can start, firstly -

---

# Don't Estimate

notes:

Don't estimate!

Look back at the estimates you made last sprint.

Did the estimation exercise help you build better software?

Sure you might have tweaked the scope of the sprint.

But time and tide would have done this for you:

You know how to figure out how much is in a sprint? Work as fast as you can, then in two weeks see how much you did.

What would you have done better if you had estimated the individual stories?

By contrast, Gerald likes No Bullshit estimation here, which has three sizes of estimate:

---

## No Bullshit Estimation

![[no-bull-estimation-cards.png]]
notes:

- 1 Point
- Too ~~Fucking~~ Big
- No ~~Fucking~~ Clue

(I think this is actually where the name for NFCs came from, now that I see it written down)

---

## If The Cost of Failure is Less Than the Cost of Planning

# YOU MUST NOT PLAN

notes:

I'll remind you again, if estimation takes longer than doing the work, you morally must not do it.

And software is so fast to develop! that's why we're all here, right?

---

# Interlude

## A Love-letter to Software Development

notes:

In just a few lines of code, we can get the computer to do a set of instructions SO QUICKLY that it's indistinguishable from magic.

And some people don't know how easy this is, especially if they're non-programmers.

They look at the box with the blinking lights and reasonably assume it must be built in the same way that bridges, railroads, and cars are built.

---

# Interlude

## He's Off Again Isn't He, Classic Whimsical Tris

notes:

of course, you and I know, it's much, much more direct than that.

We've got access to a sort of field-programmable daemon, who can do our bidding and basically grant wishes.

We express those wishes in code.

And in silicon valley, they often come true.

Back to the concrete advice:

---

# No Sprints

notes:

sprints are called 'ceremonies'

(Which the dictionary defines as an action performed only formally with no deep significance)

You gotta remember that sprints are a framework to make agile palatable to management.
They don't actually help you build software.

Resist, and just deliver value, getting feedback from the customer as early as you can.

Build something and show it to the customer.

There's your plan.

---

# Fight Faux Agile

notes:

Agile is not scrum.

Agile is not sprints.

Agile is not stories or kanban boards or planning poker.

Agile is doing what is valuable, and not doing what is not.

---

## The Risk of Metrics

> "Tell me how you will measure me, and then I will tell you how I will behave."

&mdash;Eli Goldratt

notes:

Scrum has bad incentives, like velocity, over-planning, and doing exactly 2 weeks of work.

If you measure something and rate what is "good" on metrics that don't actually align with your core job of making good software that the user wants, then you have provided a metric that is in opposition to that core goal.

So what is the best metric?

I've told you already, and you already know.

---

# Working Software

## Over Comprehensive Documentation

notes:

Working software is the way to judge performance.

Not beautiful documentation.

Or designs.

Or prototypes:

Working software.

Though there is plenty of value in the others, we value working software the highest.

Anything that gets in the way of working software, by our user's definition of working, is a distraction.

---

## 1. Read the [manifesto](https://agilemanifesto.org)

## 2. Do What is Valuable

## 3. Don't Do What is Not

---

![[tri-hex-moon-white-transparent.png|300]]

# Thank you
## [Patreon.com/NoBoilerplate](http://www.patreon.com/noboilerplate)

---


# AGILE
# `!=`
# SCRUM


<!-- element style="font-size: 2.6em;" -->





notes:

# OUTTRO

Thank you for your time.
If you would like to support my channel, get early ad-free and tracking-free videos and vip discord access head to patreon.com/noboilerplate.

If you're interested in transhumanism and hopepunk stories, please check out my sci-fi podcast, Lost Terminal.

Or if urban fantasy is more your bag, click the bottom video to listen to a strange and beautiful podcast I produce called Modem Prometheus.

Transcripts and markdown sourcecode are available on github, links in the description, and corrections are in the pinned ERRATA comment.

Thank you so much for watching, talk to you on Discord.
