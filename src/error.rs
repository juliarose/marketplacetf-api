/// An error occurred while interacting with the marketplace.tf API.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An invalid parameter was passed to the API.
    #[error("Invalid parameter: {}", .0)]
    Parameter(&'static str),
    /// An unexpected response was received from the API.
    #[error("Unexpected response: {}", .0)]
    Response(String),
    /// An error occurred while making a request to the API.
    #[error("Request error: {}", .0)]
    Reqwest(#[from] reqwest::Error),
    /// An error occurred while processing a request to the API.
    #[error("Request middleware error: {}", .0)]
    ReqwestMiddleware(anyhow::Error),
    /// An error occurred while parsing a response from the API.
    #[error("Error parsing response: {}", .0)]
    Parse(#[from] serde_json::Error),
    /// An HTTP error occurred while interacting with the API.
    #[error("{}", .0)]
    Http(reqwest::StatusCode),
}

impl Error {
    /// An unrecoverable error occurred.
    pub fn is_fatal(&self) -> bool {
        match self {
            Error::Parameter(_) => true,
            Error::Response(_) => true,
            Error::Reqwest(_) => true,
            Error::ReqwestMiddleware(_) => true,
            Error::Parse(_) => true,
            Error::Http(status_code) => !status_code.is_server_error(),
        }
    }
}
