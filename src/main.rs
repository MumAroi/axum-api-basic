#![allow(unused)]
use axum::{
    Router,
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::{get, get_service},
};
use serde::Deserialize;
use tower_http::services::ServeDir;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let routes_all = Router::new().merge(route_hello());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("->> LISTENING ON {} \n", addr);
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
}

fn route_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

fn route_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("-> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello <strong>{name}!!</strong>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("-> {:<12} - handler_hello2 - {name:?}", "HANDLER");

    Html(format!("Hello <strong>{name}!!</strong>"))
}
