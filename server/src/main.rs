use app::{App, shell};
use axum::{Router, serve};
use database::database_init;
use dotenv::dotenv;
use leptos::prelude::*;
use leptos_axum::{LeptosRoutes, file_and_error_handler, generate_route_list};
use tokio::net::TcpListener;
use tower_http::{CompressionLevel, compression::CompressionLayer};

#[tokio::main]
async fn main() -> Result<(), ServerFnError> {
    // do not change to "?", causes serious issues
    dotenv().ok();

    let conf = get_configuration(None)?;

    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let compression_layer = CompressionLayer::new()
        .br(true)
        .quality(CompressionLevel::Fastest);

    database_init()?;

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .layer(compression_layer)
        .fallback(file_and_error_handler(shell))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let listener = TcpListener::bind(&addr).await?;
    serve(listener, app.into_make_service()).await?;

    Ok(())
}
