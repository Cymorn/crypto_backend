use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new() //ceates a new empty router
        .route("/health", get(health_check)); //adds a route
    let listener =
    tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap(); //starts the HTTP server

    async fn health_check() -> &'static str {"Ok"}
}