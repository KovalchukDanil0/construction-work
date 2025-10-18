use crate::components::Billboard;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::{LazyRoute, lazy_route};
use serde::{Deserialize, Serialize};
use serde_json::Number;

pub struct HomePage {
    billboards: OnceResource<Result<Vec<BillboardWithImage>, ServerFnError>>,
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
    img_src: String,
    img_alt: String,
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
            billboards: OnceResource::new(load_billboards()),
        }
    }

    fn view(this: Self) -> AnyView {
        let HomePage {
            billboards,
        } = this;

        view! {
            <Title text="Home Page"/>

            <Billboard src="/metal-construction.jpeg" alt="Construction Work" title="Construction Work" description="Discover Big Range of Components" />

            <div class="flex flex-col gap-3 items-center justify-center w-full">
                <Transition>
                    <For
                        each={move || billboards.get().unwrap_or(Ok(Vec::new())).unwrap_or_default()}
                        key={|state| state.id.clone()}
                        let(BillboardWithImage { id: _, title, description, img_src, img_alt })
                    >
                        <Billboard src={img_src} alt={img_alt} title={title} description={description} />
                    </For>
                </Transition>
            </div>
        }.into_any()
    }
}
