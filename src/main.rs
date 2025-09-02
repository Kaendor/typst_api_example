use axum::{Router, routing::get};
use tracing::info;

mod routes;

use routes::pdf_generation_controller;
use typst_pdf_api::World;

#[tokio::main]
async fn main() {
    // Setup tracing subscriber for logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // build our application with a single route
    // let world = Arc::new(TypstWrapperWorld::new("examples".to_owned()));
    let world = World::new("examples".to_string());

    let app = Router::new()
        .route("/", get(pdf_generation_controller))
        .with_state(world);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
    info!("Server running on port 3000");
}
