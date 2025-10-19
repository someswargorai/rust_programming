mod routes;
mod controller;
mod middleware;

use std::net::SocketAddr;

use axum::{
    Router,
};

#[tokio::main]
async fn main() {

    let app: Router = routes::user_routes::create_app_routes();
    let listener = tokio::net::TcpListener::bind(SocketAddr::from(([127,0,0,1],3000))).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    
}