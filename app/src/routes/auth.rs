use leptos::prelude::*;
use leptos_router::{lazy_route, LazyRoute};
use serde::{Deserialize, Serialize};
use crate::components::Button;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LoginData {
    username: String,
}

#[server]
async fn login(data: LoginData) -> Result<String, ServerFnError> {
    use axum::http::{HeaderValue, header::SET_COOKIE};
    use cookie::{Cookie, SameSite};
    use leptos_axum::ResponseOptions;

    let LoginData { username } = data;

    // Build cookie
    let cookie = Cookie::build(("user", username.clone()))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .build();

    let header_value = HeaderValue::from_str(&cookie.to_string())?;

    // Get the ResponseOptions injected by leptos_axum
    expect_context::<ResponseOptions>().append_header(SET_COOKIE, header_value);

    Ok(username)
}

#[server]
async fn load_username() -> Result<String, ServerFnError> {
    use axum::http::HeaderMap;
    use cookie::Cookie;
    use leptos_axum::extract;

    let headers = extract::<HeaderMap>().await?;

    let Some(cookie_header) = headers.get("cookie") else {
        return Err(ServerFnError::new("Cookie header not found"));
    };

    let Some(user) = Cookie::split_parse(cookie_header.to_str()?)
        .filter_map(Result::ok)
        .find(|c| c.name() == "user")
    else {
        return Err(ServerFnError::new("Error parsing cookie"));
    };

    Ok(user.value().into())
}

pub struct AuthPage {
    username: OnceResource<Result<String, ServerFnError>>,
    login_action: ServerAction<Login>
}

#[lazy_route]
impl LazyRoute for AuthPage {
    fn data() -> Self {
        Self {
            username: OnceResource::new(load_username()),
            login_action: ServerAction::<Login>::new()
        }
    }

    fn view(this: Self) -> AnyView {
        let AuthPage {
            username,
            login_action
        } = this;

        view! {
            <h2>"Auth"</h2>

            <Transition>
                <h2>
                    {move || if let Some(Ok(username)) = login_action.value().get() {
                        username
                    } else if let Some(Ok(username)) = username.get() {
                        username
                    } else {
                        "Guest".into()
                    }}
                </h2>
            </Transition>

            <ActionForm action={login_action}>
                <input
                    type="text"
                    name="data[username]"
                    placeholder="Enter your name"
                />
                <Button {..} type="submit">"Login"</Button>
            </ActionForm>

            <p>
                // Show when action is pending
                {move || if login_action.pending().get() {
                    "Logging in…"
                } else {
                    "Ready"
                }}
            </p>
        }.into_any()
    }
}
