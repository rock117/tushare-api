use tushare_api::{TushareClient, Api, request, TushareEntityList, TushareRequest, params, fields};
use tushare_api::DeriveFromTushareData;

#[derive(Debug, Clone, DeriveFromTushareData)]
struct Stock {
    ts_code: String,
    symbol: String,
    name: String,
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

    // Method 1: Use the generic call_api_as method with TushareEntityList
    println!("=== Method 1: Using call_api_as with TushareEntityList ===");
    let stock_list: TushareEntityList<Stock> = client.call_api_as(request.clone()).await?;
    
    println!("Found {} stocks:", stock_list.len());
    for (i, stock) in stock_list.iter().take(5).enumerate() {
        println!("{}. {} ({}) - {}", i + 1, stock.ts_code, stock.symbol, stock.name);
    }
    
    // Show pagination info
    println!("Total records: {}", stock_list.count());
    println!("Has more pages: {}", stock_list.has_more());

    // Method 2: Use the traditional call_api and manual conversion
    println!("\n=== Method 2: Using call_api + manual conversion ===");
    let response = client.call_api(&request).await?;
    let stocks: Vec<Stock> = tushare_api::utils::response_to_vec(response)?;
    
    println!("Found {} stocks:", stocks.len());
    for (i, stock) in stocks.iter().take(5).enumerate() {
        println!("{}. {} ({}) - {}", i + 1, stock.ts_code, stock.symbol, stock.name);
    }

    Ok(())
}
