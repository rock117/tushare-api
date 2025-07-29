use tushare_api::{TushareData, TushareResponse};
use tushare_derive::{FromTushareData, TushareResponseList};
use serde_json::json;

// 使用过程宏定义简单的股票结构体
#[derive(Debug, Clone, FromTushareData, TushareResponseList)]
struct Stock {
    ts_code: String,
    symbol: String,
    name: String,
    area: Option<String>,
}

// 使用字段映射和跳过字段的示例
#[derive(Debug, Clone, FromTushareData, TushareResponseList)]
struct StockWithMapping {
    ts_code: String,
    #[tushare(field = "symbol")]
    stock_symbol: String,
    #[tushare(field = "name")]
    stock_name: String,
    #[tushare(skip)]
    calculated_field: f64,
}

impl Default for StockWithMapping {
    fn default() -> Self {
        Self {
            ts_code: String::new(),
            stock_symbol: String::new(),
            stock_name: String::new(),
            calculated_field: 100.0, // 默认值
        }
    }
}

fn main() {
    println!("=== 过程宏演示 ===");

    // 模拟 Tushare API 响应数据
    let response = TushareResponse {
        request_id: "demo123".to_string(),
        code: 0,
        msg: None,
        data: TushareData {
            fields: vec![
                "ts_code".to_string(),
                "symbol".to_string(),
                "name".to_string(),
                "area".to_string(),
            ],
            items: vec![
                vec![
                    json!("000001.SZ"),
                    json!("000001"),
                    json!("平安银行"),
                    json!("深圳"),
                ],
                vec![
                    json!("000002.SZ"),
                    json!("000002"),
                    json!("万科A"),
                    json!(null), // area 为空
                ],
                vec![
                    json!("600000.SH"),
                    json!("600000"),
                    json!("浦发银行"),
                    json!("上海"),
                ],
            ],
        },
    };

    // 使用过程宏自动转换
    let stocks: StockList = response.try_into().unwrap();
    
    println!("成功转换 {} 只股票:", stocks.len());
    
    for stock in stocks.iter() {
        println!(
            "  {}: {} ({}), 地区: {}",
            stock.ts_code,
            stock.name,
            stock.symbol,
            stock.area.as_deref().unwrap_or("N/A")
        );
    }

    println!("\n=== 字段映射演示 ===");

    // 模拟字段映射的响应
    let mapping_response = TushareResponse {
        request_id: "mapping123".to_string(),
        code: 0,
        msg: None,
        data: TushareData {
            fields: vec![
                "ts_code".to_string(),
                "symbol".to_string(),
                "name".to_string(),
            ],
            items: vec![
                vec![
                    json!("000001.SZ"),
                    json!("000001"),
                    json!("平安银行"),
                ],
            ],
        },
    };

    let mapped_stocks: StockWithMappingList = mapping_response.try_into().unwrap();
    
    println!("字段映射结果:");
    for stock in mapped_stocks.iter() {
        println!(
            "  代码: {}, 符号: {}, 名称: {}, 计算字段: {}",
            stock.ts_code,
            stock.stock_symbol,
            stock.stock_name,
            stock.calculated_field
        );
    }

    println!("\n过程宏演示完成！");
}
