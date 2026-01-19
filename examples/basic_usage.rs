use tushare_api::{TushareClient, TushareRequest, Api, TushareResult, params, fields};
use std::time::Duration;
use std::collections::HashMap;
use std::env;

#[tokio::main]
async fn main() -> TushareResult<()> {
    // 从环境变量获取 token
    let token = env::var("TUSHARE_TOKEN")
        .expect("请设置环境变量 TUSHARE_TOKEN");

    // 创建客户端（使用默认超时设置）
    let client = TushareClient::new(&token);
    
    println!("=== 使用默认超时设置获取股票基本信息 ===");
    
    let request = TushareRequest {
        api_name: Api::StockBasic,
        params: params!("list_status" => "L"),
        fields: fields!["ts_code", "name", "industry", "area"],
    };
    
    match client.call_api(request).await {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("成功获取到 {} 条记录", data.items.len());

                // 显示前10条记录
                println!("\n前10条股票信息:");
                for (i, item) in data.items.iter().take(10).enumerate() {
                    println!("{}. {:?}", i + 1, item);
                }
            }
        }
        Err(e) => {
            eprintln!("获取股票列表失败: {}", e);
        }
    }
    
    println!("\n=== 使用自定义超时设置查询特定股票 ===");
    
    // 创建客户端（自定义超时设置）
    let client_with_timeout = TushareClient::with_timeout(
        &token,
        Duration::from_secs(5),  // 连接超时 5 秒
        Duration::from_secs(60)  // 请求超时 60 秒
    );
    
    let mut stock_params = HashMap::new();
    stock_params.insert("ts_code".into(), "000001.SZ".into());
    
    let stock_request = TushareRequest {
        api_name: Api::StockBasic,
        params: stock_params,
        fields: vec![
            "ts_code".into(),
            "symbol".into(),
            "name".into(),
            "area".into(),
            "industry".into(),
            "list_date".into(),
        ],
    };
    
    match client_with_timeout.call_api(stock_request).await {
        Ok(response) => {
            if let Some(data) = response.data {
                if let Some(item) = data.items.first() {
                    println!("找到股票信息:");
                    for (field, value) in data.fields.iter().zip(item.iter()) {
                        println!("  {}: {}", field, value);
                    }
                } else {
                    println!("未找到该股票");
                }
            }
        }
        Err(e) => {
            eprintln!("查询股票失败: {}", e);
        }
    }

    Ok(())
}
