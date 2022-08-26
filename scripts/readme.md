These are the scripts for my fast technical videos.

Slides are separated with `---` and notes are deliniated with `notes:`.

I present them with the Advanced Slides plugin in Obsidian.md, which are broadly reveal.js compatible. 

For presentations after 28/07/22, code inside the markdown can be built, and therefore tested, using the makefile in this folder. It uses a combination of `literate` and `cargo-script` as you'll see. Install both from cargo (`cargo install literate`)

I'm still working out this method, and I'm afriad the `Cargo.toml`s are not currently included. That's a problem I will fix. Untill then, you'll need the crates that are mentioned or imported in the presentation.

Prior to video 9 I hadn't got the exact format sorted out, apologies. 9 and onward should build reliably.