use crate::error::Error;
use std::sync::Arc;
use reqwest::header;
use reqwest::cookie::CookieStore;
use reqwest::{Client, ClientBuilder};
use serde::Deserialize;
use serde::de::DeserializeOwned;

pub fn get_default_client<T>(cookie_store: Arc<T>, user_agent_string: &'static str) -> Client
where
    T: CookieStore + 'static
{
    let mut headers = header::HeaderMap::new();
    
    headers.insert(header::USER_AGENT, header::HeaderValue::from_static(user_agent_string));
    
    ClientBuilder::new()
        .cookie_provider(cookie_store)
        .default_headers(headers)
        .build()
        .unwrap()
}

pub async fn parses_response<D>(response: reqwest::Response) -> Result<D, Error>
where
    D: DeserializeOwned
{
    #[derive(Deserialize, Debug)]
    struct ErrorResponse {
        error: String,
    }
    
    let status = &response.status();
    
    match status.as_u16() {
        300..=399 => {
            Err(Error::Http(*status))
        },
        400..=499 => {
            Err(Error::Http(*status))
        },
        500..=599 => {
            Err(Error::Http(*status))
        },
        _ => {
            let body = &response
                .bytes()
                .await?;

            match serde_json::from_slice::<D>(body) {
                Ok(body) => Ok(body),
                Err(parse_error) => {
                    // unexpected response
                    if let Ok(error_body) = serde_json::from_slice::<ErrorResponse>(body) { 
                        Err(Error::Response(error_body.error))
                    } else {
                        Err(parse_error.into())
                    }
                }
            }
        }
    }
}