use marketplacetf_api::{MarketplaceAPI, error::Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let key = "key";
    let marketplacetf = MarketplaceAPI::new(key);
    let bots = marketplacetf.get_bots().await?;
    
    println!("{bots:?}");
    
    Ok(())
}
