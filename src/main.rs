use fullsendwaxing_server;

/// <summary> 
///  Main entry point into our app
/// </summary>
#[tokio::main]
async fn main() {
    fullsendwaxing_server::run().await; // no unwrap needed -> no result is returned
}
