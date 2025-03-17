#[tokio::main]
async fn main() {
    if let Err(e) = mural_server::run().await {
        eprintln!("error: {}", e);
    }
}
