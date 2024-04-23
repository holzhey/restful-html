use config::Config;

use routes::init;
use tokio::net::TcpListener;
use tracing::{info, Level};

pub mod config;
pub mod routes;

#[derive(Clone, Debug)]
pub struct AppState {
    config: Config,
}

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
    let cfg = config::init();
    let address = cfg.address.clone();
    let state = AppState { config: cfg };
    let app = init(state);
    let listener = TcpListener::bind(address).await.unwrap();
    info!("Starting server");
    axum::serve(listener, app).await.unwrap();
}
