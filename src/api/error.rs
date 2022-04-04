use thiserror::Error;

// pub const RESPONSE_UNSUCCESSFUL_MESSAGE: &str = "Empty response";

#[derive(Debug, Error)]
pub enum APIError {
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

impl APIError {
    
    /// An unrecoverable error occurred.
    pub fn is_fatal(&self) -> bool {
        match self {
            APIError::Parameter(_) => true,
            APIError::Response(_) => true,
            APIError::Reqwest(_) => true,
            APIError::ReqwestMiddleware(_) => true,
            APIError::Parse(_) => true,
            APIError::Http(status_code) => !status_code.is_server_error(),
        }
    }
}

impl From<reqwest_middleware::Error> for APIError {
    fn from(error: reqwest_middleware::Error) -> APIError {
        match error {
            reqwest_middleware::Error::Reqwest(e) => {
                APIError::Reqwest(e)
            },
            reqwest_middleware::Error::Middleware(e) => {
                APIError::ReqwestMiddleware(e)
            },
        }
    }
}