use leptos::prelude::*;
use crate::components::{Link, Input, Button, Logo};

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-6 justify-center py-6">
            <div class="flex flex-row gap-3 items-center justify-around">
                <Logo />

                <div class="flex flex-col gap-3">
                    <p class="font-bold">"Customer service"</p>

                    <Link href="about">"About us"</Link>
                    <Link href="contact-us">"Contact us"</Link>
                </div>

                <div class="flex flex-col gap-3">
                    <p class="font-bold">"Newsletter"</p>

                    <p>"Receive the latest updates, news and offers via email"</p>
                    <div class="flex flex-row gap-3">
                        <Input name="data[newsletter]" placeholder="youremail@example.com" />
                        <Button>"Register"</Button>
                    </div>
                </div>
            </div>

            <div class="flex flex-row gap-3 items-center justify-around">
                <p>"Copyright © 2025"</p>
                <p>"Powered by Rust Leptos"</p>
            </div>
        </div>
    }
}
