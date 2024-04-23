use axum::{
    extract::State,
    routing::{get, post},
    Router,
};

use config::Config;
use maud::{html, Markup, DOCTYPE};
use tokio::net::TcpListener;

pub mod config;

#[derive(Clone)]
pub struct AppState {
    config: Config,
}

#[tokio::main]
async fn main() {
    let cfg = config::init();
    let address = cfg.address.clone();
    let state = AppState { config: cfg };
    let app = Router::new()
        .route("/", get(base_handler))
        .route("/clicked", post(click_handler))
        .with_state(state);
    let listener = TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn click_handler() -> Markup {
    html! {
        "New text after click"
    }
}

async fn base_handler(State(state): State<AppState>) -> Markup {
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
