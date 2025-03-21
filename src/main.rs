use std::process::ExitCode;

fn main() -> ExitCode {
	tracing_subscriber::fmt::init();

	if let Err(e) = mural_server::run() {
		eprintln!("error: {}", e);
		return ExitCode::from(1);
	}

	return ExitCode::SUCCESS;
}
