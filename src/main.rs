use axum::{
    routing::{get, post},
    Router,
};
use maud::{html, Markup, DOCTYPE};
use tokio::net::TcpListener;

const HTMX_SOURCE: &str = "https://unpkg.com/htmx.org@1.9.12";
const HTMX_SHA: &str = "sha384-ujb1lZYygJmzgSwoxRggbCHcjc0rB2XoQrxeTUQyRjrOnlCoYta87iKBWq3EsdM2";

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(base_handler))
        .route("/clicked", post(click_handler));
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
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
