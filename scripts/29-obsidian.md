<style>
:root {--r-code-font: "FiraCode Nerd Font";}
.reveal .hljs {min-height: 50%;}
</style>


![[obsidian-logo-gradient.svg|400]]
#### Hacking your brain with 
# Obsidian


notes:

Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

I have used many note-taking tools over the years, simple-note, notion, and extensively emacs org-mode, 
none have improved my thinking in the way that Obsidian has.

Today is a deep-dive into my second brain, and if you take my advice, YOUR second brain.

---

# Public Domain Videos

[github.com/0atman/noboilerplate/](https://github.com/0atman/noboilerplate/)

notes:
Everything you see in this video from the script to the images are part of a Markdown obsidian document available on GitHub under a public domain licence.

---

![[obsidian-hopepunk.png]]



notes:

As I mentioned in my coping mechanisms video, I have built my whole workflow for life around Obsidian.

---

![[org-mode-splits.png]]

_"Be productive with Org-mode"_
&mdash;[Ayrat Badykov](https://www.badykov.com/emacs/be-productive-with-org-mode/)

notes:

I used to use emacs' org-mode, and actually for a certain kind of delicate nerd, I'd recommend it more highly than obsidian.

I wrote the first year and a half of my sci-fi podcast Lost Terminal using org-mode, it's really great.

Try spacemacs if you want to give it a go.

There is much discussion of the pros and cons of org-mode vs obsidian that you can find online, but for me, the SYNCING story, specifically onto my mobile phone, was unacceptable.

Contrary to popular belief, I don't spend all my life in front of a computer.
Sometimes I go out for food.
And when I do, I can't leave my brain behind.

---

# Active Externalism

"**The Extended Mind**", [Clark](https://en.wikipedia.org/wiki/Andy_Clark "Andy Clark") and [Chalmers](https://en.wikipedia.org/wiki/David_Chalmers "David Chalmers") (1998)

_[JSTOR](https://en.wikipedia.org/wiki/JSTOR_(identifier) "JSTOR (identifier)") [3328150](https://www.jstor.org/stable/3328150)_

notes:

The theory I'm using here is called Active Externalism, part of the Extended Mind Thesis.
The idea is that our mind is more than our physical brain, it extends to paper, computers, abacuses, your fingers for counting, and for me, obsidian.

---

# Obsidian features
- Markdown based
- Cross-platform
- Perfect synchronisation
- Offline-first
- Comprehensive linking of notes
- Huge community of plugins
- ENORMOUS POPULARITY

notes:

Obsidian is freeware, not open source, and while that usually is a problem for me, it is not in this instance.
My data is open, stored as plain text markdown.

The two-person company behind obsidian may eventually go away, but my DATA is stored on my computer, in plain text.
Will Dropbox, Google Drive, and iCloud all exist in 10 years? I wouldn't bet my brain on it.

Obsidian is free for personal use, and the two value-add services Sync and Publish have a fair monthly fee to support the company.

It's perhaps as good a time as any to say that this video is not sponsored, I just love obsidian. 

Obsidian works everywhere, and where it doesn't, I still have the data in plain text, so I can edit and read it with any text editor.

Obsidian's sync is FANTASTIC and the reason I switched.
Better than Dropbox, better than iCloud, better than Git, or syncthing, better than whatever you're thinking of.

You don't need to pay for Obsidian Sync, however, there are many plugins that support Dropbox or git, but getting those working on a phone is non-trivial, so they get my money for their seamless native sync.

Offline first means that the data is stored on your computer, or phone. You control it, it's just plain-text files.

Like a wiki, obsidian is a set of linked pages, or notes. Building knowledge through links between pages is awesomely useful, and why we built the web in the first place.

At the time of recording obsidian has 1014 plugins in the built-in plugin installer, and you may easily side-load your own without going through this method if you would like.
Plugins are written in JavaScript and can change any part of the system.

Obsidian is the most popular of the markdown-based note takers, and in my opinion the best of all note systems.

---

# My path

0. Forgetting everything
1. Evernote (RIP)
2. Simplenote
3. Workflowy
4. Notion
5. Org-mode
6. Obsidian

notes:

I have tried nearly every cross-platform note-taking app of the last 2 decades.
The ones I've used seriously start with Evernote, and end with Obsidian.

ALL of these tools are great, and if you don't get on with obsidian, you could have a look at them.

But all of them except for the last two hold your data to ransom.
Notion, especially, is very insidious, because it LOOKS like Markdown in their pretty browser editor, but if you export it, the export is full of proprietary tags and symbols that are NOT portable.

Keep that in mind.

If you take my advice, you'll try obsidian first.

---


![[obsidian-website-downloads.png|500]]

##### https://obsidian.md


notes:

You can download it for all platforms even weird ones such as windows for arm, Raspberry Pi, and my distro, Asahi Linux running on apple slilicon.


---

![[obsidian-start-page.png]]


notes:

After you've installed, you will be asked where you would like to store your notes.
In Obsidian terms, this is called a vault, but it's just a folder.
Put it in your documents directory, in Dropbox, on a USB drive, or wherever you would like it to live.

You can also open a folder full of markdown files as a new vault, this is very handy for editing existing markdown folders, perhaps, books, blogs, or GitHub repos.

And the last option lets you log in with your obsidian sync account and download one of your synced vaults.

---

![[nbobsidian-empty-tab.png]]

notes:

When you first set up your obsidian vault, it will be empty, no markdown files, no notes, you will be greeted with this screen.
Let's do as it says and create a new note with Ctrl or Cmd n.

---

![[nbobsidian-web-work-init.png]]

notes:

For this demo, I'll name the first note the topic I'm currently researching, which we'll say is how the web works.

---


![[nbobsidian-web-browser-init.png]]
notes:

By pressing Ctrl N I can create new files, like here, a web browser note, where I'm going to write my research, including links to other ideas as they come up.

It's worth talking about Markdown formatting at this point.

---

# \# this is a heading
### \#\#\# this is a sub heading

_\_this is italic text\__

***\*\*this is bold\*\****

\[\[this is a link\]\]

_(learn more: [markdownguide.org/basic-syntax/](https://www.markdownguide.org/basic-syntax/))_

notes:

Obsidian, like slack, discord, many websites, and sometimes Facebook depending on the phase of the moon, formats text using a lightweight standard called Markdown.

Markdown is really simple to use, and is designed to look good both in plain text and rendered as rich text, unlike HTML, which is unreadable by most people unless rendered in a browser.

Here we've got a heading, denoted by the hash symbol, italic with underscores, bold with double asterisks, and links using double square brackets.

There are a few more options available, which you can look up here, but this is the overwhelming majority of formatting you'll need on a day by day basis.

Back to the demo

---

![[nbobsidian-dns-resolution-init.png]]

`- [ ] this is a checkbox`

notes:

I've followed the link into my DNS resolution note and added a few tasks for myself, using markdown checkboxes and some tags.

We'll see both of those again later.

But now, the thing every obsidian demo must have: The Graph.

---

![[nbobsidian-command-open-graph.png]]
notes:

The graph is accessed through the command pallet, and is how you get quick access to EVERYTHING inside obsidian.

Hit Ctrl P to open it.

Every function, button and feature of Obsidian is searchable from the list, and if you don't know exactly what you're looking for, just type some likely search terms, it'll fuzzy find it all!

---
![[nbobsidian-graph-init.png]]
notes:

# Graph

Here is our graph so far.
In the middle, our Web Browser note, and everything we've linked to from that page.
See we've only actually made notes for DNS resolution and the top-level How Does The Web Work note. That's why they are in white - nodes in grey are linked to, but not created yet.

This is huge.

This allows you to fill in the gaps in your knowledge incrementally.
Here it's clear I've not learned about JavaScript yet, it's a link to a note that doesn't exist.

Students and Researchers, pay attention.

---

![[nbobsidian-mobile-libris.png]]

notes:

All this works on mobile, by the way, perfectly synced with obsidian sync, or whatever method you would prefer - it's all just plain text files!

This is an example of my mature ish graph, by the way, I've coloured lost terminal scripts green, my daily journal notes orange, and in red is my video schedule main note.

Very pretty.


---
![[nbobsidian-web-work-backlinks.png]]

notes:

Let's click back to our original "how does the web work" note.

There are no Markdown links here, but I've clicked the backlinks tab on the right, and we can see that the Web Browser note links back to this note.

---

![[nbobsidian-canvas-backlinks.png]]

notes:

This, for database nerds, is like a foreign key relationship.

A list of all notes that links back to the note you are currently reading.
Obsidian allows you to follow links both ways along your knowledge graph, allowing you to explore your brain.

You click normal links to get from the `Web Browser` note to the other two notes on the right, and to go back, you follow the automatically generated backlinks.

---

![[nbobsidian-dns-resolution-backlinks.png]]

notes:

If we open the `DNS Resolution` note again, we can see the backlink to the `web browser note` here too.


---

![[nbobsidian-web-browser-outgoing-links.png]]
notes:

Let's open that Web Browser note, you can see in purple the links we just saw, but also I have clicked on the outgoing links tab on the right, and we can see them there too.

Note the two different icons indicating a link to a page that exists, and pages that don't exist.
Same as the white and grey nodes on our graph.
Same as the bright and dim purple links in the note body itself.

OK, OK, OK Tutorial over, go test it out for yourself, it's way easier in person.

Let's look at PLUGINS.

---

![[nbobsidian-canvas-demo.png]]
notes:

Plugins change the way obsidian behaves in small or large ways, up to and including embedding entire new ways of working with your data, like here, the built-in canvas plugin, which I used earlier.

This allows, instead of working with your notes in tabs, or splits or windows, you to throw them on a canvas in whatever layout you would like.

Notice that I am actually EDITING the Web Browser note in this screenshot, it's not just about layout.

How does notion store canvas data?
There are limits to Markdown, of course.

---


```js
{ // 🗎 web research.canvas
  "nodes": [
    {
      "id": "a031632ece723b5f",
      "x": -780,
      "y": -520,
      "width": 400,
      "height": 400,
      "type": "file",
      "file": "How does the Web work?.md"
    },
    {
      "id": "e92055d16f24da48",
...
```

notes:

The standard for storing non-document data is a human-readable JSON file.
Canvas and all other plugins store their metadata in this portable format.

No binary databases anywhere, everything is plain text.

---

![[nbobsidian-core-plugins-top.png]]

notes:

All native obsidian functionality can be toggled on and off inside the settings. 
Most of the core plugins are enabled by default, though if you would prefer a more minimal, streamlined app, you could turn them all off and still have the best markdown editor on the planet.


---

![[nbobsidian-core-plugins-bottom.png]]

notes:

There are a few plugins disabled by default:
Publish, slides, sync, and workspaces are the big ones.

One that is enabled by default that I will highlight is tag support.


---

![[nbobsidian-web-work-tags.png]]

notes:

You can tag your notes in two main ways:
In the body of the note with a hashtag, or in the frontmatter of the page.

The frontmatter is a section of configuration at the start of the document where you can add your own keys and values, and plugins can then act on this metadata.
The built-in tag plugin, for instances, reads the list of tags from this frontmatter, as well as inside the body of the note.

frontmatter is in YAML format, another open standard.

---

![[nbobsidian-graph-with-tags.png]]

notes:
Now that I've tagged the how does the web work document with research, web, and todo tags, they show up on the graph, by default, in a different colour.

I also added some tags to the other documents at the same time.


---

![[nbobsidian-tag-search.png]]
notes:

You can search for tags, like here in the top left, I'm searching for all notes with the DNS tag, there's only one result of course


---

![[nbobsidian-plugin-tagfolder.png]]

notes:

Linear folder organisation is a little restrictive, so let's install the TagFolder plugin from community plugins, in obsidian settings.

This plugin shows tags like folders.

---

![[nbobsidian-tagfolder-demo.png]]

notes:

Here on the left, you can search through nested tags in a folder-like structure.

These are the two main ways of organising your obsidian vault, by the way, Tags or folders.
You can either have no folders, organising everything with tags, or a traditional folder tree.

I'm not sure which is best. Try them both!

---
![[nbobsidian-dataview-settings.png]]

notes:

Now we're going to unlock obsidian's real power with the dataview plugin.

I think this plugin should be built-in, it's so ubiquitous.

Install it, and then in the dataview settings, enable all the query toggles at the top here.

---

# Code blocks

````sql
```dataview
Table
file.ctime as Created
FROM #todo
SORT file.ctime DESC
LIMIT 10
```
````

notes:

Sidenote:

Markdown has a way to add code snippets, perhaps HTML, JavaScript or any other code you would like, inside triple backticks, with an optional name for syntax highlighting on the first line.

This is handy for writing code documentation of any kind, not just programming, but wikimarkup, bbcode, discord formatting, or anything like that, and dataview uses it to encode its queries.

---

![[nbobsidian-dataview-tasks-edit.png]]

notes:

Dataview allows you to treat your vault, your brain, your collection of linked, tagged markdown files, as a database.

You can query this database simply using SQL, or get programmatic access to it using inline JavaScript.

If you know how to code, you already can imagine how the JavaScript query SDK works, so I will focus on simple SQL queries, which are what most non-programmers will use.

This code block, when you click away from it, renders the dataview query.

---

![[nbobsidian-dataview-tasks-render.png]]
notes:

And becomes a table, listing all notes tagged todo, ordered by their creation time.

Dataview can filter notes by date, length, tags, titles, links, backlinks anything you can imagine, and you can build your own systems and rules simply using it.

---
![[nbobsidian-mapview-init.png]]

notes:

The last plugin I want to demo is the Map View plugin.
Install it in the same way as all other plugins, through settings and then community plugins.


---

![[nbobsidian-mapview-command.png]]

notes:

Now hit CTRL-P to open up the command pallet, and type "map front matter".

---
![[nbobsidian-mapview-command-search.png]]

notes:
This will open up a location search for any place in the world, the results provided by open street map, though the Google Maps API is togglable in the settings.

---
![[nbobsidian-mapview-ucl.png]]
notes:

Here I've done that for UCL, in London.
Note the coordinates in the frontmatter of the document.

---
![[nbobsidian-mapview-ucla.png]]

notes:

UCLA, in California.


---



![[nbobsidian-mapview-ucsb.png]]

notes:
And UCSB.

---
![[nbobsidian-mapview-final.png]]

notes:

Now my research is starting to really come together, with the primary ARPANET sites, the precursor to the internet, graphed inside my obsidian vault.

---

![[obsidian-graph-nicole.png|700]]
[https://nicolevanderhoeven.com](https://nicolevanderhoeven.com/blog/20210518-how-i-use-obsidian-at-work/)

notes:

Just like the internet, as you build your brain, the links between notes become exponentially greater, and obsidian becomes exponentially more useful!

---

|                 |                           |
| --------------- | ------------------------- |
| Advanced Slides | How I make these videos!  |
| Day Planner     | Daily notes and plans     |
| Dice Roller     | For D&D!                  |
| Excalidraw      | Drawing and simple graphs |
| GPT-3 Notes     | Simple chatgpt interface  |
| Kanban          | Trello IN MARKDOWN        |
| LanguageTool    | Comprehensive stylecheck  |
| Linter          | Markdown style formatting |

notes:

I've run out of time! There are SO MANY COOL PLUGINS that I simply can't get round to talking about today.

(read list)

My full list is in the Markdown document I built this video with, using Obsidian and the Advanced Slides plugin, check the description for links to my github for the document

Try obsidian out, I'm excited for you, and comment on this video if you need any help!

(hi curious friends!)

```json
[
  "dataview",
  "homepage",
  "calendar",
  "table-editor-obsidian",
  "obsidian-day-planner",
  "obsidian-dice-roller",
  "obsidian-dictionary-plugin",
  "cm-editor-syntax-highlight-obsidian",
  "obsidian-emoji-toolbar",
  "obsidian-excalidraw-plugin",
  "obsidian-5e-statblocks",
  "file-explorer-note-count",
  "obsidian-file-info-plugin",
  "gpt3-notes",
  "obsidian-kanban",
  "obsidian-languagetool-plugin",
  "obsidian-linter",
  "obsidian-map-view",
  "music-code-blocks",
  "metadata-menu",
  "obsidian-mind-map",
  "obsidian-nomnoml-diagram",
  "obsidian-leaflet-plugin",
  "obsidian-plugin-todo",
  "url-into-selection",
  "darlal-switcher-plus",
  "quickadd",
  "obsidian-read-it-later",
  "recent-files-obsidian",
  "scales-chords",
  "simple-embeds",
  "obsidian-tagfolder",
  "wikipedia-search",
  "obsidian-zoom"
]

```


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
