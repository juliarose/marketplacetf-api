use super::helpers;
use super::api_response;
use crate::response::{User, Bot, DashboardDetails, Sale};
use crate::error::Error;
use crate::SteamID;
use std::sync::Arc;
use serde::Serialize;
use reqwest::cookie::Jar;
use reqwest_middleware::ClientWithMiddleware;

const USER_AGENT_STRING: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.71 Safari/537.36";

/// API for interacting with marketplace.tf
#[derive(Debug)]
pub struct MarketplaceAPI {
    /// The API key.
    key: String,
    /// The HTTP client.
    client: ClientWithMiddleware,
}

impl MarketplaceAPI {
    const HOSTNAME: &'static str = "marketplace.tf";
    
    /// Creates a new instance of the API.
    pub fn new(key: &str) -> Self {
        let cookies = Arc::new(Jar::default());
        
        Self {
            key: key.into(),
            client: helpers::get_default_middleware(Arc::clone(&cookies), USER_AGENT_STRING),
        }
    }
    
    /// Gets the URI for a given pathname.
    fn uri(&self, pathname: &str) -> String {
        format!("https://{}{}", Self::HOSTNAME, pathname)
    }
    
    /// Gets the URI for the seller API.
    fn seller_api_uri(&self, endpoint: &str) -> String {
        format!("{}{}", self.uri("/api/Seller"), endpoint)
    }
    
    /// Gets the URI for the bots API.
    fn bots_api_uri(&self, endpoint: &str) -> String {
        format!("{}{}", self.uri("/api/Bots"), endpoint)
    }
    
    /// Gets the URI for the bans API.
    fn bans_api_uri(&self, endpoint: &str) -> String {
        format!("{}{}", self.uri("/api/Bots"), endpoint)
    }
    
    /// Gets a list of available marketplace.tf bots.
    pub async fn get_bots(&self) -> Result<Vec<Bot>, Error> {
        #[derive(Serialize, Debug)]
        struct GetBotsParams<'a> {
            key: &'a str,
        }
        
        let url = self.bots_api_uri("/GetBots/v2");
        let response = self.client.get(url)
            .query(&GetBotsParams {
                key: &self.key,
            })
            .send()
            .await?;
        let body: api_response::GetBotsResponse = helpers::parses_response(response).await?;
        
        Ok(body.bots)
    }
    
    /// Gets a ban for a given SteamID.
    pub async fn get_ban(&self, steamid: &SteamID) -> Result<User, Error> {
        #[derive(Serialize, Debug)]
        struct GetBansParams<'a, 'b> {
            key: &'a str,
            steamid: &'b SteamID,
        }
        
        let url = self.bans_api_uri("/GetBans/v2");
        let response = self.client.get(url)
            .query(&GetBansParams {
                key: &self.key,
                steamid,
            })
            .send()
            .await?;
        let body: api_response::GetBansResponse = helpers::parses_response(response).await?;
        
        if let Some(ban) = body.results.first() {
            Ok(ban.to_owned())
        } else {
            Err(Error::Response("Empty results".into()))
        }
    }
    
    /// Gets your dashboard items.
    pub async fn get_dashboard_items(&self) -> Result<DashboardDetails, Error> {
        #[derive(Serialize, Debug)]
        struct GetDashboardItemsParams<'a> {
            key: &'a str,
        }
        
        let url = self.seller_api_uri("/GetDashboardItems/v2");
        let response = self.client.get(url)
            .query(&GetDashboardItemsParams {
                key: &self.key,
            })
            .send()
            .await?;
        let body: DashboardDetails = helpers::parses_response(response).await?;
        
        Ok(body)
    }
    
    /// Gets your sales.
    pub async fn get_sales(&self, num: u32, start_before: Option<u32>) -> Result<Vec<Sale>, Error> {
        #[derive(Serialize, Debug)]
        struct GetSalesParams<'a> {
            key: &'a str,
            num: u32,
            start_before: Option<u32>,
        }
        
        let url = self.seller_api_uri("/GetSales/v2");
        let response = self.client.get(url)
            .query(&GetSalesParams {
                key: &self.key,
                num,
                start_before,
            })
            .send()
            .await?;
        let body: api_response::GetSalesResponse = helpers::parses_response(response).await?;
        
        Ok(body.sales)
    }
}