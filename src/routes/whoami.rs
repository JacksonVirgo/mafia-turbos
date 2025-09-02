use crate::builders::webpage::WebPageBuilder;
use axum::response::Html;
use axum_extra::extract::CookieJar;
use maud::html;

pub async fn who_am_i_root(jar: CookieJar) -> Html<String> {
    let username = jar.get("username");

    let username_str = match username {
        Some(name) => {
            format!("Username: {}", name.value())
        }
        _ => "Not Logged In".to_string(),
    };

    WebPageBuilder::new()
        .title("Mafia Turbos")
        .subtitle(Some("Who"))
        .body(html! {
            h1."text-2xl bold underline"{ "Mafia Turbos "}
            h2 { "Session Information"}
            ul."list-disc list-inside" {
                li {
                    (username_str)
                }
            }

        })
        .build_as_html()
}
