use crate::components::{Button, ButtonVariant, Input, Link, Logo};
use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-6 justify-center py-6">
            <div class="flex flex-row flex-wrap gap-3 justify-between items-center text-center">
                <Logo class="flex-grow w-full md:w-auto" />

                <div class="flex flex-col flex-1 gap-3">
                    <p class="font-bold">"Customer service"</p>

                    <Link href="about">"About us"</Link>
                    <Link href="contact-us">"Contact us"</Link>
                </div>

                <div class="flex flex-col flex-grow gap-3">
                    <p class="font-bold">"Newsletter"</p>

                    <p>"Receive the latest updates, news and offers via email"</p>
                    <div class="flex flex-row gap-3 justify-center items-center">
                        <Input
                            class="flex-shrink w-full"
                            name="data[newsletter]"
                            placeholder="youremail@example.com"
                        />
                        <Button variant={ButtonVariant::Black}>"Register"</Button>
                    </div>
                </div>
            </div>

            <div class="flex flex-row gap-3 justify-around items-center">
                <p>"Copyright © 2025"</p>
                <p>"Powered by Rust Leptos"</p>
            </div>
        </div>
    }
}
