use crate::components::{
    Billboard,
    card::{CardBanner, CardFeature},
};
use icondata::{
    FaBoxesStackedSolid, FaCalendarDaysSolid, FaHeadsetSolid, FaLocationDotSolid,
    FaPeopleGroupSolid, FaWarehouseSolid,
};
use leptos::prelude::*;
use leptos_icons::Icon;
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
        let HomePage { billboards } = this;

        view! {
			<Title text="Home Page" />

			<div class="grid grid-cols-1 grid-rows-3 gap-10 m-0 sm:grid-cols-2 sm:grid-rows-3 md:grid-cols-3 md:grid-rows-2 md:m-10">
				<CardBanner img="/metal-construction.jpeg" alt="" title="Test" />
				<CardBanner img="/steel-building.jpg" alt="" title="Test" />
				<CardBanner img="/metal-construction.jpeg" alt="" title="Test" />
				<CardBanner img="/metal-construction.jpeg" alt="" title="Test" />
				<CardBanner
					img="/steel-building.jpg"
					alt="Steel Building"
					title="Title"
					description="Description"
					button={("cart", "Cart")}
				/>
				<CardBanner img="/metal-construction.jpeg" alt="" title="Test" />
			</div>

			<div class="flex flex-col gap-10 justify-center items-center">
				<h1>"Construction Work"</h1>

				<div class="flex flex-col gap-10 justify-between items-center md:flex-row">
					<Icon icon={FaLocationDotSolid} {..} class="size-full" />

					<div class="flex flex-col flex-shrink gap-3 justify-center items-start">
						<p>
							"Welcome to the official web-shop of Construct – where innovation, quality, and service come together to optimize your work!"
						</p>

						<p>
							"Our Construction Work website is specially developed for customers in Belgium, the Netherlands, Luxembourg, France, and Germany. Are you located in one of these countries? Then you can order directly through this site and benefit from a smooth processing, fast delivery, and excellent service."
						</p>
						<p>
							"Do you live outside this region? No worries! Please feel free to contact our experienced and multilingual team. We are happy to work with you to find the best way to process your order smoothly and efficiently."
						</p>
						<p>
							"At Van Maele Benelux, we are ready to support you – wherever you are."
						</p>
					</div>
				</div>

				<div class="flex flex-row gap-3 justify-center items-center">
					<CardFeature icon={FaCalendarDaysSolid}>
						<p>"Almost 30 years of experience, expertise and advice"</p>
					</CardFeature>

					<CardFeature icon={FaBoxesStackedSolid}>
						<p>"300 items on our e‑shop"</p>
					</CardFeature>

					<CardFeature icon={FaHeadsetSolid}>
						<p>"Multilingual customer service available"</p>
					</CardFeature>

					<CardFeature icon={FaWarehouseSolid}>
						<p>"Warehouse of 5000 m² and Showroom of 750 m²"</p>
					</CardFeature>

					<CardFeature icon={FaPeopleGroupSolid}>
						<p>"More than 1500 active customers"</p>
					</CardFeature>
				</div>
			</div>

			<Billboard
				src="/metal-construction.jpeg"
				alt="Construction Work"
				title="Construction Work"
				description="Discover Big Range of Components"
			/>

			<Transition>
				<ShowLet some={move || billboards.get().and_then(Result::ok)} let:billboards>
					<div class="flex flex-col gap-3 justify-center items-center w-full">
						<For
							each={move || billboards.clone()}
							key={|state| state.id.clone()}
							let(BillboardWithImage { id: _, title, description, img_src, img_alt })
						>
							<Billboard src={img_src} alt={img_alt} title description />
						</For>
					</div>
				</ShowLet>
			</Transition>
		}.into_any()
    }
}
