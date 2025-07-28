// 这个示例演示如何在使用 tracing 的用户程序中集成 tushare-api 的日志
// 
// 运行方式：
// cargo run --example tracing_example --features tracing

use tushare_api::{TushareClient, TushareRequest, Api, LogLevel, LogConfig};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Tushare API Tracing Integration Example ===");
    
    // Method 1: Using tracing-log bridge (recommended for mixed ecosystems)
    #[cfg(all(feature = "tracing", feature = "tracing-log"))] 
    {
        println!("\n--- Method 1: Using tracing-log bridge ---");
        use tracing_subscriber;
        use tracing_log::LogTracer;
        
        // Initialize log-to-tracing bridge
        LogTracer::init()?;
        
        // Set up tracing subscriber
        tracing_subscriber::fmt()
            .with_env_filter("debug")
            .with_target(false)
            .with_thread_ids(true)
            .with_level(true)
            .with_max_level(tracing::Level::DEBUG)
            .init();
    }
    
    // Method 2: Using native tracing feature (when library is compiled with tracing feature)
    #[cfg(feature = "tracing")]
    {
        println!("\n--- Method 2: Using native tracing feature ---");
        println!("库和用户程序都使用 tracing\n");
        
        // 直接初始化 tracing subscriber
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    }

    println!("初始化 Tushare 客户端...");
    
    let client = TushareClient::builder()
        .with_token("demo_token_for_testing")
        .with_log_level(LogLevel::Debug)
        .log_requests(true)
        .log_responses(false)
        .log_performance(true)
        .build()?;

    println!("创建测试请求...");
    
    let mut params = HashMap::new();
    params.insert("list_status".to_string(), "L".to_string());
    
    let request = TushareRequest {
        api_name: Api::StockBasic,
        params,
        fields: vec!["ts_code".to_string(), "name".to_string()],
    };

    println!("发送 API 请求（注意观察日志输出）...");
    
    // 这会触发我们库的日志输出
    match client.call_api(request).await {
        Ok(_response) => {
            println!("✅ API 调用成功（实际会因为 token 无效而失败，但能看到日志）");
        }
        Err(e) => {
            println!("❌ API 调用失败（预期行为）: {}", e);
        }
    }

    #[cfg(not(feature = "tracing"))]
    {
        println!("\n--- Tracing feature not enabled ---");
        println!("To enable tracing support, compile with: cargo build --features tracing");
        println!("Or add tracing-log bridge support with: cargo build --features tracing-log");
    }

    println!("\n=== 总结 ===");
    #[cfg(feature = "tracing")]
    println!("✅ 使用了原生 tracing 支持，日志输出更加结构化");
    
    #[cfg(not(feature = "tracing"))]
    println!("✅ 使用了 tracing-log 桥接，成功捕获了库的 log 输出");

    Ok(())
}
