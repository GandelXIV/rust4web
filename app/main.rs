pub mod model;
pub mod routes;
pub mod view;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = routes::build_routes().await.into_make_service();

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app)
        .await
        .unwrap();
}
