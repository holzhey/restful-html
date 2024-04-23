use axum::{
    routing::{get, post},
    Router,
};
use maud::{html, Markup, DOCTYPE};
use serde::Deserialize;
use tokio::net::TcpListener;

const HTMX_SOURCE: &str = "https://unpkg.com/htmx.org@1.9.12";
const HTMX_SHA: &str = "sha384-ujb1lZYygJmzgSwoxRggbCHcjc0rB2XoQrxeTUQyRjrOnlCoYta87iKBWq3EsdM2";

#[derive(Deserialize)]
struct Config {
    address: String,
}

#[derive(Deserialize)]
struct Htmx {
    source: String,
    sha: String,
}

#[tokio::main]
async fn main() {
    let config: Config = toml::from_str(
        r#"
        address='127.0.0.1:3000'

        [htmx]
        source='https://unpkg.com/htmx.org@1.9.12'
        sha='sha384-ujb1lZYygJmzgSwoxRggbCHcjc0rB2XoQrxeTUQyRjrOnlCoYta87iKBWq3EsdM2'
        "#,
    )
    .unwrap();
    let app = Router::new()
        .route("/", get(base_handler))
        .route("/clicked", post(click_handler));
    let listener = TcpListener::bind(config.address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn click_handler() -> Markup {
    html! {
        "New text after click"
    }
}

async fn base_handler() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en-US" {
            head {
                meta charset="utf-8";
                script src=(HTMX_SOURCE) integrity=(HTMX_SHA) crossorigin="anonymous" {}
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
