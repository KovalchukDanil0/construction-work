mod components;
mod routes;

use components::{AdvantagesBanner, Footer, Header};
use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    Lazy,
    components::{Route, Router, Routes},
    path,
};
use routes::{ContactUsPage, HomePage, NotFound, SearchPage, AuthPage, CartPage, AboutPage};

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
        <Stylesheet id="leptos" href="/pkg/construction-work.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <Header />

            <AdvantagesBanner />

            <main class="flex h-full flex-1 flex-col items-start justify-start gap-6">
                <Routes fallback=||NotFound>
                    <Route path=path!("") view={Lazy::<HomePage>::new()}/>
                    <Route path=path!("contact-us") view={Lazy::<ContactUsPage>::new()}/>
                    <Route path=path!("search") view={Lazy::<SearchPage>::new()}/>
                    <Route path=path!("auth") view={Lazy::<AuthPage>::new()}/>
                    <Route path=path!("cart") view={Lazy::<CartPage>::new()}/>
                    <Route path=path!("about") view={Lazy::<AboutPage>::new()}/>
                </Routes>
            </main>

            <Footer />
        </Router>
    }
}
