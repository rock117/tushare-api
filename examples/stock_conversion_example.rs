use tushare_api::{TushareClient, TushareResponse, TushareError, TushareRequest, Api, request, params, fields};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct Stock {
    ts_code: String,
    symbol: String,
    name: String,
}

// Create a wrapper type to avoid orphan rule violations
#[derive(Debug)]
struct StockList(Vec<Stock>);

impl TryFrom<TushareResponse> for StockList {
    type Error = TushareError;

    fn try_from(response: TushareResponse) -> Result<Self, Self::Error> {
        let mut stocks = Vec::new();
        
        // Find the indices of the fields we need
        let ts_code_idx = response.data.fields.iter()
            .position(|f| f == "ts_code")
            .ok_or_else(|| TushareError::ParseError("Missing ts_code field".to_string()))?;
            
        let symbol_idx = response.data.fields.iter()
            .position(|f| f == "symbol")
            .ok_or_else(|| TushareError::ParseError("Missing symbol field".to_string()))?;
            
        let name_idx = response.data.fields.iter()
            .position(|f| f == "name")
            .ok_or_else(|| TushareError::ParseError("Missing name field".to_string()))?;

        // Convert each item to a Stock
        for item in response.data.items {
            if item.len() <= ts_code_idx || item.len() <= symbol_idx || item.len() <= name_idx {
                return Err(TushareError::ParseError("Item has insufficient fields".to_string()));
            }

            let ts_code = item[ts_code_idx].as_str()
                .ok_or_else(|| TushareError::ParseError("ts_code is not a string".to_string()))?
                .to_string();
                
            let symbol = item[symbol_idx].as_str()
                .ok_or_else(|| TushareError::ParseError("symbol is not a string".to_string()))?
                .to_string();
                
            let name = item[name_idx].as_str()
                .ok_or_else(|| TushareError::ParseError("name is not a string".to_string()))?
                .to_string();

            stocks.push(Stock {
                ts_code,
                symbol,
                name,
            });
        }

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

    // Use the new generic method to get StockList directly
    let stock_list: StockList = client.call_api_as(request).await?;

    // Display the results
    println!("Found {} stocks:", stock_list.0.len());
    for (i, stock) in stock_list.0.iter().take(10).enumerate() {
        println!("{}. {} ({}) - {}", i + 1, stock.ts_code, stock.symbol, stock.name);
    }

    if stock_list.0.len() > 10 {
        println!("... and {} more stocks", stock_list.0.len() - 10);
    }

    Ok(())
}
