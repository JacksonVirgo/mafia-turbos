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
            button hx-get="/user_info" {
                "Get User"
            }
            hr {}
            form."border border-black" hx-post="/create_user" hx-swap="outerHTML" {
                button type="submit" {
                    "Create User"
                }
            }
        })
        .build_as_html()
}

pub async fn get_user() -> Html<String> {
    let markdown = html! {
        div {
            "User Information: N/A"
        }
    };
    Html(markdown.into())
}

pub async fn create_user() -> Html<String> {
    let markdown = html! {
        div {
            "User has been created (but not really)"
        }
    };
    Html(markdown.into())
}
