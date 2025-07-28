use tushare_api::{TushareClient, TushareRequest, Api, TushareResult};
use std::collections::HashMap;
use std::env;

#[tokio::main]
async fn main() -> TushareResult<()> {
    // 从环境变量获取 Tushare token
    let token = env::var("TUSHARE_TOKEN")
        .expect("请设置环境变量 TUSHARE_TOKEN");

    // 创建客户端
    let client = TushareClient::new(&token);

    println!("=== 使用通用 API 方法获取股票列表 ===");
    
    // 构建请求参数
    let mut params = HashMap::new();
    params.insert("list_status".to_string(), "L".to_string());
    
    // 创建请求结构体
    let request = TushareRequest {
        api_name: Api::StockBasic,
        params,
        fields: vec![
            "ts_code".to_string(),
            "symbol".to_string(),
            "name".to_string(),
            "area".to_string(),
            "industry".to_string(),
            "list_date".to_string(),
        ],
    };
    
    // 调用通用 API 方法
    let response = client.call_api(request).await?;
    
    println!("API 调用成功！");
    println!("请求 ID: {}", response.request_id);
    println!("返回码: {}", response.code);
    println!("返回字段: {:?}", response.data.fields);
    println!("数据条数: {}", response.data.items.len());
    
    // 显示前5条记录
    println!("\n前5条股票记录:");
    for (i, item) in response.data.items.iter().take(5).enumerate() {
        println!("{}. {:?}", i + 1, item);
    }
    
    println!("\n=== 使用通用 API 方法查询特定股票 ===");
    
    // 查询特定股票
    let mut params = HashMap::new();
    params.insert("ts_code".to_string(), "000001.SZ".to_string());
    
    let request = TushareRequest {
        api_name: Api::StockBasic,
        params,
        fields: vec![
            "ts_code".to_string(),
            "name".to_string(),
            "industry".to_string(),
            "market".to_string(),
            "list_date".to_string(),
        ],
    };
    
    let response = client.call_api(request).await?;
    
    if let Some(stock_data) = response.data.items.first() {
        println!("找到股票信息:");
        for (field, value) in response.data.fields.iter().zip(stock_data.iter()) {
            println!("  {}: {}", field, value);
        }
    } else {
        println!("未找到该股票");
    }

    Ok(())
}
