use tushare_api::{TushareClient, TushareRequest, Api, TushareResult};
use std::collections::HashMap;
use std::time::Duration;

#[tokio::main]
async fn main() -> TushareResult<()> {
    println!("=== 使用环境变量 TUSHARE_TOKEN 创建客户端 ===");
    
    // 从环境变量创建客户端（使用默认超时设置）
    let client = match TushareClient::from_env() {
        Ok(client) => {
            println!("✅ 成功从环境变量 TUSHARE_TOKEN 创建客户端");
            client
        }
        Err(e) => {
            eprintln!("❌ 无法从环境变量创建客户端: {}", e);
            eprintln!("请确保设置了环境变量 TUSHARE_TOKEN");
            eprintln!("例如: export TUSHARE_TOKEN=your_token_here");
            return Err(e);
        }
    };
    
    // 创建请求
    let mut params = HashMap::new();
    params.insert("list_status".to_string(), "L".to_string());
    
    let request = TushareRequest {
        api_name: Api::StockBasic,
        params,
        fields: vec![
            "ts_code".to_string(),
            "name".to_string(),
            "industry".to_string(),
            "area".to_string(),
        ],
    };
    
    // 调用 API
    match client.call_api(request).await {
        Ok(response) => {
            println!("✅ 成功获取到 {} 条记录", response.data.items.len());
            
            // 显示前5条记录
            println!("\n前5条股票信息:");
            for (i, item) in response.data.items.iter().take(5).enumerate() {
                println!("{}. {:?}", i + 1, item);
            }
        }
        Err(e) => {
            eprintln!("❌ 获取股票列表失败: {}", e);
        }
    }
    
    println!("\n=== 使用环境变量和自定义超时设置 ===");
    
    // 从环境变量创建客户端（自定义超时设置）
    let client_with_timeout = TushareClient::from_env_with_timeout(
        Duration::from_secs(5),  // 连接超时 5 秒
        Duration::from_secs(60)  // 请求超时 60 秒
    )?;
    
    let mut stock_params = HashMap::new();
    stock_params.insert("ts_code".to_string(), "000001.SZ".to_string());
    
    let stock_request = TushareRequest {
        api_name: Api::StockBasic,
        params: stock_params,
        fields: vec![
            "ts_code".to_string(),
            "symbol".to_string(),
            "name".to_string(),
            "area".to_string(),
            "industry".to_string(),
            "list_date".to_string(),
        ],
    };
    
    match client_with_timeout.call_api(stock_request).await {
        Ok(response) => {
            if let Some(item) = response.data.items.first() {
                println!("✅ 找到股票信息:");
                for (field, value) in response.data.fields.iter().zip(item.iter()) {
                    println!("  {}: {}", field, value);
                }
            } else {
                println!("未找到该股票");
            }
        }
        Err(e) => {
            eprintln!("❌ 查询股票失败: {}", e);
        }
    }

    Ok(())
}
