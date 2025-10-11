use crate::components::{Billboard, Button};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::{LazyRoute, lazy_route};
use serde::{Deserialize, Serialize};
use serde_json::Number;

pub struct HomePage {
    username: OnceResource<Result<String, ServerFnError>>,
    billboards: OnceResource<Result<Vec<BillboardWithImage>, ServerFnError>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LoginData {
    username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Billboards {
    id: Number,
    title: String,
    description: String,
    img_id: Number,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BillboardWithImage {
    id: Number,
    title: String,
    description: String,
    img_id: Number,
    img_src: String,
    img_alt: String,
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
        return Err(ServerFnError::ServerError("cookie header not found".into()));
    };

    let Some(user) = Cookie::split_parse(cookie_header.to_str()?)
        .filter_map(Result::ok)
        .find(|c| c.name() == "user")
    else {
        return Err(ServerFnError::new(""));
    };

    Ok(user.value().into())
}

#[server]
async fn load_billboards() -> Result<Vec<BillboardWithImage>, ServerFnError> {
    use database::{get_database, get_resolved_image};

    let db = get_database()?;
    let billboards_json = db
        .select("billboards")
        .order("id", false)
        .execute()
        .await
        .map_err(|e| ServerFnError::new(&e))?;

    let billboards = billboards_json
        .into_iter()
        .filter_map(|bb| serde_json::from_value::<Billboards>(bb).ok())
        .collect::<Vec<_>>();

    let mut billboards_with_images = Vec::new();
    for bb in billboards {
        let (img_src, img_alt) = get_resolved_image(&bb.img_id.to_string()).await?;
        billboards_with_images.push(BillboardWithImage {
            id: bb.id,
            title: bb.title,
            description: bb.description,
            img_id: bb.img_id,
            img_src,
            img_alt,
        });
    }

    Ok(billboards_with_images)
}

#[lazy_route]
impl LazyRoute for HomePage {
    fn data() -> Self {
        Self {
            username: OnceResource::new(load_username()),
            billboards: OnceResource::new(load_billboards()),
        }
    }

    fn view(this: Self) -> AnyView {
        let HomePage {
            username,
            billboards,
        } = this;

        log::log!(
            log::Level::Debug,
            "{:?}",
            billboards.get().unwrap().unwrap()
        );

        let login = ServerAction::<Login>::new();

        view! {
            <Title text="Home Page"/>

            <Billboard src="/metal-construction.jpeg" alt="Construction Work" title="Construction Work" description="Discover Big Range of Components" />

            <Transition>
                <div class="flex flex-col gap-3 items-center justify-center">
                    <For each={move || billboards.get().unwrap_or(Ok(Vec::new())).unwrap_or_default()} key={|state| state.id.clone()} let(BillboardWithImage { id, img_id, title, description, img_src, img_alt })>
                        <Billboard src={img_src} alt={img_alt} title={title} description={description} />
                    </For>
                </div>
            </Transition>

            <Transition>
                <h2>
                    {move || if let Some(login) = login.value().get() {
                        login
                    } else if let Some(username) = username.get() {
                        username
                    } else {
                        Ok("Guest".into())
                    }}
                </h2>
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
        }
        .into_any()
    }
}
