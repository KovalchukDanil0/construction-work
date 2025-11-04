use crate::components::{Button, ButtonVariant, Input, Link, Logo};
use leptos::prelude::*;

#[server]
async fn newsletter_action(email: String) -> Result<bool, ServerFnError> {
    println!("{email}");

    Ok(true)
}

#[component]
pub fn Footer() -> impl IntoView {
    let newsletter_action = ServerAction::<NewsletterAction>::new();

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
                    <ActionForm
                        action={newsletter_action}
                        {..}
                        class="flex flex-row gap-3 justify-center items-center"
                    >
                        <Input
                            class="flex-shrink w-full"
                            name="email"
                            placeholder="youremail@example.com"
                            required=true
                            {..}
                            type="email"
                        />
                        <Button variant={ButtonVariant::Black}>"Register"</Button>

                        <ShowLet
                            some={move || newsletter_action.value().get().and_then(Result::ok)}
                            let:newsletter_value
                        >
                            <p>{if newsletter_value { "Email sent!" } else { "Error!" }}</p>
                        </ShowLet>
                    </ActionForm>
                </div>
            </div>

            <div class="flex flex-row gap-3 justify-around items-center">
                <p>"Copyright © 2025"</p>
                <p>"Powered by Rust Leptos"</p>
            </div>
        </div>
    }
}
