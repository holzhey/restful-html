use axum::{
    extract::State,
    routing::{get, post},
    Router,
};

use maud::{html, Markup, DOCTYPE};
use tracing::info;

use crate::AppState;

pub fn init(state: AppState) -> Router {
    Router::new()
        .route("/", get(base_handler))
        .route("/clicked", post(click_handler))
        .with_state(state)
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
            }
            body {
                h1 { "RESTful HTML" }
                div id="some-text" { "Some text" }
                button hx-post="/clicked" hx-target="#some-text" hx-swap="outerHTML" { "Click ME!" }
            }
        }
    }
}
