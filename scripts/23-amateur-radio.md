<style>
:root {--r-code-font: "FiraCode Nerd Font";}
.reveal .hljs {min-height: 50%;}
</style>

<split even>

![[amateur-radio-logo.png|300]]
&nbsp;
&nbsp;
&nbsp;
&nbsp;
# &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; Open-Source Airwaves
</split>

%% lol look at all that %%

notes:

# Amateur Radio: Open-Source Airwaves
%%
- Tell them what you're going to tell them
- Tell them
- Tell them what you told them
%%

---


# 📻
# 2E1OAT
(that's my radio callsign!)

notes:

Hi friends my name is Tris 2E1OAT and this is No Boilerplate, focusing on fast, technical videos.

Today I'm going to talk about a whole world I've discovered, a world that predated the internet and that will be here after the internet as we know it is gone.

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

You can sometimes catch glimpses of this society on rooftops.
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

notes:

In every city in the world, there are amateur radio operators.
Millions of us.

---

![[baofeng.png|700]]

notes:
If you want to become a ham, which is what we call ourselves, the price of admission is very low.
for $25 on amazon, you can have a simple radio delivered tomorrow.

---

![[cb-radio-car.png]]

notes:

If you've heard of CB radio, and you're thinking "oh this sounds similar".
You're getting there, but Amateur radio is so much more.

A short list of things you can do with amatuer radio include but are not limited to:

---

![[diana-eng-make-magazine.png|600]]



[Listening to Satellites with a Homemade Yagi Antenna](https://makezine.com/projects/homemade-yagi-antenna/)
&mdash; Diana Eng, MAKE Magazine

notes:

Communicate with amateur radio satellites.

---

![[chris-hadfield-iss-radio.png]]

notes:

AND EVEN WITH THE INTERNATIONAL SPACE STATION
This is Chris Hadfield, superstar astronaut, at the ISS's amateur radio station.

https://www.amsat.org/

---

![[earth-moon-earth-antenna-with-moon.png]]
notes:

You can bounce your signal off the moon and hear it come back 2 and a half seconds later.
If you coordinate, you can have a conversation across the world using the moon as a reflector.

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

https://parksontheair.com/

notes:

Because height means good reception, and many national parks are up high, there are regular organised events combining hiking and radio.
In the USA there's Parks On The Air, and if you want to go further -

---

![[pota-mountaineering-radio.jpg]]

https://www.sota.org.uk/

notes:

There's summits on the air!
The reception from the top of a mountain is quite something!

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

The frequency you'll be most familiar with is 2.4GHz, which is what both WiFi and Bluetooth run on, for the most part.

CB radio operates at about 27MHz, at the top of the HF band.
CB is much better in an urban environment than WiFi, and can travel up to about 4km.

Where does Amateur radio operate? 

EVERYWHERE

---

<split even>

![[arrl-license-book.png|400]]

&nbsp;
&nbsp;
&nbsp;
&nbsp;
&nbsp;

![[rsgb-test-example.png|400]]

</split>

notes:

Anyone can listen to any frequency - shortwave radio listening or even aircraft and marine listening is just a radio purchase away.
The radio waves were inside your house already, after all!
But if you want to transmit, you must have a license.

License procedures vary from country to country, but certainly in the UK and US the barrier to entry is very low.
You do not need to know morse code or understand complex radio jargon.
For the foundation exam in the UK I did just an hour of study, and then completed the exam entirely online in half an hour.

License restrictions vary slightly from country to country, but because it is by definition a global hobby, they are broadly aligned across the world.


---

![[january_2016_spectrum_wall_chart.jpg]]

notes:

Amateur radio is one of the great commons of our society, and it is nothing short of a miracle that we have them for our use.

Here is a graph of the whole frequency spectrum, from 0Hz all the way up to 300GHz.

This is the license space in the US, but it's about the same in every country - you can listen to any band, but you can't transmit.
The airwaves are cut up and divided.
Just like land use, someone owns it, or in this case, is licensing it from the government.
TV, Broadcast radio, military, and lots and lots of commercial areas.
You can use it, but you have to pay. Millions of dollars a year, in licensing fees, to the government.

There's no free space. Even the unlicensed areas you might have heard of, wifi, bluetooth, lora, CB and so on, are tiny slices of the airwaves with strict transmit power and antenna size limitations.

There's no free areas. except the Amateur Radio bands.

---

![[ham-january_2016_spectrum_wall_chart.jpg]]

notes:
Let me colour in the ham bands for you to make it clear.
They're EVERYWHERE, little slices of every single section, from low-frequency all the way up to microwave.

these areas aren't free to use, it's better than that, they are PROTECTED.

---

# Licence restrictions

1. No commercial use
2. No encryption
3. No broadcasting
4. Transmitter and receivers must be licensed. 

notes:

they are protected by the Amateur Radio License.

Broadly speaking here are the core limits of the amateur radio bands.

These restrictions prevent the tragedy of the commons occurring on our airwaves.

---

| Closed Source    | Open Source   |
| ---------------- | ------------- |
| WiFi & Bluetooth | Amateur Radio |

notes:

You can think of this as the distinction between closed source software and Open Source software.

Amateur radio, because it can not be encrypted, can be analysed 
it's source, so to speak, is open.
If an amateur invents a new encoding method that uses less bandwidth or is higher quality (as happens all the time!) anyone listening can hear it and figure it out.

The primary activities on the ham bands are learning, experimentation, and teaching.

---

# Code vs Encryption

notes:

Morse code is an encoding of data.
It's allowed on the ham bands because it's not designed to hide information, and even though most people don't know morse code, it's freely understandable by anyone who wants to learn.

SSL, RSA, even Enigma cyphers are all designed to hide information.
They are not allowed on the ham bands.

If you want to tunnel HTTP traffic through the amateur bands, you can, but you can't use SSL.

---
## Aside: Morse is COOL
 `−− −−− ·−· ··· ·` &nbsp; &nbsp; &nbsp; `−·−· −−− −·· ·`

![[morse.png]]

_(thanks @laund on discord for this awesome graphic!)_

notes:
BTW
I didn't think I'd ever be interested in Morse Code.
I'm delighted the basic exams don't require it any more.
But now that I understand a bit about how radio works, I see that the reason we're still using Morse, 186 years later, is because it's a really efficient use of the radio power.

I don't have time to explain in this short video, but for weak signal propagation between two humans, Morse can out-perform nearly every fancy computer-generated mode we have.
All without a microprocessor.

After the apocalypse, I want to know a Morse operator.
Or I suppose, before the apocalypse I SHOULD LEARN MORSE.

Let's talk about the apocalypse a little more.

---

<split even>

![[radio-ambulance.png|400]]
![[field-radio-operation.png|400]]

</split>

<split even>
![[car-radio-operation.png|400]]
![[field-radio-team.png|400]]

</split>


notes:

ARES RAYNET

the first thing that goes down in an emergency is the cell phone system.
Either through damage or congestion.

This is something that amateur radio clubs practice for.
When there is a disaster, perhaps an earthquake, hams are ready to spring into action, and set up field bases in the disaster site.
They then coordinate with hams back at the hospitals or wherever, to bridge the communications gap.

For example, they are currently helping the humanitarian effort in Ukraine.

The two organisations I am aware of are ARES in the US, and RAYNET here in the UK.

But your country will have something similar too.


But what if you live in a city, with all the buildings in the way?



More on that after I tell you about returning sponsor, Quadratic.
2:45

---


<!-- slide bg="rgb(37, 34, 43)" -->

![[quadratic-website.png]]

notes:

Quadratic are building an Open Source spreadsheet for engineers and data scientists, built in Rust, Webassembly and WebGL

---

<!-- slide bg="rgb(37, 34, 43)" -->
![[cell.png]]
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

This is all running at 60fps on the gpu using webgl, all inside your browser.


---


<!-- slide bg="rgb(37, 34, 43)" -->

![[quadratic-section-zoom-in-out.gif]]

notes:
Quadratic built their infinite canvas on webgl, allowing for smooth scrolling and pinch to zoom.

---

<!-- slide bg="rgb(37, 34, 43)" -->
![[quadratic-gpt.png|500]]

notes:

And just before I published this video, they released GPT integration, giving you a copilot or pair programmer while you're writing. Fantastic!

---


<!-- slide bg="rgb(37, 34, 43)" -->

![[quadratic-github.png|800]]

https://github.com/quadratichq/quadratic

notes:

it's open source and free to use.

---

<!-- slide bg="rgb(37, 34, 43)" -->

![[Quadratic Logo.png]]

The infinite canvas spreadsheet with code

https://QuadraticHQ.com

notes:

-   Signup today!
-   Head to QuadraticHQ.com to try it out.

My thanks to quadratic for their support of this channel.

---

![[repeater-on-mountain.png]]

notes:

To get your hand-held radio's signal across town, or over a hill you might need a repeater.

Cell phones don't have very long range.
In fact, you need towers in every neighbourhood to support them.
It's a clever idea, and one that Amateur Radio operators were experimenting with even before the first commercial cell phone in 1983.

There are repeaters EVERYWHERE, no matter where you live.

In decending order of my youtube audience , here are the repeater maps for -

---



![[usa-repeaters.png]]

notes:

The United States.

---


![[germany-repeaters.png]]

notes:

Germany.

---


![[eu-repeaters.png]]

notes:
Oh and the rest of Europe obviously.

---


![[uk-repeaters.png]]

notes:

My home country of the UK has a few.

---

![[india-repeaters.png]]


---


![[canada-repeaters.png]]


---


![[global-repeaters.png]]

notes:

You might note that as usual, this kind of map approximates a population map.

Wherever there are towns and cities there are repeaters!

If you want access to this world, it's cheap and easy, but like driving a car, there's a test and a license.
And just like driving a car it augments any outdoor activity you might want to do.

I've not even talked about the lower frequencies, where you can get worldwide propogation without using repeaters!

---

![[hackers-blades.png|700]]

(If you've not seen Hackers (1995), you're missing out!)

notes:
The dream of the 90s is alive and well in the Amateur Radio community.

---

![[qrz-tris.png]]

notes:

We even have our own 90s style social network that is so early-myspace-looking I'm surprised Tom hasn't started litigation.
this is qrz.com, the most popular ham log book site.
The name qrz comes from Q codes, which are short control codes originally used by Morse telegraph operators.

https://en.wikipedia.org/wiki/Q_code#Amateur_radio
QRZ means "Who is calling me?"

Everyone, including me, as you see, registers so you can log your contacts, which is an important part of the hobby, because you can't know if you're being heard unless someone tells you!

---

![[prop-report.jpg]]

notes:
There are a few people who set up automated propagation reports for some digital modes of operation.
Here's my report for a day last September when I was testing out my new antenna, for example.

Because I got these reports back, I can be reasonably sure my antenna was set up correctly

---

![[atmo-prop-handdrawn.png]]



notes:

# Personal communications

When you talk to someone on radio without a repeater, you are talking directly to them at the speed of light, from your microphone to their speaker, with no intermediaries.
Using the right frequency and antenna, this can be local in your city, or transcontinental, bouncing off the atmosphere or ground.

This direct communication feels much more personal - your voice is being mixed with a radio carrier signal or similar and broadcast in an analogue signal, then the reciever reverses the process to demodulate your voice.

It's analogue all the way, no bits, no bytes.

Isn't that wonderful?

---

> 73 - 2E1OAT

| Code | Meaning                 | Code | Meaning                |
| ---- | ----------------------- | ---- | ---------------------- |
| 1    | wait a minute           | 25   | busy on another wire   |
| 2    | Very important          | 26   | Put on ground wire     |
| 18   | What's the trouble?     | **73**   | **Best regards**           |

notes:

Have fun, get licenced, and maybe I'll chat to you on the air soon!

2E1OAT going clear, 73

---
![[tri-hex-moon-white-transparent.png|300]]

# Thank you
## [Patreon.com/NoBoilerplate](http://www.patreon.com/noboilerplate)
notes:

# OUTRO

If you would like to support my channel, get early ad-free and tracking-free videos and vip discord access head to patreon.com/noboilerplate.

If you're interested in transhumanism and hopepunk stories, please check out my sci-fi podcast, Lost Terminal.

Or if urban fantasy is more your bag, do listen to a strange and beautiful podcast I produce called Modem Prometheus. Season 2 premiered YESTERDAY!

Transcripts and compile-checked markdown sourcecode are available on github, links in the description, and corrections are in the pinned ERRATA comment.

Thank you so much for watching, talk to you on Discord.

```rust
  println!("That's all folks!");
} 
```




---

# VHF 144-146MHz
# UHF 430-440MHz

notes:

We'll start with the two most common amateur radio bands, VHF and UHF.

They are simple to start with, only requiring inexpensive radios.
The signals don't tend to go over the horizon, but 

With just these two bands, you can do SO MUCH FUN STUFF.
