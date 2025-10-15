use crate::components::{Button, Input};
use leptos::prelude::*;
use leptos_router::{LazyRoute, lazy_route};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AddTodoData {
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
async fn add_todo(data: AddTodoData) -> Result<(), ServerFnError> {
    let AddTodoData {
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

    println!("{}", name);
    Ok(())
}

pub struct ContactUsPage {
    action: ServerAction<AddTodo>,
}

#[lazy_route]
impl LazyRoute for ContactUsPage {
    fn data() -> Self {
        Self {
            action: ServerAction::<AddTodo>::new(),
        }
    }

    fn view(this: Self) -> AnyView {
        let ContactUsPage { action } = this;

        view! {
            <h1 class="font-bold">"Contact Us"</h1>

            <p>"Contact us about anything related to our company or our services. We are doing our best to get back to you as soon as possible."</p>

            <ActionForm action {..} class="w-1/2">
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
        .into_any()
    }
}
