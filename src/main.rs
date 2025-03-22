use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    tracing_subscriber::fmt::init();

    if let Err(e) = mural_server::run().await {
        eprintln!("error: {}", e);
        return ExitCode::from(1);
    }

    return ExitCode::SUCCESS;
}
