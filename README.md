Crypto Backend API (Rust + Axum)

A simple crypto-focused backend service built with Rust and Axum.
This project is part of my journey into backend development and API design, with a focus on clean structure, validation, and real-world patterns.

The service exposes basic endpoints for health checks and wallet address handling, and is designed to be extended with real blockchain logic later.

FEATURES

* Rust backend using Axum

* Simple and clean project structure

* Health check endpoint

* Wallet address endpoint with validation

* JSON responses

* Proper HTTP status codes

* Beginner-friendly and open to contributions

TECH STACK

* Rust

* Axum (web framework)

* Tokio (async runtime)

* Serde (JSON serialization)

GETTING STARTED
Prerequisites

* Rust (stable)

* Cargo

You can install Rust from:
https://www.rust-lang.org/tools/install

Run Locally

Clone the repository: "git clone https://github.com/cymorn/crypto-backend.git"
"cd crypto-backend"

Run the server: "cargo run"

The server will start at: http://127.0.0.1:3000

API Endpoints
Health Check

GET /health

Response: "OK"

Wallet Address Endpoint

GET /wallet/{address}

Example: "GET /wallet/0xABC123"

Successful response (200 OK):
{
  "address": "0xABC123",
  "valid": true
}

If the address is invalid, the API returns: "400 Bad Request"
(Note: Wallet validation is intentionally simple for now and will be expanded in future versions.)

Project Structure
src/
 └── main.rs
Cargo.toml
README.md
CONTRIBUTING.md

Roadmap

Planned improvements:

Stronger crypto wallet validation (Ethereum, etc.)

JSON error messages

Unit and integration tests

Better API structure

Optional database integration

Contributing

This is a learning-focused project and contributions are welcome.

If you’d like to help:

Improve validation

Suggest better API design

Refactor code

Add tests

Please see CONTRIBUTING.md for details.

Author

Amujo Simon Oluwagbeminiyi

GitHub: https://github.com/cymorn

Email: amujosimon01@gmail.com




