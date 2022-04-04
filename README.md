# marketplacetf-api

Interface for marketplace.tf API endpoints.

```rs
use marketplacetf_api::MarketplaceAPI;

let key = "XXXXXXXXXXXXXXXXXXXXXXXX";
let marketplacetf = MarketplaceAPI::new(key);

match marketplacetf.get_sales(10, None).await {
    Ok(sales) => println!("{:?}", sales),
    Err(error) => println!("Error loading sales: {}", error),
}
```

## License

MIT