use crate::components::code_preview::CodePreview;
use crate::components::seo_title::SeoTitle;
use hirola::prelude::*;

pub fn forms_page(_app: &HirolaApp) -> Dom {
    html! {
        <div>
      <SeoTitle title={"Forms | Hirola"} />

            <h1>"Forms"</h1>
            <p>"Hirola is un-opinionated in form management. It should be pretty easy to roll out your own. To enable the inbuilt form management use the feature flag "<code class="one-liner">"form"</code></p>
            <h2>"Getting started"</h2>
            <CodePreview
             code=
r#"struct Login {
    #[validate(length(min = 1, message = "Email is required"))]
    email: String,
    #[validate(length(min = 1, message = "Password is required"))]
    password: String
}

fn form_demo(_app: &HirolaApp) -> Dom {
    let form = FormHandler::new(Login {
        email: String::from_str("example@gmail.com").unwrap(),
        password: String::new(),
    });
    html! {
        <form
            ref=&form.node_ref()
            >
            <div class="mb-6">
                <label for="email">
                     "Your email"
                </label>
                <input
                    type="email"
                    id="email"
                    name="email"
                    placeholder="name@example.com"
                    mixin:form={&form.register::<HtmlInputElement>()}
                    />
                <span mixin:text=&text(&form.error_for("email"))></span>
            </div>
            ......

}
"#
            file="src/main.rs"
            />
        </div>
    }
}
