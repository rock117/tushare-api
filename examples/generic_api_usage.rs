use tushare_api::{TushareClient, TushareRequest, Api, TushareResult, params, fields};
use std::env;

#[tokio::main]
async fn main() -> TushareResult<()> {
    // 从环境变量获取 Tushare token
    let token = env::var("TUSHARE_TOKEN")
        .expect("请设置环境变量 TUSHARE_TOKEN");

    // 创建客户端
    let client = TushareClient::new(&token);

    println!("=== 使用通用 API 方法获取股票列表 ===");
    
    // 使用宏构建请求（支持直接使用字符串字面量）
    let request = TushareRequest {
        api_name: Api::StockBasic,
        params: params!("list_status" => "L"),
        fields: fields!["ts_code", "symbol", "name", "area", "industry", "list_date"],
    };
    
    // 调用通用 API 方法
    let response = client.call_api(request).await?;
    
    println!("API 调用成功！");
    println!("请求 ID: {}", response.request_id);
    println!("返回码: {}", response.code);
    if let Some(data) = response.data {
        println!("返回字段: {:?}", data.fields);
        println!("数据条数: {}", data.items.len());
        
        // 显示前5条记录
        println!("\n前5条股票记录:");
        for (i, item) in data.items.iter().take(5).enumerate() {
            println!("{}. {:?}", i + 1, item);
        }
    }
    
    println!("\n=== 使用通用 API 方法查询特定股票 ===");
    
    // 查询特定股票
    let request = TushareRequest {
        api_name: Api::StockBasic,
        params: params!("ts_code" => "000001.SZ"),
        fields: fields!["ts_code", "name", "industry", "market", "list_date"],
    };
    
    let response = client.call_api(request).await?;

    if let Some(data) = response.data {
        if let Some(stock_data) = data.items.first() {
            println!("找到股票信息:");
            for (field, value) in data.fields.iter().zip(stock_data.iter()) {
                println!("  {}: {}", field, value);
            }
        } else {
            println!("未找到该股票");
        }
    }

    Ok(())
}
