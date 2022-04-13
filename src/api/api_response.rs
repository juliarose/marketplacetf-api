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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_sales() {
        let response: GetSalesResponse = serde_json::from_str(include_str!("fixtures/sales.json")).unwrap();
        
        assert_eq!(response.sales.first().unwrap().earned_credit, 3);
    }
}