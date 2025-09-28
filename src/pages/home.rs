use crate::components::{Billboard, Button};
use leptos::prelude::*;
use leptos_meta::Title;

#[server]
async fn login(username: String) -> Result<(), ServerFnError> {
    use axum::http::{header::SET_COOKIE, HeaderValue};
    use cookie::{Cookie, SameSite::Lax};
    use leptos_axum::ResponseOptions;

    // Build cookie
    let cookie = Cookie::build(("user", username))
        .path("/")
        .http_only(true)
        .same_site(Lax)
        .build();

    let Ok(header_value) = HeaderValue::from_str(&cookie.to_string()) else {
        return Err(ServerFnError::new("Cookie Value Error"));
    };

    // Get the ResponseOptions injected by leptos_axum
    expect_context::<ResponseOptions>().append_header(SET_COOKIE, header_value);

    Ok(())
}

#[server]
async fn load() -> Result<String, ServerFnError> {
    use axum::http::HeaderMap;
    use cookie::Cookie;
    use leptos_axum::extract;

    let guest = Ok("Guest".into());

    let headers = extract::<HeaderMap>().await?;

    let Some(cookie_header) = headers.get("cookie") else {
        return guest;
    };

    let Ok(cookie_str) = cookie_header.to_str() else {
        return guest;
    };

    let Some(user) = Cookie::split_parse(cookie_str)
        .filter_map(Result::ok)
        .find(|c| c.name() == "user")
    else {
        return guest;
    };

    Ok(user.value().to_string())
}

#[component]
pub fn HomePage() -> impl IntoView {
    let login_action = Action::new(|name: &String| login(name.clone()));

    let name = Resource::new(|| {}, |_| load());

    // A simple input field for username
    let (username, set_username) = signal(String::new());

    view! {
        <Title text="Home Page"/>

        <Billboard>
            <h1>"Construction Work"</h1>
            <p>"Discover Big Range of Components"</p>
        </Billboard>

        <h2>{name}</h2>

        <form on:submit=move |ev| {
            ev.prevent_default(); // stop page reload
            let name = username.get();
            // Call the server function through the action
            login_action.dispatch(name);
        }>
            <input
                type="text"
                on:input=move |ev| set_username.set(event_target_value(&ev))
                placeholder="Enter your name"
            />
            <Button {..} type="submit">"Login"</Button>
        </form>

        // Show when action is pending
        {move || if login_action.pending().get() {
            view! { <p>"Logging in…"</p> }
        } else {
            view! { <p>"Ready"</p> }
        }}
    }
}
