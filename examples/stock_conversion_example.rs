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

    // Use the new generic method to get TushareEntityList<Stock> directly
    let stock_list: TushareEntityList<Stock> = client.call_api_as(request).await?;

    // Display the results
    println!("Found {} stocks:", stock_list.len());
    for (i, stock) in stock_list.iter().take(10).enumerate() {
        println!("{}. {} ({}) - {}", i + 1, stock.ts_code, stock.symbol, stock.name);
    }

    if stock_list.len() > 10 {
        println!("... and {} more stocks", stock_list.len() - 10);
    }
    
    // Show pagination info
    println!("Total records: {}", stock_list.count());
    println!("Has more pages: {}", stock_list.has_more());

    Ok(())
}
