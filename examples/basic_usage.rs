use tushare_api::{TushareClient, Stock};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量获取 Token
    let token = std::env::var("TUSHARE_TOKEN")
        .unwrap_or_else(|_| {
            println!("警告: 未设置TUSHARE_TOKEN环境变量，请设置您的Tushare API Token");
            println!("示例: set TUSHARE_TOKEN=your_token_here (Windows)");
            println!("示例: export TUSHARE_TOKEN=your_token_here (Linux/Mac)");
            "your_token_here".to_string()
        });

    if token == "your_token_here" {
        println!("请先设置有效的Tushare API Token!");
        return Ok(());
    }

    // 创建客户端
    let client = TushareClient::new(&token);

    println!("正在获取A股股票列表...");
    
    // 获取股票列表
    match client.get_stock_list().await {
        Ok(stocks) => {
            println!("成功获取到 {} 只A股股票:", stocks.len());
            println!("{:<12} {:<8} {:<20} {:<8} {:<15} {:<8} {:<10}", 
                     "股票代码", "简称", "公司名称", "地区", "行业", "市场", "上市日期");
            println!("{}", "-".repeat(90));
            
            // 显示前20条记录
            for (i, stock) in stocks.iter().enumerate() {
                println!("{:<12} {:<8} {:<20} {:<8} {:<15} {:<8} {:<10}", 
                         stock.ts_code, stock.symbol, stock.name, 
                         stock.area, stock.industry, stock.market, stock.list_date);
                
                if i >= 19 {
                    println!("... 还有 {} 条记录", stocks.len() - 20);
                    break;
                }
            }

            // 演示获取特定股票信息
            println!("\n演示：获取平安银行(000001.SZ)的信息...");
            match client.get_stock_by_code("000001.SZ").await {
                Ok(Some(stock)) => {
                    println!("找到股票: {} - {}", stock.ts_code, stock.name);
                    println!("地区: {}, 行业: {}, 上市日期: {}", 
                             stock.area, stock.industry, stock.list_date);
                }
                Ok(None) => {
                    println!("未找到该股票");
                }
                Err(e) => {
                    eprintln!("获取股票信息失败: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("获取股票列表失败: {}", e);
        }
    }

    Ok(())
}
