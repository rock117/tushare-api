// 简单的过程宏测试，不使用字段映射功能
use tushare_api::{TushareData, TushareResponse};
use tushare_derive::{FromTushareData, TushareResponseList};
use serde_json::json;

#[derive(Debug, Clone, FromTushareData, TushareResponseList)]
pub struct SimpleStock {
    pub ts_code: String,
    pub symbol: String,
    pub name: String,
}

fn main() {
    println!("=== 简单过程宏测试 ===");

    // 模拟 Tushare API 响应数据
    let response = TushareResponse {
        request_id: "test123".to_string(),
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
                vec![
                    json!("000002.SZ"),
                    json!("000002"),
                    json!("万科A"),
                ],
            ],
        },
    };

    // 测试过程宏自动转换
    match SimpleStockList::try_from(response) {
        Ok(stock_list) => {
            println!("\n过程宏转换成功，获取到 {} 只股票:", stock_list.len());
            for stock in stock_list.iter() {
                println!("  {}: {} ({})", stock.ts_code, stock.name, stock.symbol);
            }
        }
        Err(e) => {
            println!("过程宏转换失败: {}", e);
        }
    }

    println!("\n测试完成！");
}
