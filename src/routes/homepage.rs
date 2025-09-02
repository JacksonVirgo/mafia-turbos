use crate::builders::webpage::WebPageBuilder;
use axum::response::Html;
use maud::html;

pub async fn homepage_root() -> Html<String> {
    WebPageBuilder::new()
        .title("Mafia Turbos")
        .body(html! {
            h1."text-2xl bold underline"{ "Mafia Turbos "}
            h2 { "Subheader"}
            hr {}
            button hx-get="/login_form" {
                "Log In"
            }
        })
        .build_as_html()
}
