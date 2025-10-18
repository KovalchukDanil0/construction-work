use crate::components::{Button, Input, Link};
use leptos::prelude::*;
use leptos_router::{LazyRoute, lazy_route};
use serde::{Deserialize, Serialize};
use convert_case::{Case, Casing};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ContactUsData {
    name: String,
    phone: String,
    email: String,
    company: String,
    postal: String,
    place: String,
    street: String,
    house: String,
    subject: String,
    question: String,
}

#[server]
async fn contact_us(data: ContactUsData) -> Result<(), ServerFnError> {
    let ContactUsData {
        name,
        phone,
        email,
        company,
        postal,
        place,
        street,
        house,
        subject,
        question,
    } = data;

    Ok(())
}

#[component]
fn Form(action: ServerAction<ContactUs>) -> impl IntoView {
    view! {
        <ActionForm action {..} class="w-1/2 flex flex-col gap-3 items-center justify-center">
            <div class="grid grid-cols-2 gap-4">
                <Input name="data[name]" label="Name" />
                <Input name="data[phone]" label="Phone number" />
                <Input name="data[email]" label="E-mail" />
                <Input name="data[company]" label="Company" />
                <Input name="data[postal]" label="Postal code" />
                <Input name="data[place]" label="Place" />
                <Input name="data[street]" label="Street" />
                <Input name="data[house]" label="House number" />
                <Input class="col-span-2" name="data[subject]" label="Subject" />
                <Input class="col-span-2" name="data[question]" label="Ask your question" />
            </div>

            <Button {..} type="submit">"Submit"</Button>
        </ActionForm>
    }
}

pub struct ContactUsPage {
    action: ServerAction<ContactUs>,
    email: &'static str,
    phone: &'static str
}

#[lazy_route]
impl LazyRoute for ContactUsPage {
    fn data() -> Self {
        Self {
            action: ServerAction::<ContactUs>::new(),
            email: "keretski94@gmail.com",
            phone: "+420 776 412 521",
        }
    }

    fn view(this: Self) -> AnyView {
        let ContactUsPage { action, email, phone } = this;

        view! {
            <h1 class="font-bold">"Contact Us"</h1>

            <p>"Contact us about anything related to our company or our services. We are doing our best to get back to you as soon as possible."</p>

            <div class="flex flex-row gap-3 items-center justify-between w-full">
                <Form action />

                <div class="w-1/2 flex flex-col gap-3 items-center justify-start">
                    <h2>"Construction Work"</h2>

                    <Link href={format!("mailto:{}", email)}>{email}</Link>
                    <Link href={format!("tel:{}", phone.to_case(Case::Flat))}>"+420 776 412 521"</Link>
                </div>
            </div>
        }.into_any()
    }
}
