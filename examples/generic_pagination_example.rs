use tushare_api::{TushareClient, Api, request, TushareEntityList};
use tushare_api::DeriveFromTushareData;

#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct Stock {
    pub ts_code: String,
    pub name: String,
    pub area: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a mock TushareResponse to test the new generic pagination container
    use tushare_api::{TushareResponse, TushareData};
    use serde_json::json;
    
    let mock_response = TushareResponse {
        request_id: "test123".to_string(),
        code: 0,
        msg: None,
        data: TushareData {
            fields: vec!["ts_code".to_string(), "name".to_string(), "area".to_string()],
            items: vec![
                vec![json!("000001.SZ"), json!("平安银行"), json!("深圳")],
                vec![json!("000002.SZ"), json!("万科A"), json!("深圳")],
                vec![json!("600000.SH"), json!("浦发银行"), json!("上海")],
            ],
            has_more: true,
            count: 4500,
        },
    };
    
    // Test the new TushareEntityList conversion
    let stocks: TushareEntityList<Stock> = TushareEntityList::try_from(mock_response)?;
    
    println!("=== New Generic Pagination Container Demo ===");
    println!("Current page: {} stocks", stocks.len());
    println!("Total available: {} stocks", stocks.count());
    println!("Has more pages: {}", stocks.has_more());
    println!();
    
    println!("Stocks in current page:");
    for (i, stock) in stocks.iter().enumerate() {
        println!("  {}: {} - {} ({})", 
                 i + 1,
                 stock.ts_code, 
                 stock.name, 
                 stock.area.as_deref().unwrap_or("Unknown"));
    }
    println!();
    
    // Test direct iteration
    println!("Direct iteration:");
    for stock in &stocks {
        println!("  {}: {}", stock.ts_code, stock.name);
    }
    println!();
    
    // Test accessing items directly
    let first_stock = &stocks.items()[0];
    println!("First stock via items(): {} - {}", first_stock.ts_code, first_stock.name);
    
    // Test Deref functionality (accessing Vec<T> methods directly)
    println!("Is empty: {}", stocks.is_empty());
    println!("Length via deref: {}", stocks.len());
    
    // Test conversion to Vec
    let items_vec = stocks.clone().into_items();
    println!("Converted to Vec<Stock> with {} items", items_vec.len());
    
    println!("\n=== API Usage Comparison ===");
    
    // Old way (would be):
    // let stocks: StockList = client.call_api_as<StockList>(request).await?;
    println!("❌ Old way: let stocks: StockList = client.call_api_as<StockList>(request).await?;");
    println!("   Problem: Need to remember StockList type name, not clear what's inside");
    
    // New way:
    // let stocks: TushareEntityList<Stock> = client.call_api_as<Stock>(request).await?;
    println!("✅ New way: let stocks: TushareEntityList<Stock> = client.call_api_as<Stock>(request).await?;");
    println!("   Benefit: Clear, intuitive, shows exactly what entity type is contained");
    
    Ok(())
}
