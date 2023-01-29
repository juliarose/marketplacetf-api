use serde::Deserialize;
use crate::response;
        
#[derive(Deserialize, Debug)]
pub struct GetSalesResponse {
    pub sales: Vec<response::Sale>,
}

#[derive(Deserialize, Debug)]
pub struct GetBotsResponse {
    pub bots: Vec<response::Bot>,
}

#[derive(Deserialize, Debug)]
pub struct GetBansResponse {
    pub results: Vec<response::UserBan>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_get_sales() {
        let response: GetSalesResponse = serde_json::from_str(include_str!("fixtures/get_sales.json")).unwrap();
        
        assert_eq!(response.sales.first().unwrap().earned_credit, 3);
    }
    
    #[test]
    fn parses_get_bans() {
        let response: GetBansResponse = serde_json::from_str(include_str!("fixtures/get_bans.json")).unwrap();
        
        assert!(response.results.first().unwrap().banned);
    }
}