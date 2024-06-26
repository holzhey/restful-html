use axum::{
    extract::State,
    routing::{get, post},
    Router,
};

use maud::{html, Markup, Render, DOCTYPE};
use tracing::info;

use crate::AppState;

struct Css(&'static str);

impl Render for Css {
    fn render(&self) -> Markup {
        html! {
            link rel="stylesheet" type="text/css" href=(self.0);
        }
    }
}

pub fn init(state: AppState) -> Router {
    Router::new()
        .route("/", get(base_handler))
        .route("/clicked", post(click_handler))
        .route("/style.css", get(style_handler))
        .with_state(state)
}

#[tracing::instrument]
pub async fn style_handler() -> String {
    info!("style");
    "background-color: #ff0000;".to_string()
}

#[tracing::instrument]
pub async fn click_handler() -> Markup {
    info!("click");
    html! {
        "New text after click"
    }
}

#[tracing::instrument]
pub async fn base_handler(State(state): State<AppState>) -> Markup {
    info!("base");
    html! {
        (DOCTYPE)
        html lang="en-US" {
            head {
                meta charset="utf-8";
                script src=(state.config.htmx.source) integrity=(state.config.htmx.sha) crossorigin="anonymous" {}
                title { "RUST-HTMX Sandbox" }
                (Css("style.css"))
            }
            body {
                h1 { "RESTful HTML" }
                div id="some-text" { "Some text" }
                button hx-post="/clicked" hx-target="#some-text" hx-swap="innerHTML" { "Click ME!" }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        config::Htmx,
        routes::{base_handler, click_handler},
        AppState,
    };

    #[tokio::test]
    async fn click_handler_test() {
        let v: String = click_handler().await.into();

        assert_eq!(v, "New text after click");
    }

    #[tokio::test]
    async fn base_handler_test() {
        let v: String = base_handler(axum::extract::State(create_app_state()))
            .await
            .into();

        assert_eq!(v.len(), 382);
    }

    fn create_app_state() -> AppState {
        AppState {
            config: crate::config::Config {
                address: "".to_string(),
                htmx: Htmx {
                    source: "".to_string(),
                    sha: "".to_string(),
                },
            },
        }
    }
}
