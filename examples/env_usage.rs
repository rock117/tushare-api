use tushare_api::{TushareClient, TushareRequest, Api, TushareResult, params, fields};
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
    
    // 使用宏构建请求（支持直接使用字符串字面量）
    let request = TushareRequest {
        api_name: Api::StockBasic,
        params: params!("list_status" => "L"),
        fields: fields!["ts_code", "name", "industry", "area"],
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
    
    println!("\n=== 演示自定义 API 调用 ===\n");
    
    let custom_request = TushareRequest {
        api_name: Api::Custom("daily".to_string()),
        params: params!("ts_code" => "000001.SZ"),
        fields: fields!["ts_code", "trade_date", "close"],
    };
    
    match client_with_timeout.call_api(custom_request).await {
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
