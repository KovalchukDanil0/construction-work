use leptos::prelude::*;
use leptos_meta::Title;

#[server]
async fn login(username: String) -> Result<(), ServerFnError> {
    use axum::http::{HeaderName, HeaderValue};
    use cookie::{Cookie, SameSite};
    use leptos_axum::ResponseOptions;

    // Get the ResponseOptions injected by leptos_axum
    let response = expect_context::<ResponseOptions>();

    // Build cookie
    let cookie = Cookie::build(("user", username))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .build();

    // Add Set-Cookie header
    let header_name = HeaderName::from_static("set-cookie");
    let Ok(header_value) = HeaderValue::from_str(&cookie.to_string()) else {
        return Err(ServerFnError::new("Cookie Value Error"));
    };

    response.insert_header(header_name, header_value);

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

        <h1>{name}</h1>

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
            <button type="submit">"Login"</button>
        </form>

        // Show when action is pending
        {move || if login_action.pending().get() {
            view! { <p>"Logging in…"</p> }
        } else {
            view! { <p>"Ready"</p> }
        }}
    }
}
