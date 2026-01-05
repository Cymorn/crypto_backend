use axum::{
    routing::get,
    extract::Path,
    response::Json,
    http::StatusCode,
    Router,
};
use serde::Serialize;

#[derive(Serialize)]
struct WalletResponse {
    address: String,
    valid: bool,
}

fn is_valid_wallet(address: &str) -> bool {
    address.len() >= 5
}

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

async fn wallet_handler(
    Path(address): Path<String>,
) -> Result<Json<WalletResponse>, StatusCode> {
    if !is_valid_wallet(&address) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let response = WalletResponse {
        address,
        valid: true,
    };

    Ok(Json(response))
}
