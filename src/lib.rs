mod database;
mod routes;
mod utilities;

use sea_orm::Database;

use routes::create_routes;

pub async fn run(database_uri: &str) {
    let connection = Database::connect(database_uri).await.unwrap();
    let router = create_routes(connection);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
