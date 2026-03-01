<style>
:root {--r-code-font: "FiraCode Nerd Font";}
.reveal .hljs {min-height: 50%;}
</style>

![[git-logo.png|500]]

# Plain-Text Team

notes:
%%
- Tell them what you're going to tell them
- Tell them
- Tell them what you told them
%%
Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

All good teams are alike; each bad team is bad in its own way.
%% to paraphrase Tolstoy %%

Software is an incredible thing, isn't it?
Combined with the internet, a small team of friends can change the world overnight.

Every company, no matter what their industry, must now run a tech team, even if only to maintain their website.

So why are they all so bad at it?

---

![[cc-logo.png]]

## Public Domain Videos

[https://github.com/0atman/noboilerplate/](https://github.com/0atman/noboilerplate/)

notes:
Everything you see in this video: script, links, and images are part of a plain-text markdown document available freely on GitHub under a public domain licence.

---

# First World Problems

notes:
If you've worked in a web team, tech team or any digital creative team, you've likely felt the pain.

- Bad software,
- constantly changing processes,
- and lots and lots of meetings.

I discussed some of these problems in my Agile video that made me a lot of friends.
But today, I want to go bigger.
You can solve all these problems in a single blow.

The secret is, in order to do more, you must have the discipline to do LESS.

---

<split even>

![[rework-book.png|400]]

&nbsp;
&nbsp;
&nbsp;
&nbsp;
&nbsp;
&nbsp;

![[remote-book.png|400]]

</split>

notes:

A lot of the ideas that I will mention today are not new.
They've been well-understood in the startup and digital world for a long time.

But regression to the mean is prevalent.

It's not just enough to argue for good tools today, you must stop the future churn of new apps and processes that solve the same things in different, but equivalent ways.

And you do this with a Ulysses pact.

---

![[ulysses-and-the-sirens-waterhouse.jpg]]

_"Ulysses and the Sirens"_ [John William Waterhouse](https://en.wikipedia.org/wiki/John_William_Waterhouse)

notes:

# Tie Yourself to the Mast

%%pron. oh diss e us%%
In the Odyssey, Odysseus (confusingly called Ulysses in English literature) had to travel through siren-infested waters.

This was a well-understood problem in his world.
Sailors would simply solve this by putting wax in their ears, so the sirens' tempting song wouldn't lure them to their deaths.

But Odysseus had a challenge: He WANTED to hear the Sirens' beautiful song. He certainly didn't want to drown, so he ordered his crew to tie him to the mast of the ship, and to ignore any of his please to let him go, until safety.

This way, he was able to guard against future bad decisions he knew he would make by setting up a framework to control his future self.

This is the Ulysses pact, and it's a very common trick:

- Leaving your credit card or car keys at home when going out drinking is a Ulysses pact.
- Publishing a warrant canary on your company's website is a Ulysses pact,
- and standardising all your tools on plain text is a Ulysses pact.

---

![[the-fbi-has-not-been-here.png]]

An example of a warrant canary

notes:

In the future, you, or your successor, or your team might well be tempted to try the latest hot project management software, or documentation tool or scrum system.

While it might be good for a while, the act of changing tools constantly is an enormous overhead for your team, and one that gives the lasting impression that anything we write is likely to be legacy very soon, trapped in a deprecated app that "we just don't use any more", so why bother writing anything down.

Tying yourself to the mast by standardising on one tool, and not only that, but a plain text tool, means your data will live forever, and the network effect can make it more and more valuable over time, instead of less and less.

---

<i class="fas fa-quote-left fa-2x fa-pull-left"></i>
"The greatest problem in communication is the _illusion_ that it has been achieved."

## &mdash; William H. Whyte

[(not George Bernard Shaw, apparently)](https://quoteinvestigator.com/2014/08/31/illusion/)

notes:

# Decoupled Organisation Through Plain Text

Teams of people need to be on the same page.
Both literally and figuratively.

The natural way to do this is by talking to one another.
But talking does not scale, and is extremely impermanent.
After the sound waves have bounced off the walls and reverberated for a second... the words are gone, and what is left is our memory of them.

---

<i class="fas fa-quote-left fa-2x fa-pull-left"></i>
"The difference between science and screwing around is _writing it down_."

## &mdash; Adam Savage

notes:

Human memory is extremely unreliable, subjective, and the root cause of many problems.

After a discussion, it is not apparent that everyone has agreed upon exactly the same thing.
And you now need another meeting to double-check that.

The solution is documentation.

---

# Documentation-First Teams

notes:
Communication is most reliable when it is in black and white.

Everyone understands this, from 10,000-page government specifications to an email sign-off from the client you're making a 3-minute track for.

Yes, have more immediate conversations, by video, or chat, but write down what you concluded, and get the other person to confirm it.

---

# ["Documenting Architecture Decisions"](https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions)

(aka the _ADR_ process)

&mdash; [Michael Nygard](https://cognitect.com/authors/MichaelNygard.html)

notes:
You can improve every part of your team, business, or organisation by recording what decisions you have made, and WHY, in a system that allows for asynchronous discussion and improvements.

The ADR process is excellent for this, for example.

There are a thousand competing apps that claim to solve these problems for you.

---

<split even>

![[gdocs-screenshot.png|200]]

![[jamboard-photo.png|200]]

![[confluence-screenshot.png|200]]

![[pivotal-tracker-screenshot.png|200]]

![[notion-screenshot.png|200]]

![[trello-screenshot.png|200]]

</split>

notes:
These apps all re-invent the wheel in their own way, and new ones are being released every week.
I've used most of them, perhaps you have too, and they're all rubbish.

But there is a group of people who are extremely practised at managing enormous distributed, concurrent, text projects:

*Programmers!*

As an example, if you use Google Docs, your small team can collaborate on a few files a day, in a drive of perhaps a hundred or two hundred.
And just like in most other documentation systems, that won't scale.

Programmers simultaneously edit thousands of files a day, across repositories of data so numerous that we don't keep count.

What are programmers using, and can non-programmers use it too?

---

# Enter Git

- GitHub
- GitLab
- Bitbucket
- SourceForge
- Etc.

notes:

The answer is yes, yes we can.

I recommend you use the most popular distributed version control system on the planet: Git.

You'll use this through one of the many git web hosts, the largest of which is GitHub, which I recommend for most people.

---

# Popularity Matters

notes:

Though I mention GitHub primarily in this video, I'm not sponsored by them, or anything like that, I just acknowledge that popularity matters.
Support, experience, and integrations with other services will all be far, far easier if you use the standard.

All these tools started as a web interface around the incredible tool: Git.

---

## Aside:

# Linux & Git

notes:
By the way, the creator of Linux, Linus Torvalds, also later created git, to solve the problem that he created: that the Linux project had become SO LARGE that existing plain text collaboration tools were not scaling.

He jokes that he named his first project, Linux, after himself, and so it was natural to name the second one after himself too!

---

# Github Et Al. Are Greater Than the Sum of Their Parts

notes:
From simple code-hosting beginnings, these git services have grown to be so much more than that, trusted by the largest projects in the world, built by the largest companies in the world.

The foundation of my ideal team uses the raw materials that GitHub has given us.

What are the raw materials?

I'll show you this with a demo:
We're going to build a GitHub organisation for No Boilerplate.

This video is not sponsored by GitHub, my work is possible, thanks to viewers like you.

---

![[nb-patreon-aug-23.png|700]]

https://www.patreon.com/noboilerplate

notes:

If you'd like to see and give feedback on my videos up to a week early, as well as get discord perks, and even your name in the credits, it would be very kind of you to check my Patreon.

I'm also offering a limited number of mentoring slots. If you'd like 1:1 tuition on Rust, Python, Web tech, Personal organisation, or anything that I talk about in my videos, do sign up and let's chat!

It's just me running this channel, and I'm so grateful to everyone for supporting me on this wild adventure.

Let's make our plain text team:

---
![[repo.png]]

# Repos

notes:
The foundational unit with any git host is the repo.
This doesn't just correspond with one git repository, but one logical project or subproject.
Organisational tools like the Wiki (for documentation), Projects (for project management) and more can sit here, right next to your project's files, right where you need them.

---

![[wiki2.png]]

## Wikis

notes:
Each GitHub repository has a wiki, a folder of linked markdown files that anyone with access can edit, either in the friendly web editor, or, by cloning the wiki with git, on their own computer with whatever editor they like.

This is the minimum viable documentation tool, and it's useful for when git's full collaboration system isn't needed, and you just want to throw some linked markdown files together quickly.

---

# \# This is a Heading

### \#\#\# This is a Sub Heading

_\_this is italic text\__

**\*\*this is bold\*\***

\[[this is a link]()\](http://example.com)

_(learn more: [markdownguide.org/basic-syntax/](https://www.markdownguide.org/basic-syntax/))_

notes:

## Aside: Markdown is Great

Github, GitLab, and most of the Internet have standardised on Markdown.
Just like Slack, Discord, many websites, and sometimes Facebook depending on the phase of the moon, they all format text using this lightweight standard called Markdown.

Markdown is my favourite text format, it's really simple to use, and is designed to look good both in plain text and rendered as rich text, unlike HTML, which is unreadable by most people unless rendered in a browser.

Here we've got a heading, denoted by the hash symbol, italic with underscores, bold with double asterisks, and links using this bracket pairing syntax.

There are a few more options available, which you can look up at markdownguide.org, but this is the overwhelming majority of formatting you'll need on a day-to-day basis.

---

![[obsidian-kanban-paint.png]]

notes:

The genius of storing your data in this universal plain-text format is that should you wish to migrate from GitHub to another similar platform, your data is portable and under your control.

GitHub formats Markdown very nicely, but you can export it in any format you like, and edit it with any tool you like, present and future.

Including my favourite tool here, Obsidian.

Markdown keeps your team focussed on what is important by allowing you just enough formatting, but no rich customisation options.
You're not making a beautiful client brochure, so you shouldn't use 90s desktop publishing tools to make your company's critical documentation.

Back to GitHub's features:

---

![[issues.png]]

## Issues

notes:

Though not part of the git system, Issues are a natural addition that all git hosts have implemented:
A simple task system for capturing work that needs doing.
These could be new features, customer requests, bug reports, or ideas.
They have a rich comment thread for discussion, can be assigned to team members, and tagged with custom tags.

This minimum viable project management system could be all you need.
Certainly, for a solo or small team, capturing requirements in Issues might be enough.

But if you need more, you need Milestones.

---
![[milestones.png]]

## Milestones

notes:
GH milestones are a grouping of issues with a deadline.

They typically represent a target, a release, or something the team is working towards.
Milestones just have a title, a description, and a date.
No burndown charts, no swim lanes, no complex statistics, just a progress bar.

This might be enough project management for you.
If not, it is time for GH Projects.

---
![[projects.png]]

## Projects

notes:

Each GitHub repo, team, and organisation can have a project board, a lightweight kanban board with customisable columns, allowing you to group your issues together, and observe their progress through your current iteration.

This represents the information radiator for your team, a bird's-eye view of what is happening with the project, and something you might gather around for your morning catchup meeting.

You don't need all the features of Trello, JIRA, or anything like that. You need the minimum viable board.

This bare-bones tool completely side-steps 'the JIRA effect', which is if you have a tool that is packed full of time tracking, velocity points, and so on, the temptation is to use all these features, even if they give no value, and complicate your processes.

---

## Aside:

# Standups Are Great

notes:

If you have one meeting a day, it should be a standup.
Good standups replace other meetings and accelerate your project dramatically.

The way I like to do standups is not by asking everyone what they did yesterday and what they intend to do today.

That's a great way to find out at length what Dave did on his day off, but not a good way to find out what's happening with the project specifically.

I recommend walking your board, backwards, from right to left.
Everyone is timeboxed, keeping the meeting tight.
If you don't have an issue on the board, you don't speak yet. Perhaps you could write or pick up a task from the backlog and talk about it tomorrow.

At the end, the team member who is facilitating the meeting asks if anyone has any blockers, and we're done.

The most important part of this most important meeting is asking if you have any blockers.
A good standup means that no-one can get lost or delayed by more than 24 hours.

Note I didn't say the PM or DM or scrum master or whoever facilitates.
That person doesn't exist in my team.
These are not roles, they are hats.

I don't know if your team needs any other meetings, but it is vital that you do a standup.

Back to GitHub.

---
![[org-public.png]]

## Organisations

notes:
GitHub provides an umbrella group of users called an organisation.
This is your company, and if you're building your products in the open, as I recommend you do, you won't pay GitHub a thing.
Most git hosts provide their services for free for open-source companies.
If you have too much money, you can pay GitHub for a plan to make your data closed.

If you require more subdivision,
Organisations are divided into Teams.

---
![[teams.png]]

## Teams

notes:

Teams on GitHub allow you to granularly scope repo, project, wiki, and other permissions to the different teams in your organisation.

I recommend allowing everyone to write and contribute to all projects, you want the network effect and low admin overhead.
This pattern is called "internal open source".

But if you wish, perhaps for regulatory reasons, read and write access to repos can be restricted by team.

---

![[PR.png]]

## Pull Requests

notes:

Now we're getting into the detail of GitHub.
I adore pull requests, sometimes called merge requests in other systems.
PRs represent a change to the files in a repo, with an explanation of what you did, some links, and a discussion.

---
![[PR diff.png]]

## Pull Requests (diffs)

notes:

After the discussion is satisfied, perhaps as simply as a colleague saying "LGTM" or as heavy weight as a full change review with an audit trail that would satisfy a bank, the changes are merged into the repo.

PRs can have powerful automation, called Actions

---
![[action.png]]

# Github Actions

notes:

PRs and actions can run your company for you if you let them.
Though Actions were built for running tests on source code, with a little imagination, they can be used for anything:
- If you're uploading vector images, an action can build all the rasterised resolutions the client wants.
- If you're uploading video or audio clips, an action can run them through plugins to remove noise, add a music track, and upload the draft to YouTube.
- If you're checking in company documents, an action can simply spellcheck it.

Actions can run hundreds of times an hour, always adhere to best practice, and never make mistakes.

Automating your company gives you an enormous competitive speed and quality advantage.

---

![[language-tool-on-premise.png]]

notes:
You could also, for example, set up style guide enforcement, blocking the PR if the phrase "on premise" has been found.

---

![[nvme.png]]
notes:

# Offline Work

When your company's code, visual assets, and administration, are all in git repositories, you gain another huge superpower.

All of this becomes accessible offline.
Every file, every photo, every design, and document can be on your computer.
The magic of offline isn't necessarily that you don't need the internet (though that is a handy feature on a plane)
But that it's FAST, the data is RIGHT HERE on your computer, and you can do ANYTHING with it.
If you need to change the company's name across 10,000 files, it's trivial. It's find and replace.

If you, instead, had 5-15 different web services that you scattered all your team's data across, you'd have to log in to each one, and hope they had the feature to find and replace within their own walled garden.

Most, somehow, don't have this basic feature.

---

# Who is flying this thing?

notes:

GOOGLE DOCS doesn't have this basic feature: you can't find and replace across a drive of files!?

And the reason for this, I suspect, is that would be TERRIFYING, wouldn't it? What if a new hire accidentally did that, you'd have to roll back all those files manually, that could take days!

But with git, it would be a PR, clearly showing what was happening, and mistakes are trivial to fix.

Plain-text files, with just enough Markdown syntax to convey meaning, allow you and your team to work with this data in the way that they want, not the way that google or Atlassian or WHOEVER thinks is best.

Doesn't this take up a lot of space on your disk?
Not plain text.
But even if you're storing large files, storage is cheap if you're smart.
A topic for another video, perhaps.

---

# Future-Proofing

notes:
The benefits of git, especially for teams already using it for code and text collaboration, are that
1. you're already paying for it, and
2. It's never going away.

It is impossible to imagine a management reshuffle that might decide on another tool just because it is the flavour of the month.
Git and GitHub dominate the coding world, and I recommend GitHub not only because it is the biggest but also the most featureful.

---

![[killed-by-google-10-23.png]]

<https://killedbygoogle.com>

notes:

Even if you don't think git and plain text are the best option, I still suggest you use them because stability is far, far better than a constantly churning tool choice, as staff come, and go and fashions change, and Google decommission ANOTHER product.

RIP Jamboard :-(

---

# Conclusion

Click around my demo organisation for yourself:
https://github.com/noboilerplate

notes:

You can't do all these utopian things in most companies, I'm painfully aware.

But you CAN do SOME of these things, even just in your immediate team, or only for yourself.

I'd be interested to know what other ways good teams buck the hype cycle in favour of sane, evidence-based improvements.

Thank you.

---

![[tri-hex-moon-white-transparent.png|300]]

# Thank You

## [Patreon.com/NoBoilerplate](http://www.patreon.com/noboilerplate)

notes:

# OUTRO

If you would like to support my channel, get early ad-free and tracking-free videos, vip discord access or 1:1 mentoring, head to patreon.com/noboilerplate.

If you're interested in transhumanism and hopepunk stories, please check out my weekly sci-fi podcast, Lost Terminal.

Or if urban fantasy is more your bag, do listen to a strange and beautiful podcast I produce every full moon called Modem Prometheus.

Transcripts and compile-checked markdown source code are available on GitHub, links in the description, and corrections are in the pinned ERRATA comment.

Thank you so much for watching, talk to you on Discord.

%% NOW READ THE INTRO AGAIN %%
