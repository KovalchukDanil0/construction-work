#[deny(non_snake_case)]

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), leptos::prelude::ServerFnError> {
    use axum::{serve, Router};
    use construction_work::app::{shell, App};
    use dotenv::dotenv;
    use leptos::prelude::*;
    use leptos_axum::{file_and_error_handler, generate_route_list, LeptosRoutes};
    use tokio::net::TcpListener;
    use tower_http::{compression::CompressionLayer, CompressionLevel};
    use supabase_rs::SupabaseClient;
    use std::env::var;

    dotenv().ok();

    let conf = get_configuration(None)?;

    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let compression_layer = CompressionLayer::new()
        .br(true)
        .quality(CompressionLevel::Fastest);

    let supabase_client = SupabaseClient::new(
        var("SUPABASE_URL")?,
        var("SUPABASE_KEY")?,
    )?;

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .layer(compression_layer)
        .fallback(file_and_error_handler(shell))
        .with_state(leptos_options).with_state(supabase_client);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let listener = TcpListener::bind(&addr).await?;
    serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[cfg(not(feature = "ssr"))]
fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
