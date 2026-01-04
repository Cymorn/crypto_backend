use axum::{
    routing::get,
    extract::Path,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/wallet/{address}", get(wallet_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}

async fn wallet_handler(Path(address): Path<String>) -> String {
    format!("Wallet address received: {}", address)
}
