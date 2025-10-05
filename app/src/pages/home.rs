use crate::components::{Billboard, Button};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::{lazy_route, LazyRoute};
use serde::{Serialize, Deserialize};

pub struct HomePage {
    name: OnceResource<Result<String, ServerFnError>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginData {
    username: String,
}

#[server]
pub async fn login(data: LoginData) -> Result<(), ServerFnError> {
    use axum::http::{header::SET_COOKIE, HeaderValue};
    use cookie::{Cookie, SameSite};
    use leptos_axum::ResponseOptions;

    // Build cookie
    let cookie = Cookie::build(("user", data.username))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .build();

    let Ok(header_value) = HeaderValue::from_str(&cookie.to_string()) else {
        return Err(ServerFnError::new("Cookie Value Error"));
    };

    // Get the ResponseOptions injected by leptos_axum
    expect_context::<ResponseOptions>().append_header(SET_COOKIE, header_value);

    Ok(())
}

#[server]
pub async fn load() -> Result<String, ServerFnError> {
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

#[lazy_route]
impl LazyRoute for HomePage {
    fn data() -> Self {
        Self {
            name: OnceResource::new(load())
        }
    }

    fn view(this: Self) -> AnyView {
        let HomePage { name } = this;

        let login = ServerAction::<Login>::new();

        view! {
            <Title text="Home Page"/>

            <Billboard>
                <h1>"Construction Work"</h1>
                <p>"Discover Big Range of Components"</p>
            </Billboard>

            <Transition>
                <h2>{move || name.get()}</h2>
            </Transition>

            <ActionForm action={login}>
                <input
                    type="text"
                    name="data[username]"
                    placeholder="Enter your name"
                />
                <Button {..} type="submit">"Login"</Button>
            </ActionForm>

            // Show when action is pending
            {move || if login.pending().get() {
                view! { <p>"Logging in…"</p> }
            } else {
                view! { <p>"Ready"</p> }
            }}
        }.into_any()
    }
}
