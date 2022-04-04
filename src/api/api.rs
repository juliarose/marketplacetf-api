use serde::{Serialize, Deserialize};
use std::sync::Arc;
use super::{
    APIError,
    helpers::{
        get_default_middleware,
        parses_response
    }
};
use crate::response;
use reqwest::cookie::Jar;
use reqwest_middleware::ClientWithMiddleware;

const USER_AGENT_STRING: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.71 Safari/537.36";

pub struct MarketplaceAPI {
    key: String,
    client: ClientWithMiddleware,
}

impl MarketplaceAPI {
    
    const HOSTNAME: &'static str = "marketplace.tf";
    
    pub fn new(key: String) -> Self {
        let cookies = Arc::new(Jar::default());
        
        Self {
            key,
            client: get_default_middleware(Arc::clone(&cookies), USER_AGENT_STRING),
        }
    }
    
    fn get_uri(&self, pathname: &str) -> String {
        format!("https://{}{}", Self::HOSTNAME, pathname)
    }

    fn get_api_uri(&self, endpoint: &str) -> String {
        format!("{}{}", self.get_uri("/api/Seller"), endpoint)
    }
    
    pub async fn get_dashboard_items(&self) -> Result<response::DashboardDetails, APIError> {
        #[derive(Serialize, Debug)]
        struct GetDashboardItemsParams<'a> {
            key: &'a str,
        }
        
        let url = self.get_api_uri("/GetDashboardItems/v2");
        let response = self.client.get(url)
            .query(&GetDashboardItemsParams {
                key: &self.key,
            })
            .send()
            .await?;
        let body: response::DashboardDetails = parses_response(response).await?;
        
        Ok(body)
    }
    
    pub async fn get_sales(&self, num: u32, start_before: Option<u32>) -> Result<Vec<response::Sale>, APIError> {
        #[derive(Serialize, Debug)]
        struct GetSalesParams<'a> {
            key: &'a str,
            num: u32,
            start_before: Option<u32>,
        }
        
        let url = self.get_api_uri("/GetSales/v2");
        let response = self.client.get(url)
            .query(&GetSalesParams {
                key: &self.key,
                num,
                start_before,
            })
            .send()
            .await?;
        let body: GetSalesResponse = parses_response(response).await?;
        
        Ok(body.sales)
    }
}
        
#[derive(Deserialize, Clone, Debug)]
struct GetSalesResponse {
    sales: Vec<response::Sale>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
