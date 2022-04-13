use serde::Deserialize;
use crate::response;
        
#[derive(Deserialize, Clone, Debug)]
pub struct GetSalesResponse {
    pub sales: Vec<response::Sale>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_delete_listings() {
        let response: GetSalesResponse = serde_json::from_str(include_str!("fixtures/sales.json")).unwrap();
        
        assert_eq!(response.sales.first().unwrap().earned_credit, 3);
    }
}