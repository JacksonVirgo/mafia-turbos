use axum::response::Html;
use maud::{Markup, html};

#[derive(Debug)]
pub struct WebPageBuilder {
    pub title: String,
    pub subtitle: Option<String>,
    pub body: Markup,
}

impl WebPageBuilder {
    pub fn new() -> Self {
        Self {
            title: "Ms.Walrus".into(),
            subtitle: None,
            body: html! {},
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn subtitle(mut self, subtitle: Option<impl Into<String>>) -> Self {
        self.subtitle = match subtitle {
            Some(s) => Some(s.into()),
            _ => None,
        };
        self
    }

    pub fn body(mut self, body: impl Into<Markup>) -> Self {
        self.body = body.into();
        self
    }

    pub fn build(self) -> Markup {
        let gen_title = match self.subtitle {
            Some(sub) => format!("{} | {}", sub, self.title),
            _ => self.title,
        };

        html! {
            head {
                meta charset="utf-8";
                title { (gen_title) }
                script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4" {}
                script src="https://unpkg.com/htmx.org@1.9.10" {}
                script src="https://cdn.jsdelivr.net/npm/htmx-ext-ws@2.0.2" {}

            }
            body."p-4" hx-ext="ws" {
               (self.body)
            }
        }
    }

    pub fn build_as_html(self) -> Html<String> {
        let html = self.build();
        Html(html.into_string())
    }
}
