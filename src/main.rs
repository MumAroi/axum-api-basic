#![allow(unused)]
use axum::{Router, response::Html, routing::get};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let route_hello = Router::new().route(
        "/hello",
        get(|| async { Html("Hello <strong>Wrold!!</strong>") }),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("->> LISTENING ON {} \n", addr);
    axum::Server::bind(&addr)
        .serve(route_hello.into_make_service())
        .await
        .unwrap();
}
