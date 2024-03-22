use std::net::SocketAddr;

pub mod env;
pub mod model;
pub mod routes;
pub mod views;

#[tokio::main]
async fn main() {
    println!("ENV_DEBUG = {}", env::ENV_DEBUG.as_str());

    // build our application with a single route
    let app = routes::build_routes().await.into_make_service();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
