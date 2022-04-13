
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid parameter: {}", .0)]
    Parameter(&'static str),
    #[error("Unexpected response: {}", .0)]
    Response(String),
    #[error("Request error: {}", .0)]
    Reqwest(#[from] reqwest::Error),
    #[error("Request middleware error: {}", .0)]
    ReqwestMiddleware(anyhow::Error),
    #[error("Error parsing response: {}", .0)]
    Parse(#[from] serde_json::Error),
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

impl From<reqwest_middleware::Error> for Error {
    fn from(error: reqwest_middleware::Error) -> Error {
        match error {
            reqwest_middleware::Error::Reqwest(e) => {
                Error::Reqwest(e)
            },
            reqwest_middleware::Error::Middleware(e) => {
                Error::ReqwestMiddleware(e)
            },
        }
    }
}