use config::Config;

use routes::init;
use tokio::net::TcpListener;

pub mod config;
pub mod routes;

#[derive(Clone)]
pub struct AppState {
    config: Config,
}

#[tokio::main]
async fn main() {
    let cfg = config::init();
    let address = cfg.address.clone();
    let state = AppState { config: cfg };
    let app = init(state);
    let listener = TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
