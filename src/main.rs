use axum::{routing::get, Router};
use maud::{html, Markup, DOCTYPE};
use tokio::net::TcpListener;

const HTMX_SOURCE: &str = "https://unpkg.com/htmx.org@1.9.12";
const HTMX_SHA: &str = "sha384-ujb1lZYygJmzgSwoxRggbCHcjc0rB2XoQrxeTUQyRjrOnlCoYta87iKBWq3EsdM2";

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                script src=(HTMX_SOURCE) integrity=(HTMX_SHA) crossorigin="anonymous";
            }
            body {
                h1 { "Hello" }
            }
        }
    }
}
