use axum::{Form, http::StatusCode, response::Html};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use maud::html;
use serde::Deserialize;

pub async fn login_form(jar: CookieJar) -> Html<String> {
    if let Some(cookie) = jar.get("username") {
        let markdown = html! {
            div {
                h3."text-xl" { "Welcome back, " (cookie.value()) }
            }
        };
        Html(markdown.into_string())
    } else {
        let markdown = html! {
            div."border border-2 border-black p-4 rounded-lg" {
                h2."bold underline text-lg" { "Log in as a guest!"}
                form hx-post="/signup" hx-swap="innerHTML" ."flex flex-col gap-2" {
                    label for="name" { "Username:"}
                    input id="name" name="name" type="text" required ."border border-black" {}
                    button type="submit" ." border border-black hover:bg-zinc-200 hover:cursor-pointer" { "Signup as Guest"}
                }
            }
        };

        Html(markdown.into_string())
    }
}

#[derive(Deserialize)]
pub struct GuestLoginQuery {
    name: String,
}

pub async fn login_form_submission(
    jar: CookieJar,
    Form(form): Form<GuestLoginQuery>,
) -> (CookieJar, (StatusCode, Html<String>)) {
    let mut c = Cookie::new("username", form.name.clone());
    c.set_path("/");
    c.set_http_only(true);
    c.set_secure(true);

    let form = format!("<div>Logged in as {}</div>", form.name);

    let updated_jar = jar.add(c);
    (updated_jar, (StatusCode::ACCEPTED, Html(form)))
}
