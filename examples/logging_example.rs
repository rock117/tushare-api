use tushare_api::{TushareClient, LogLevel, LogConfig, Api, TushareRequest};
use std::time::Duration;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 注意：在实际使用中，您需要初始化一个日志记录器（如 env_logger）
    // env_logger::init();

    println!("=== Tushare API 日志功能演示 ===\n");

    // 示例 1: 使用默认日志配置
    println!("1. 使用默认日志配置:");
    let client1 = TushareClient::builder()
        .with_token("your_token_here")
        .build()?;

    // 示例 2: 自定义日志级别
    println!("\n2. 自定义日志级别为 Debug:");
    let client2 = TushareClient::builder()
        .with_token("your_token_here")
        .with_log_level(LogLevel::Debug)
        .build()?;

    // 示例 3: 详细的日志配置
    println!("\n3. 详细的日志配置:");
    let client3 = TushareClient::builder()
        .with_token("your_token_here")
        .with_log_level(LogLevel::Trace)
        .log_requests(true)
        .log_responses(true)
        .log_sensitive_data(false) // 生产环境建议设为 false
        .log_performance(true)
        .with_connect_timeout(Duration::from_secs(5))
        .with_timeout(Duration::from_secs(30))
        .build()?;

    // 示例 4: 使用自定义 LogConfig
    println!("\n4. 使用自定义 LogConfig:");
    let log_config = LogConfig {
        level: LogLevel::Info,
        log_requests: true,
        log_responses: false,
        log_sensitive_data: false,
        log_performance: true,
    };
    
    let client4 = TushareClient::builder()
        .with_token("your_token_here")
        .with_log_config(log_config)
        .build()?;

    // 示例 5: 关闭日志
    println!("\n5. 关闭日志:");
    let client5 = TushareClient::builder()
        .with_token("your_token_here")
        .with_log_level(LogLevel::Off)
        .build()?;

    // 演示 API 调用（需要有效的 token）
    if std::env::var("TUSHARE_TOKEN").is_ok() {
        println!("\n=== 实际 API 调用演示 ===");
        
        let client = TushareClient::builder()
            .with_token(&std::env::var("TUSHARE_TOKEN")?)
            .with_log_level(LogLevel::Info)
            .log_performance(true)
            .build()?;

        let mut params = HashMap::new();
        params.insert("list_status".to_string(), "L".to_string());
        
        let req = TushareRequest {
            api_name: Api::StockBasic,
            params,
            fields: vec!["ts_code".to_string(), "name".to_string()],
        };

        match client.call_api(req).await {
            Ok(response) => {
                println!("✅ API 调用成功，返回 {} 条记录", response.data.items.len());
            }
            Err(e) => {
                println!("❌ API 调用失败: {}", e);
            }
        }
    } else {
        println!("\n💡 提示: 设置 TUSHARE_TOKEN 环境变量以查看实际的 API 调用日志");
    }

    println!("\n=== 日志级别说明 ===");
    println!("• Off    - 关闭所有日志");
    println!("• Error  - 只记录错误信息");
    println!("• Warn   - 记录错误和警告");
    println!("• Info   - 记录基本信息（推荐）");
    println!("• Debug  - 记录详细调试信息");
    println!("• Trace  - 记录所有信息包括原始数据");

    println!("\n=== 日志配置选项说明 ===");
    println!("• log_requests      - 是否记录请求参数");
    println!("• log_responses     - 是否记录响应内容（可能很大）");
    println!("• log_sensitive_data - 是否记录敏感数据（如 token）");
    println!("• log_performance   - 是否记录性能指标（耗时等）");

    Ok(())
}
