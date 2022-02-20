# marketplacetf-api

Interface for marketplace.tf API endpoints.

```rs
use marketplacetf_api::{
    MarketplaceAPI,
    request::GetSales
};

let key = "XXXXXXXXXXXXXXXXXXXXXXXX";
let marketplacetf = MarketplaceAPI::new(key);

match marketplacetf.get_sales(&GetSales {
    num: Some(10),
    start_before: None,
}).await {
    Ok(response) => println!("{:?}", response),
    Err(error) => println!("Error loading sales: {}", error),
}
```

## License

MIT