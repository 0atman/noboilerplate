---
audience: Linux users
---

<style>
:root {
	--r-code-font: "FiraCode Nerd Font";
}
.reveal .hljs {
	min-height: 50%;
}

.reveal .mermaid {
	min-width: 100%; height: auto;
}
.reveal .mermaid svg {
	max-width: 100% !important;
}
.reveal .mermaid svg .edgeLabel {
	background-color: var(--r-background-color) !important;
	color: var(--r-main-color) !important;
	font-size: 13px;
}

.contrast{
	background:var(--r-background-color);
}
</style>

---

![[nixos-icon.svg|800]]

notes:

# NixOS: Everything, Everywhere, All At Once

# The Best Linux Distro

Hi friends my name is Tris and this is No Boilerplate, focusing on fast, technical videos.

---

![[gnome-screenshot-tiled.png]]

notes:

I have dual-booted Linux, mostly Ubuntu, since 2006, and used it as my only daily driver for over a decade.
All of my videos, music, podcasts, and gaming is on Linux.

---

## What makes a linx distro?

- ✅ Package Manager
- ✅ Release cycle (stability, freshness)
- ❌ Desktop Environment
- ❌ Theme and colours

notes:

Linux distributions are primarily differentiated by their package manager.
And init system, I guess, but as a user who wants to get on with my life, I don't care very much.

They're certainly not differentiated by the desktop, gnome, kde, or whatever, and the fancy colour theme.
You can get that anywhere, these are general purpose machines.

Let's get more abstract:

---

- EVERYTHING
	- Does it have every package I want?
- EVERYWHERE
	- How easily can I reproduce it?
- ALL AT ONCE
	- Safely, with confidence

_"How many fresh packages can I safely install"_

notes:

The 3 most important aspects for a linux distro for me are Features, Reproducibility, and Stability.
(E E A A O)

And they all boil down to one simple principle:
How many fresh packages can I safely install.

There is standout winner in all of these categories and it's not even close:

---

