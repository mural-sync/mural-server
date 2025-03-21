#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to parse the .env file: '{line_content}' on line {line_number} is invalid")]
    DotenvyLineParse {
        line_content: String,
        line_number: usize,
    },
    #[error("failed to read the .env file: {0}")]
    DotenvyIo(std::io::Error),

    #[error("{0}")]
    Custom(String),
}
