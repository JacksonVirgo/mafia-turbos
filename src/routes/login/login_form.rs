use axum::{
    Form,
    http::{HeaderMap, HeaderName, HeaderValue, StatusCode},
    response::{Html, IntoResponse},
};
use axum_extra::extract::CookieJar;
use maud::html;
use serde::Deserialize;

pub async fn login_form(_: CookieJar) -> Html<String> {
    let markdown = html! {
        div."border border-2 border-black p-4 rounded-lg" {
            h2."bold underline text-lg" { "Log in as a guest!"}
            form hx-post="/signup" hx-swap="innerHTML" ."flex flex-col gap-2" {
                label for="name" { "Username:"}
                input id="name" name="name" type="text" required ."border border-black" {}

                label for="code" {"Room Code:"}
                input id="code" name="code" type="text" required ."border border-black" {}

                button type="submit" ." border border-black hover:bg-zinc-200 hover:cursor-pointer" { "Signup as Guest"}
            }
        }
    };

    Html(markdown.into_string())
}

#[derive(Deserialize)]
pub struct GuestLoginQuery {
    name: String,
    code: String,
}

// pub async fn login_form_submission(Form(form): Form<GuestLoginQuery>) -> impl IntoResponse {
//     let mut headers = HeaderMap::new();
//     let redirect_url = format!("/chatroom?username={}&code={}", form.name, form.code);
//     headers.insert("HX-Redirect", redirect_url.parse().unwrap());
//     let body = Html(format!("<div>Logged in as {}</div>", form.name));
//     (headers, StatusCode::OK, body)
// }

pub async fn login_form_submission(Form(form): Form<GuestLoginQuery>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    let redirect_url = format!("/chatroom?username={}&code={}", form.name, form.code);

    headers.insert(
        HeaderName::from_static("hx-redirect"),
        HeaderValue::from_str(redirect_url.as_str()).unwrap(),
    );

    (
        StatusCode::OK,
        headers,
        Html(String::from("<div>Loading</div>")),
    )
}