![[map_repo_size_fresh.svg|800]]
[Tag yourself](https://repology.org/)

notes:

Here's the graph from Repology.org that made me realise I'd been sleeping on something special with NixOS, IT HAS MORE FRESH PACKAGES THAN ANY OTHER DISTRO.
Like, 3x more.
This astonished me!

Even the impressive Arch User Repository has fewer packages, and FAR fewer fresh packages,

Of note is that the inflection point was around 2022, looking at the old stable repos.

But the genius of NixOS isn't just the number of packages.

---

![[cc-logo.png]]

## Public Domain Videos

[https://github.com/0atman/noboilerplate/](https://github.com/0atman/noboilerplate/)

notes:
Everything you see in this video: script, links, and images are part of a markdown document available freely on github under a public domain licence.

---

## I Used to Use Arch Linux, Btw

![[pacman-update.png]]
notes:

Two years ago, I tried out using a steam deck as my main machine, it worked very well, then a year ago, tried Asahi Linux on Apple Silicon.
Both these Linux distros use Arch.
When I switched back to my main desktop after realising that Apple's hardware was not as good as they claimed, I kept using Arch. I had converted all my dotfiles and config , I might as well stay, I thought!

---

# Let Me Tell You

# A Story

notes:

well, One day, as happens now and then with a bleeding-edge rolling release distro , a Systemd update reversed my mouse buttons.
It only affected my exact mouse driver, which had a bug in it.

The great thing about rolling release and cutting edge distributions like Arch, is that you get to be a beta tester, whether you like it or not!
I filed a bug, and that afternoon it was patched.
The community is incredible.

Then a few weeks ago a kernel update stopped my machine from booting, and it seemed as though no-one on the whole internet had the same problem as me, and so I had to reinstall.

A tale as old as time.
I have good backups for a reason.

I thought this was the way it had to be on Linux: Stability or Cutting edge features, not both.

---

## Has NixOS Got a Deal for You

- ✅ More packages than Arch Linux's AUR
- ✅ Configure your whole system in one file
- ✅ Reproduce it INSTANTLY on another machine
- ✅ Perfect rollback if something goes wrong
- ✅ Native stable or rolling release, OR BOTH
- ✅ Auto-upgrade in the background without fear

notes:

NixOS is incredible in a way I didn't realise until recently.
Probably like you, over the years, I'd heard something about its declarative package manager, and system-wide rollback of updates.
But that sounded like an overly-cautious focus, I'm here for the latest and greatest, stability is for cowards!

But that wasn't right at all.

NixOS's genius is in its simplicity:

---

#### Traditional System Package Managers

```sh
apt install firefox
```

```sh
pacman -S firefox
```

```sh
yum install firefox
```

```sh
brew install firefox
```

notes:

NixOS is built on the nix package manager, which is nothing like apt or pacman or yum or brew.
You don't use it like this.

---

#### Language Package Managers

```json
{
  "dependencies": {
    "tailwind": "3.4.1"
  }
}
```

```ruby
gem 'rails', '5.0.0'
```

```ini
[project]
dependencies = [
  "django>5.0.0",
]
```

notes:

It's a package manager much more like npm, gem and whatever tool Python is using by the time you watch this.

To use these tools, you specify in a package file (be it package.json, Gemfile, or pyproject.toml) the exact packages you want, and then run a single install command to bring your system up to date with whatever you have declared in the file.

This is called declarative package management.

Imagine your whole system configured from a single declarative package file. That's NixOS.

---

![[nixos-installer.png]]
notes:

And you don't have to write the package file from scratch! NixOS has a graphical installer that looks like all the other Linux installers.
(it looks a bit better than the arch one, actually)

---

%% HEY YOU! DON'T COPY THIS, I edited it to fit on screen, do not boot this! %%

```nix[]
imports = [ ./hardware-configuration.nix ];
boot.loader.systemd-boot.enable = true;
boot.loader.efi.canTouchEfiVariables = true;
networking.hostName = "oatman-pc"; 
networking.networkmanager.enable = true;
time.timeZone = "Europe/London";
users.users.oatman = {
    isNormalUser = true;
    description = "oatman";
    extraGroups = [ "networkmanager" "wheel" ];
};
system.stateVersion = "23.11";
```

#### `/etc/nixos/configuration.nix`

notes:

Here's a slightly tidied up version of my base config that the NixOS installer gave me.

EVERYTHING is configured here, from the bootloader and efi config at the top, to the networking, timezone and the users on the machine.

You can think of NixOS, wrongly, as a templating config for system files.
Imagine the strings here get built into the /etc/fstab file, the /etc/passwd file, the networking daemon config - all without you having to learn anything about them, or how they work.

How many config files are on your machine right now? Do you even know where they all are?

While you're learning how to read the weird 20-year old Nix config format, just remember that it's the last config file you'll ever have to touch.

Cool, right?
It's worth the small upfront learning of this new format.
Everything configured from a single file!

well... it's two files, actually, but no more I promise!
See line 1 imports a hardware configuration?

---

#### `/etc/nixos/hardware-configuration.nix`

```nix
boot.initrd.availableKernelModules = [ "nvme" "xhci_pci"
boot.kernelModules = [ "kvm-amd" "amdgpu"]; 
fileSystems."/" =
{ device = "/dev/disk/by-uuid/c4a7f0..4b389";
  fsType = "ext4";
};
fileSystems."/boot" =
{ device = "/dev/disk/by-uuid/80..77";
  fsType = "vfat";
};
networking.useDHCP = lib.mkDefault true;
nixpkgs.hostPlatform = lib.mkDefault "x86_64-linux";
```

notes:

These two files are separate so that you can share configuration.nix across all your computers because each one has their own, separate, hardware config.

GENIUS defaults!

Here's an excerpt of my hardware configuration, generated by the NixOS installer automatically.

Kernel module config, file system mount points and networking tweaks. It's all here!

---

### Day to Day with NixOS

```nix
environment.systemPackages = with pkgs; [
  rustup
];

programs.neovim = {
  enable = true;
  defaultEditor = true;
};

programs.steam = {
  enable = true;
  remotePlay.openFirewall = true;
  dedicatedServer.openFirewall = true;
};
```

#### `/etc/nixos/configuration.nix`

notes:

But it's not JUST a list of packages, it's ALSO configuration for those packages!

Yes, you can throw all your normal packages in environment.systemPackages but that's not MAGIC enough!

There are NixOS options (which is what these are called) for everything from neovim to steam.
(Baulders Gate 3 is playing great on my machine by the way!)

---

## Reproducible Config

```sh
$ git clone git@github.com/0atman/mynix.git /etc/nixos
```

(there are better ways of doing this - you get the idea)

notes:

And once you've got your system config exactly how you like, version it with git, store it on a usb drive - anything you like.
Then build the same configuration on your laptop, the hardware config is separate, remember?

You could even drop the config on a live usb drive and have your own personalised live os.

Or temporarily boot a friend's machine with ALL YOUR CONFIG, then perfectly revert back when you're done.

---

### Rollback

![[nixos-bootloader.png|700]]
notes:

You can do this all fearlessly:

Every time you make a change to your system, by updating configuration.nix, in addition to applying the changes, nixos creates a new menu item that you can roll back to, RIGHT from the boot menu - it's impossible to leave yourself in an unbootable state because all previous configurations are saved.

This is like booting a previous kernel version, but with ALL THE OTHER MOVING PIECES reverted, too.

For someone who has suddenly become a professional online person, this is SUCH a relief when I have deadlines and videos need making, and an update ruins my day.
This can't happen to me anymore. Or if it does, I roll back, keep working, and fix it later on, after the deadline.

---

# Part 2

## TIPS

notes:

Part 2: Tips

Through trial and error here are all the tips I've found along the way as I have tumbled down the NixOS rabit hole

---

![[patreon.png|200]]

## [Patreon.com/NoBoilerplate](http://www.patreon.com/noboilerplate)

notes:

It's just me running this channel, and I'm so grateful to everyone for supporting me on this wild adventure.

If you'd like to see and give feedback on my videos up to a week early, as well as get discord perks, and even your name in the credits, it would be very kind of you to check my Patreon.

I'm also offering a limited number of mentoring slots. If you'd like 1:1 tuition on Rust, Personal organisation, creative production, Web tech, or anything that I talk about in my videos, do sign up and let's chat!

---

## Don't use `nix-env`

```sh
$ nix-env -iA nixpkgs.firefox

$ nix-env --uninstall firefox
```

notes:

Many nix tutorials talk about nix-env.
Don't use this, it doesn't interact with the global declarative configuration.nix, and when i first tried NixOS 4 years ago, I bounced off it because I couldn't see the point in learning this alien distro just to get non-reproducibility again.

Never use it.

---

`./rebuild`

```sh[]
#!/bin/bash
set -e
pushd ~/dotfiles/nixos/
nvim oatman-pc.nix
git diff -U0 *.nix
echo "NixOS Rebuilding..."
sudo nixos-rebuild switch &>nixos-switch.log || (
 cat nixos-switch.log | grep --color error && false)
gen=$(nixos-rebuild list-generations | grep current)
alejandra . &>/dev/null
git commit -am "$gen"
popd
```


(get the actual code from my [gist](https://gist.github.com/0atman/1a5133b842f929ba4c1e195ee67599d5) in the video source)

notes:

This is either genius or stupid, or perhaps more likely BOTH

When you use nixos, you do two things very often
1. Edit your config
2. Run `nixos-rebuild switch`, to switch to it

This is the primary loop of NixOS configuration, imagine how often you edit a config file or add a package or run a configuration command on other Linux distros.
All those actions and disparate configs are now unified, on NixOS, so nearly everyone builds their own script or alias for streamlining this process.

Here is mine, and I quite like it.

---

`./rebuild`

```sh[5]
#!/bin/bash
set -e
pushd ~/dotfiles/nixos/
nvim oatman-pc.nix
git diff -U0 *.nix
echo "NixOS Rebuilding..."
sudo nixos-rebuild switch &>nixos-switch.log || (
 cat nixos-switch.log | grep --color error && false)
gen=$(nixos-rebuild list-generations | grep current)
alejandra . &>/dev/null
git commit -am "$gen"
popd
```

notes:

First I edit my configuration in my editor.

---

`./rebuild`

```sh[5]
#!/bin/bash
set -e
pushd ~/dotfiles/nixos/
nvim oatman-pc.nix
git diff -U0 *.nix
echo "NixOS Rebuilding..."
sudo nixos-rebuild switch &>nixos-switch.log || (
 cat nixos-switch.log | grep --color error && false)
gen=$(nixos-rebuild list-generations | grep current)
alejandra . &>/dev/null
git commit -am "$gen"
popd
```

notes:

Then, after saving, git shows me what I have changed as a sanity check... 

---

`./rebuild`

```sh[8]
#!/bin/bash
set -e
pushd ~/dotfiles/nixos/
nvim oatman-pc.nix
git diff -U0 *.nix
echo "NixOS Rebuilding..."
sudo nixos-rebuild switch &>nixos-switch.log || (
 cat nixos-switch.log | grep --color error && false)
gen=$(nixos-rebuild list-generations | grep current)
alejandra . &>/dev/null
git commit -am "$gen"
popd
```

notes:

...before I type my sudo password and start the rebuild.

---

`./rebuild`

```sh[]
#!/bin/bash
set -e
pushd ~/dotfiles/nixos/
nvim oatman-pc.nix
git diff -U0 *.nix
echo "NixOS Rebuilding..."
sudo nixos-rebuild switch &>nixos-switch.log || (
 cat nixos-switch.log | grep --color error && false)
gen=$(nixos-rebuild list-generations | grep current)
alejandra . &>/dev/null
git commit -am "$gen"
popd
```

notes:

---

`./rebuild`

```sh[]
#!/bin/bash
set -e
pushd ~/dotfiles/nixos/
nvim oatman-pc.nix
git diff -U0 *.nix
echo "NixOS Rebuilding..."
sudo nixos-rebuild switch &>nixos-switch.log || (
 cat nixos-switch.log | grep --color error && false)
gen=$(nixos-rebuild list-generations | grep current)
alejandra . &>/dev/null
git commit -am "$gen"
popd
```

notes:

---

`./rebuild`

```sh[]
#!/bin/bash
set -e
pushd ~/dotfiles/nixos/
nvim oatman-pc.nix
git diff -U0 *.nix
echo "NixOS Rebuilding..."
sudo nixos-rebuild switch &>nixos-switch.log || (
 cat nixos-switch.log | grep --color error && false)
gen=$(nixos-rebuild list-generations | grep current)
alejandra . &>/dev/null
git commit -am "$gen"
popd
```

notes:

---

`./rebuild`

```sh[]
#!/bin/bash
set -e
pushd ~/dotfiles/nixos/
nvim oatman-pc.nix
git diff -U0 *.nix
echo "NixOS Rebuilding..."
sudo nixos-rebuild switch &>nixos-switch.log || (
 cat nixos-switch.log | grep --color error && false)
gen=$(nixos-rebuild list-generations | grep current)
alejandra . &>/dev/null
git commit -am "$gen"
popd
```

notes:

---

```diff
❯ rebuild 
```

_(I add neofetch to my config then `:wq`)_

```diff
~/dotfiles/nixos ~/dotfiles/nixos
--- a/nixos/oatman-pc.nix
+++ b/nixos/oatman-pc.nix
@@ -313,0 +314 @@
+    neofetch

NixOS Rebuilding...
activating the configuration...
setting up /etc...
[main 55e7e60] 160 current  2024-02-23 16:38:54 
 1 file changed, 1 insertion(+)
```

notes:

Here's what it outputs when adding a package, in this case neofetch.
After adding it into my package list, I save and quit vim, and the script continues.
- First, displaying a condensed diff, reminding me what I've changed across my nix files, it's simple in this case, of course 
- Then it kicks off a rebuild of my nixos config, throwing away most of stdout, I just don't care what's happening, as long as it's going fine.
	- I've already tabbed back to what I was supposed to be doing before I fell down a system configuration habit hole.
- If successfully built, the config is then autoformatted
- and then committed with the current generation's metadata as the commit message.
- Here we just built generation 160

---

```sh[3-7]
❯ git log

commit   12673c8f57e4384c[...] (HEAD -> main)
Author: Tris <contact@noboilerplate.org>
Date:   Fri Feb 23 16:39:23 2024 +0000

    160 current  2024-02-23 16:39:23 

commit  55e7e609b6c1f11a6[...] (HEAD -> main)
Author: Tris <contact@noboilerplate.org>
Date:   Fri Feb 23 16:38:56 2024 +0000

    159 current  2024-02-23 16:38:54 
```

notes:

And if we run git log, we see the latest commit is up to date.

This is a feature that's missing from nixos I think - while you can revert to any previous working generation of your  system, it doesn't revert your configuration.nix, which I assumed it would.

Versioning your config is the solution to this, and good practice, recommended everywhere.
Committing after every single successful build feels like working in a TDD environment, which I love.

If you're concerned about the number of commits this generates, firstly, don't, who cares, but secondly, the script doesn't push the changes, you could squash these commits into a larger feature before pushing.

---

![[nixos-bad-traceback.png]]

## WHO IS FLYING THIS THING!?

notes:

If something goes wrong in a nixos build, the vomit that hits your terminal is TERRIBLE! And a huge problem for newbies.

Call me spoiled, coming from the Rust world with the best errors in the business, but this is unacceptable, right?. 

Even simple errors like this variable typo are often hidden in the middle of an incomprehensible traceback, this one's relatively small.
The error isn't at the end, the error isn't at the beginning, it's in the middle.

This is just bad design.

---

```diff
~/dotfiles/nixos ~/dotfiles/nixos
--- a/nixos/oatman-pc.nix
+++ b/nixos/oatman-pc.nix
@@ -297 +297 @@
-    neofetch
+    neoERRORfetch
NixOS Rebuilding...
error:
       error: undefined variable 'neoERRORfetch'
```

notes:

Reading only the error is often as just as useful as reading the whole traceback.
The errors themselves are ususally pretty simple, and so my script just prints that, so you know where to look.

The full log for the last rebuild is saved in `nixos-switch.log`, if something has gone really wrong.
Also for very long-running compiles, such as entire browser builds, it's nice to tail this log to see what's happening.

---

```diff
~/dotfiles/nixos ~/dotfiles/nixos
--- a/nixos/oatman-pc.nix
+++ b/nixos/oatman-pc.nix
@@ -297 +297 @@
-    neoERRORfetch
+    neofetch
NixOS Rebuilding...
[main 3628d2a] 164 current  2024-02-24 09:58:40 
 2 files changed, 2 insertions(+), 2 deletions(-)
~/dotfiles/nixos
```

notes:

But if all goes well, your package is installed and ready to go.

---

![[neofetch.png]]

(minimum spec required to compile rust tbh)

notes:
And can run whatever new packages or config you just added, here's neofetch, for fun.

---

### Trivial custom services

```nix
  systemd.services.irc = {

      serviceConfig = {
        Type = "simple";
        User = "oatman";
        ExecStart = "screen -dmS irc irssi";
        ExecStop = "screen -S irc -X quit";
        }; 

      wantedBy = ["multi-user.target"];
  };
```

lightly edited for clarity, copypaste from:
<https://nixos.wiki/wiki/Extend_NixOS>

notes:

My favourite suprise nixos feature is that I can make new systemd services from arbitrary long-running commands, trivially!

Could I have done this by creating systemd units in ubuntu or arch? of course. But I tended not to, it was out of my normal routine.

By configuring systemd using the same system for my normal day-to-day package management, has made adding new services easy, mundane, normal, even.

And this is a subtle feature that touches everything in nixos.

---

| Arch Linux                                  | NixOS             |
| ------------------------------------------- | ----------------- |
| update `/etc/modules`?                      | configuration.nix |
| `update-mime-database ~/.local/share/mime`? | configuration.nix |
| `fc-list \| grep "FiraCode"`?               | configuration.nix |
| `/etc/gdm3/custom.conf`?                    | configuration.nix |
| `/usr/share/pipewire`?                      | configuration.nix |
| `usermod -aG group user`                    | configuration.nix |
| `MESA_LOADER_DRIVER_OVERRIDE=driver`        | configuration.nix |
|                                             | configuration.nix |
|                                             |                   |

notes:

- tweaking kernel modules is normal.
- changing xdg mimetype config is normal
- tweaking font settings is normal
- Changing x11 or wayland settings is normal
- adding advanced audio backends like pipewire or jack is normal
- Optimising opengl and hardware accelleration settings is normal
- and even adding my user to new groups is normal, something that I never sruggled with before, of course, but I never committed the exact command to memory, I'd look it up every time.

All these things and everything else is configured in a single file, in one simple syntax, so I don't have to look it up each time.

This is actually a huge deal.

---

## Stable or Rolling Release

## OR BOTH

```sh
$ nix-channel --add \
https://nixos.org/channels/nixos-unstable nixos
$ nixos-rebuild switch --upgrade
```

notes:

By default, NixOS will install a stable channel, where package testing happens before releasing them to the public, like most distributions.
But with two commands, you can switch to unstable ,bleeding edge!
Don't like it? Something terrible happened? Well, you know what to do, select the last working config at boot and rollback your changes.
simple.

---

### Auto-upgrade in the Background

```nix
system.autoUpgrade = {
  enable = true;
  flake = inputs.self.outPath;
  flags = [
    "--update-input"
    "nixpkgs"
    "-L" 
  ];
  dates = "09:00";
  randomizedDelaySec = "45min";
};
```

<https://nixos.wiki/wiki/Automatic_system_upgrades>

notes:

This same ease of fixing problems means that I felt, for the first time on any operating system, that I could just enable unattended background auto-updates of the whole system.
and everything would be fine.

Here's the snippet to add to your configuration.nix, you can grab it and loads of other tips from the nixos wiki

---

![](fas fa-wrench fa-2x)

# LD Fix

```nix
programs.nix-ld.enable = true;
programs.nix-ld.libraries = with pkgs; [
  # Add any missing dynamic libraries for unpackaged 
  # programs here, NOT in environment.systemPackages
];

```

<https://nix.dev/guides/faq.html>

notes:

NixOS is weird in a few ways, good and bad. The good outweighs the bad, but there's stuff to get used to.

One of these is that NixOS cannot run dynamically linked executables intended for generic Linux environments out of the box. This is because, by design, it doesn't have a global library path, nor does it follow the linux [Filesystem Hierarchy Standard](https://refspecs.linuxfoundation.org/FHS_3.0/fhs/index.html) (FHS).

Smart Nix people will tell you why this is, but I don't have the time, I'm too busy execution random binaries that wealthy princes email me on the daily.

Put these two lines in to allow normal binaries to work and get on with your life.

---

![[sioodmys-dotfiles.png|700]]
<https://github.com/sioodmy/dotfiles>

notes:

These ease of switching packages means that you don't have to start from scratch, you can drop-in other people's config and switch trivially, if you don't like it, rollback is a single command.

Maybe, like me, you'll install fresh to start with, but then want to try out other fun configs you see, that's easy!
Try out 10 in the same day!

8:00

---

### NixOS can be configured with TOML

```nix
# configuration.nix
imports = [
  (builtins.fromTOML (builtins.readFile ./myconfig.toml))
];
```

```toml
# myconfig.toml
[boot]
loader.efi.canTouchEfiVariables = true;
loader.systemd-boot.enable = true;

[programs.steam]
enable = true
remotePlay.openFirewall = true
dedicatedServer.openFirewall = true
```

(don't tell anyone)

notes:

And finally, look, I don't know if this is a good idea. I'm sharing this just between you and I and 200,000 of my closest friends.

When I first tried nixos, the byzantine configuration langauge confused me.
It seemed like toml, but it had semicolons, modules, and, after scractching the surface, I realised it was an entire turing-complete functional programming language.

After four years, I actually think the nix language is worth learning and getting used to, in the same way that for all its faults HTML is worth learning:
Because it's the standard, and you'll be able to copy and paste config you find on the internet.

However, there's nothing stopping you from using toml for a simple config like this.

If you can't get over the syntax, there is another way.

---

## Extras

- **nix flakes** = package.lock
	- exact reproducibility
- **home manager** = dotfiles management
	- I'm not sold on this (I'm using gnu `stow`)
- **Turing-complete config**
	- use if statements etc
- All packages get their own **independent dependency tree**
- **Nix dev setup**

notes:

There are a few more things to become familiar with, but you can do that while you learn your new system.
- Nix flakes are a modern feature of nix, allowing exact reproducibility and much richer configuration
- Home manager is a very popular nix-powered dotfiles manager that integrates well into the nix ecosystem. I'm not sold on this yet, but everyone loves it.
- An important implementation note is that all packages get their own dependencies, they are not shared. This uses more disk space, but NixOS is clever about re-using files and there's garbage collection options available.
- Nix, the underlying system, is incredible for building your dev environment per-project, too. This is how repl.it is able to support every language and every package without containers or VMs.

---

## [Ultimate NixOS Guide](https://www.youtube.com/watch?v=a67Sv4Mbxmc)

![[vimenjoyer-nixos-guide.png|700]]

⏯ [youtube.com/@vimjoyer](http://www.youtube.com/@vimjoyer) 💝 [ko-fi.com/vimjoyer](https://ko-fi.com/vimjoyer)

notes:

I have missed out so many details to keep this short but this video is the one you should watch next, Vimjoyer's fantastic, fast video on getting everything set up.

This video gave me the key breakthrough about nixos's enormous package repo, and dozens more. 

He's even got a repo with simple, starter, config to copy.

Go have fun! Try the latest packages, desktop environments and apps, 
And all without reversing your mouse buttons!

Thank you.

---

# Thumbnail Ideas

---

<!-- slide bg="[[nixos-everywhere.svg]]" -->

---

![[tri-hex-moon-white-transparent.png|300]]

# Thank You

## [Patreon.com/NoBoilerplate](http://www.patreon.com/noboilerplate)

notes:

# OUTRO

If you would like to support my channel, get early ad-free and tracking-free videos, vip discord access or 1:1 mentoring, head to patreon.com/noboilerplate.

I've got a new fiction Podcast out called The Phosphene Catalogue, if you like mysteries and art, check it out!

If you're interested in transhumanism and hopepunk stories, please check out my weekly sci-fi podcast, Lost Terminal.

Or if urban fantasy is more your bag, do listen to a strange and beautiful podcast I produce every full moon called Modem Prometheus.

Transcripts and compile-checked markdown sourcecode are available on github, links in the description, and corrections are in the pinned ERRATA comment.

Thank you so much for watching, talk to you on Discord.

---

- [x] write script [[@pc]]
- [x] edit video
- [x] video patreon approval
- [ ] publish final video
