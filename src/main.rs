use dotenvy::dotenv;
use dotenvy_macro::dotenv;

use fullsendwaxing_server;

/// <summary>
///  Main entry point into our app
/// </summary>
#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_uri = dotenv!("DATABASE_URL");
    println!("{database_uri}");

    fullsendwaxing_server::run(database_uri).await; // no unwrap needed -> no result is returned
}
