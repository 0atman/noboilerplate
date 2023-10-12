use color_eyre::Report;
use yate::html;

fn template(inner: String) -> String {
    html! {

            <!DOCTYPE html>
            <html lang="en">

            <head>
                <meta http-equiv="x-clacks-overhead" content="GNU Terry Pratchett" />
                <link rel="icon" href="favicon.png"/>
                <link href="bulma-min.css" rel="stylesheet"/>
                <script src="https://kit.fontawesome.com/333f3de551.js" crossorigin="anonymous"></script>

                <meta charset="utf-8"/>
                <meta name="description" content="No Boilerplate is a youtube channel of fast, technical videos"/>
                <meta content="width=device-width, initial-scale=1" name="viewport"/>
                <script src="https://kit.fontawesome.com/333f3de551.js" crossorigin="anonymous"></script>
                <title>"No Boilerplate Videos"</title>
                <style>"
                .navbar-item, .navbar-link, .has-dropbown :hover {
                    background-color: #0a0a0a !important;
                }
                .navbar-link:not(.is-arrowless)::after {
                    border-color: white;
                }
                .navbar-burger {
                    color: white;
                }
                //.hero {
                //    background: #000 url(bg.jpg) center / cover;
                //}
            "</style>
           </head>


        <body class="is-family-monospace">
            <section class="hero"/>

                <nav class="navbar has-background-black" role="navigation" aria-label="main navigation">
                    <div class="navbar-brand">

                        <a role="button" class="navbar-burger burger has-color-white" aria-label="menu" aria-expanded="false" data-target="navbar">
                            <span aria-hidden="true"></span>
                            <span aria-hidden="true"></span>
                            <span aria-hidden="true"></span>
                        </a>
                    </div>

                    <div id="navbar" class="navbar-menu has-background-black">
                        <div class="navbar-start">

                            <a class="navbar-item has-text-white" href="https://www.teepublic.com/user/no-boilerplate">

                                <span class="icon is-small">

                              <i class="fas fa-box"></i>
                            </span>
                              "&nbsp;Store"
                            </a>

                            <a rel="me" class="navbar-item has-text-white" href="https://tech.lgbt/@noboilerplate">
                                <span class="icon is-small">
                              <i class="fab fa-mastodon"></i>
                            </span>
                              "&nbsp;Follow on Mastodon"
                            </a>

                            <a class="navbar-item has-text-white" href="https://www.youtube.com/watch?v=Q3AhzHq8ogs&list=PLZaoyhMXgBzoM9bfb5pyUOT3zjnaDdSEP&index=1">
                              <span class="icon is-small">
                                <i class="fab fa-youtube"></i>
                              </span>
                              "&nbsp;Watch on YouTube"
                            </a>

                            <a class="navbar-item has-text-white" href="https://discord.gg/mCY2bBmDKZ">
                              <span class="icon is-small">
                                <i class="fab fa-discord"></i>
                              </span>
                              "&nbsp;Chat on Discord"
                            </a>


                        </div>

                        <div class="navbar-end">
                           <div class="navbar-item">
                               <div class="buttons">
                                   <a class="button" href="https://www.patreon.com/noboilerplate">
                                 <i class="fab fa-patreon"></i>
                             "&nbsp;Support me on Patreon"
                               </a>
                               </div>
                           </div>
                        </div>
                    </div>
                </nav>
                <section class="hero is-fullheight-with-navbar has-background-black has-text-light">
                    <div class="hero-body">
                        <container class="container">
                            <div class="columns">
                                <div class="column is-three-fifths is-offset-one-fifth">

                                {inner}


                                </div>
                            </div>
                        </container>
                    </div>
                </section>
            <footer class="footer has-text-white has-background-black">
            <div class="content has-text-centered">
                <p>
                        "I do not collect your data. Stop sending it to me."
                </p>
            </div>
            </footer>
        </body>

            <script language="javascript">
            r#"
    document.addEventListener('DOMContentLoaded', () => {

  // Get all "navbar-burger" elements
  const $navbarBurgers = Array.prototype.slice.call(document.querySelectorAll('.navbar-burger'), 0);

  // Check if there are any navbar burgers
  if ($navbarBurgers.length > 0) {

    // Add a click event on each of them
    $navbarBurgers.forEach( el => {
      el.addEventListener('click', () => {

        // Get the target from the "data-target" attribute
        const target = el.dataset.target;
        const $target = document.getElementById(target);

        // Toggle the "is-active" class on both the "navbar-burger" and the "navbar-menu"
        el.classList.toggle('is-active');
        $target.classList.toggle('is-active');

      });
    });
  }

      });
"#
    </script>
            </html>
             }
}

fn index() -> String {
    template(html! {
        //<figure class="image">
        //    <img alt="logo" src="logo.png"/>
        //</figure>

        <span class="is-family-monospace is-size-4">
            "No Boilerplate is fast technical videos."

        </span>

        <p class="has-text-center">
            <a href="https://www.youtube.com/watch?v=oY0XwMOSzq4&list=PLZaoyhMXgBzpr9Czgxj953GcUDkGlwa-Y" class="button has-text-right">
                "START HERE"
            </a>
        </p>
    })
}

fn errata() -> String {
    template(html! {

        <span class="is-family-monospace is-size-4">
            "ERRATA."

        </span>

        <p class="has-text-center">
            "Despite my best efforts I make mistakes on my videos all the time. When I or a commenter discovers one of these, I put a summary in the pinned ERRATA comment on the video. I wish I could edit the video easily to remove them, but that's not the way youtube operates. Please always check the videos for these fixes and changes. Thank you!"
        </p>
    })
}

fn discord() -> String {
    template(html! {

        <span class="is-family-monospace is-size-4">
            <a href="https://discord.gg/mCY2bBmDKZ">
                    <i class="fab fa-discord"></i>
                "Join us on Discord"
            </a>
        </span>

        <meta http-equiv="refresh" content="1;url=https://discord.gg/mCY2bBmDKZ"/>
        <br/>
        "(redirecting now...)"

    })
}

fn main() -> Result<(), Report> {
    std::fs::create_dir_all("docs")?;
    std::fs::write("docs/index.html", index())?;
    std::fs::write("docs/errata.html", errata())?;
    std::fs::write("docs/discord.html", discord())?;
    println!("Built site OK!");
    Ok(())
}
