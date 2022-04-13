use marketplacetf_api::{MarketplaceAPI, error::Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let key = "key";
    let marketplacetf = MarketplaceAPI::new(key);
    let sales = marketplacetf.get_sales(10, None).await?;
    
    if let Some(sale) = sales.first() {
        let names = sale.items
            .iter()
            .map(|item| item.name.as_str())
            .collect::<Vec<_>>();
        
        println!("Sold {} for ${:.2}!", names.join(", "), sale.earned_credit as f32 / 100.0);
    } else {
        println!("You have not sold anything :(");
    }
    
    Ok(())
}
