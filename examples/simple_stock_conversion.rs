use tushare_api::{TushareClient, TushareResponse, TushareError, TushareRequest, Api, request, params, fields, FromTushareData, get_string_field};
use serde_json::Value;

#[derive(Debug, Clone)]
struct Stock {
    ts_code: String,
    symbol: String,
    name: String,
}

impl FromTushareData for Stock {
    fn from_row(fields: &[String], values: &[Value]) -> Result<Self, TushareError> {
        Ok(Stock {
            ts_code: get_string_field(fields, values, "ts_code")?,
            symbol: get_string_field(fields, values, "symbol")?,
            name: get_string_field(fields, values, "name")?,
        })
    }
}

// Create a wrapper type to avoid orphan rule violations
#[derive(Debug)]
struct StockList(Vec<Stock>);

impl TryFrom<TushareResponse> for StockList {
    type Error = TushareError;

    fn try_from(response: TushareResponse) -> Result<Self, Self::Error> {
        let stocks: Vec<Stock> = tushare_api::response_to_vec(response)?;
        Ok(StockList(stocks))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client (you need to set TUSHARE_TOKEN environment variable)
    let client = TushareClient::from_env()?;

    // Create request for stock basic data
    let request = request!(Api::StockBasic, {
        "list_status" => "L"
    }, [
        "ts_code", "symbol", "name"
    ]);

    // Method 1: Use the generic call_api_as method
    println!("=== Method 1: Using call_api_as ===");
    let stock_list: StockList = client.call_api_as(request.clone()).await?;
    
    println!("Found {} stocks:", stock_list.0.len());
    for (i, stock) in stock_list.0.iter().take(5).enumerate() {
        println!("{}. {} ({}) - {}", i + 1, stock.ts_code, stock.symbol, stock.name);
    }

    // Method 2: Use the traditional call_api and manual conversion
    println!("\n=== Method 2: Using call_api + manual conversion ===");
    let response = client.call_api(request).await?;
    let stocks: Vec<Stock> = tushare_api::response_to_vec(response)?;
    
    println!("Found {} stocks:", stocks.len());
    for (i, stock) in stocks.iter().take(5).enumerate() {
        println!("{}. {} ({}) - {}", i + 1, stock.ts_code, stock.symbol, stock.name);
    }

    Ok(())
}
