use axum::{extract::Query, response::Html, routing::get, Router, self};
use std::net::{SocketAddr};
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
struct Parameters {
    first: i32,
    second: i32,
}

async fn add_handler(Query(nums): Query<Parameters>) -> Html<String> {
    let result = nums.first + nums.second;

    Html(format!("<h1>Addition result: {}</h1>", result))
}

async fn sub_handler(Query(nums): Query<Parameters>) -> Html<String> {
    let result = nums.first - nums.second;

    Html(format!("<h1>Subtract result: {}</h1>", result))
}

async fn mul_handler(Query(nums): Query<Parameters>) -> Html<String> {
    let result = nums.first * nums.second;

    Html(format!("<h1>Multiply result: {}</h1>", result))
}

async fn div_handler(Query(nums): Query<Parameters>) -> Html<String> {
    let result = nums.first / nums.second;

    Html(format!("<h1>Division result: {}</h1>", result))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .route("/add", get(add_handler))
        .route("/sub", get(sub_handler))
        .route("/mul", get(mul_handler))
        .route("/div", get(div_handler));

    let addr = SocketAddr::from(([127,0,0,1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    let _ = axum::serve(listener, app).await.unwrap();  
}

async fn handler() -> &'static str {
    "Hello, World!"
}
