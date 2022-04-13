use serde::Serialize;
use std::sync::Arc;
use super::helpers;
use super::api_response;
use crate::{response, error::Error};
use reqwest::cookie::Jar;
use reqwest_middleware::ClientWithMiddleware;

const USER_AGENT_STRING: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.71 Safari/537.36";

pub struct MarketplaceAPI {
    key: String,
    client: ClientWithMiddleware,
}

impl MarketplaceAPI {
    
    const HOSTNAME: &'static str = "marketplace.tf";
    
    pub fn new(key: &str) -> Self {
        let cookies = Arc::new(Jar::default());
        
        Self {
            key: key.into(),
            client: helpers::get_default_middleware(Arc::clone(&cookies), USER_AGENT_STRING),
        }
    }
    
    fn uri(&self, pathname: &str) -> String {
        format!("https://{}{}", Self::HOSTNAME, pathname)
    }

    fn seller_api_uri(&self, endpoint: &str) -> String {
        format!("{}{}", self.uri("/api/Seller"), endpoint)
    }

    fn bots_api_uri(&self, endpoint: &str) -> String {
        format!("{}{}", self.uri("/api/Bots"), endpoint)
    }
    
    pub async fn get_bots(&self) -> Result<Vec<response::Bot>, Error> {
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
    
    pub async fn get_dashboard_items(&self) -> Result<response::DashboardDetails, Error> {
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
        let body: response::DashboardDetails = helpers::parses_response(response).await?;
        
        Ok(body)
    }
    
    pub async fn get_sales(&self, num: u32, start_before: Option<u32>) -> Result<Vec<response::Sale>, Error> {
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