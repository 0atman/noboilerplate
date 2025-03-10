use color_eyre::Report;
#[allow(clippy::wildcard_imports)]
use html_node::{
    typed::{elements::*, html},
    Node,
};

fn template(inner: Node) -> Node {
    html! {

        <!DOCTYPE html>
        <html lang="en">
            <head>
                <title>"No Boilerplate Videos"</title>
                <link rel="stylesheet" href="app.css"/>
                <link rel="icon" href="favicon.png"/>

                <meta http-equiv="x-clacks-overhead" content="GNU Terry Pratchett" />
                <meta http-equiv="X-UA-Compatible" content="IE=edge">
                <meta charset="utf-8"/>
                <meta name="description" content=""/>
                <meta content="width=device-width, initial-scale=1" name="viewport"/>
            </head>

                <body class="bg-black text-white font-mono text-sm md:text-2xl">

                    <nav class="flex items-center justify-between flex-wrap bg-black-500 p-6">
                        <div class="flex items-center flex-shrink-0 text-white mr-6">
                            <span class="font-semibold text-xl tracking-tight">


                <a href="https://www.youtube.com/watch?v=oY0XwMOSzq4&list=PLZaoyhMXgBzpr9Czgxj953GcUDkGlwa-Y" class="block lg:inline-block lg:mt-0 text-black-200 hover:text-white mr-4">
                No Boilerplate
    </a></span>
                        </div>
                        <div class="w-full block flex-grow lg:flex lg:items-center lg:w-auto">
                            <div class="text-xl lg:flex-grow">
                                <a href="https://www.youtube.com/watch?v=oY0XwMOSzq4&list=PLZaoyhMXgBzpr9Czgxj953GcUDkGlwa-Y" class="block lg:inline-block lg:mt-0 text-black-200 hover:text-white mr-4">
                                    Watch on YouTube
                                </a>
                                <a href="https://tech.lgbt/@noboilerplate" class="blocklg:inline-block lg:mt-0 text-black-200 hover:text-white mr-4">
                                    Follow on Mastodon
                                </a>
                                <a href="https://discord.gg/mCY2bBmDKZ" class="block lg:inline-block lg:mt-0 text-black-200 hover:text-white mr-4">
                                    Chat on Discord
                                </a>
                                <a href="https://noboilerplate.dashery.com/" class="block lg:inline-block lg:mt-0 text-black-200 hover:text-white mr-4">
                                    Store
                                </a>
                                <a href="https://www.patreon.com/noboilerplate/posts" class="bg-gradient-to-r from-indigo-500 block lg:inline-block lg:mt-0 text-black-200 hover:text-white mr-4">
                                    Support me on Patreon
                                </a>

                            </div>
                        </div>
                    </nav>

                    <div class="border-black border-8 container mx-auto">


                            <br/>
                            <br/>
                            {inner}

                    </div>

            </body>
        </html>
    }
}

fn index() -> Node {
    template(html! {
        <h1 class="text-4xl">
            FAST TECHNICAL VIDEOS
        </h1>

        <b>
            <a href="https://www.youtube.com/watch?v=oY0XwMOSzq4&list=PLZaoyhMXgBzpr9Czgxj953GcUDkGlwa-Y">
                Start Here
            </a>
        </b>
    })
}

type Router<'a> = Vec<(&'a str, fn() -> Node)>;

fn build(pages: Router) -> Result<(), Report> {
    for (page, fun) in pages {
        std::fs::write(page, fun().to_string())?;
    }
    Ok(())
}

fn main() -> Result<(), Report> {
    std::fs::create_dir_all("docs")?;
    let _ = build(vec![("docs/index.html", index)]);
    println!("Built site OK!");
    Ok(())
}
