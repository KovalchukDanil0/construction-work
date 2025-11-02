use crate::components::{Button, Input, Link};
use convert_case::{Case, Casing};
use leptos::prelude::*;
use leptos_router::{LazyRoute, lazy_route};

#[server]
async fn contact_us(
    name: String,
    phone: String,
    email: String,
    company: String,
    postal: i32,
    place: String,
    street: String,
    house: String,
    subject: String,
    question: String,
) -> Result<(), ServerFnError> {
    println!("{}", postal);

    Ok(())
}

#[component]
fn Form(action: ServerAction<ContactUs>) -> impl IntoView {
    view! {
        <ActionForm action {..} class="flex flex-col gap-3 justify-center items-center">
            <div class="grid grid-cols-2 gap-4">
                <Input name="name" label="Name" required=true />
                <Input name="phone" label="Phone number" required=true r#type="tel" />
                <Input name="email" label="E-mail" required=true r#type="email" />
                <Input name="company" label="Company" />
                <Input
                    name="postal"
                    label="Postal code"
                    required=true
                    r#type="number"
                    pattern=r"\d{5}"
                />

                <Input name="place" label="Place" required=true />
                <Input name="street" label="Street" required=true />
                <Input name="house" label="House number" required=true />
                <Input name="subject" label="Subject" required=true class="col-span-2" />
                <Input name="question" label="Ask your question" required=true class="col-span-2" />
            </div>

            <Button {..} type="submit">
                "Submit"
            </Button>
        </ActionForm>
    }
}

pub struct ContactUsPage {
    action: ServerAction<ContactUs>,
    email: &'static str,
    phone: &'static str,
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
        let ContactUsPage {
            action,
            email,
            phone,
        } = this;

        view! {
            <h1 class="font-bold">"Contact Us"</h1>

            <p>
                "Contact us about anything related to our company or our services. We are doing our best to get back to you as soon as possible."
            </p>

            <div class="flex flex-col gap-3 justify-between items-center w-full md:flex-row">
                <Form action />

                <div class="flex flex-col gap-3 justify-start items-center">
                    <h2>"Construction Work"</h2>

                    <Link href={format!("mailto:{}", email)}>{email}</Link>
                    <Link href={format!(
                        "tel:{}",
                        phone.to_case(Case::Flat),
                    )}>"+420 776 412 521"</Link>
                </div>
            </div>
        }.into_any()
    }
}
