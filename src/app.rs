use crate::{
    components::{Footer, Navigation, Header},
    pages::{HomePage, NotFound},
};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

const THEME: &str = "dark";

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" class="min-h-screen overflow-x-hidden" class:dark={THEME=="dark"}>
            <head>
                <meta />
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>

                <link rel="icon" href="/favicon.png" />

                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body class="mx-6 flex min-h-screen flex-col gap-6 bg-white md:mx-36 dark:bg-black">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/tailwind.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <div class="flex flex-row gap-3 justify-around items-center py-8 h-1/4">
                <Header />
                <Navigation />
            </div>

            <main class="flex h-full flex-1 flex-col items-start justify-start gap-6">
                <Routes fallback=||NotFound>
                    <Route path=path!("/") view=HomePage/>
                </Routes>
            </main>

            <Footer />
        </Router>
    }
}
