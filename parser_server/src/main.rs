use axum::{
    extract::Path,
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::Serialize;
use tokio::net::TcpListener;
use urlencoding::decode;

use evaluator_core::{evaluate_script, response_types::Response};

#[derive(Serialize)]
struct ParseResponse {
    code: String,
    response: Option<Response>, // assuming Response implements Serialize
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/parse/:code", get(parse_code));

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Listening on http://0.0.0.0:8000");
    axum::serve(listener, app).await.unwrap();
}

#[axum::debug_handler] // <-- optional, helps with debugging this exact error
async fn parse_code(Path(code): Path<String>) -> (StatusCode, Json<ParseResponse>) {
    match decode(&code) {
        Ok(decoded) => {
            let res = evaluate_script(&decoded).await;
            let response = ParseResponse {
                code: decoded.to_string(),
                response: res,
            };
            (StatusCode::OK, Json(response))
        }
        Err(_) => (
            StatusCode::BAD_REQUEST,
            Json(ParseResponse {
                code,
                response: None,
            }),
        ),
    }
}
