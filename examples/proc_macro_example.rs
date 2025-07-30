use tushare_api::{TushareClient, TushareRequest, Api};
use tushare_derive::{FromTushareData, TushareResponseList};

// 使用过程宏定义股票结构体
#[derive(Debug, FromTushareData, TushareResponseList)]
struct Stock {
    ts_code: String,
    symbol: String,
    name: String,
    area: Option<String>,
    industry: Option<String>,
    market: String,
    list_date: Option<String>,
}

// 使用过程宏定义股票日线数据
#[derive(Debug, FromTushareData, TushareResponseList)]
struct StockDaily {
    ts_code: String,
    trade_date: String,
    open: Option<f64>,
    high: Option<f64>,
    low: Option<f64>,
    close: Option<f64>,
    pre_close: Option<f64>,
    change: Option<f64>,
    pct_chg: Option<f64>,
    vol: Option<f64>,
    amount: Option<f64>,
}

// 使用字段映射的示例
#[derive(Debug, FromTushareData, TushareResponseList)]
struct StockInfo {
    ts_code: String,
    #[tushare(field = "symbol")]
    stock_symbol: String,
    #[tushare(field = "name")]
    stock_name: String,
    #[tushare(skip)]
    calculated_value: f64,
}

impl Default for StockInfo {
    fn default() -> Self {
        Self {
            ts_code: String::new(),
            stock_symbol: String::new(),
            stock_name: String::new(),
            calculated_value: 0.0,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量获取 token
    let client = TushareClient::from_env()?;

    println!("=== 获取股票基本信息 ===");
    
    // 创建请求
    let request = TushareRequest::new(
        Api::StockBasic,
        vec![("list_status".into(), "L".into())],
        vec![
            "ts_code".into(),
            "symbol".into(),
            "name".into(),
            "area".into(),
            "industry".into(),
            "market".into(),
            "list_date".into(),
        ]
    );

    // 使用过程宏自动转换
    let stocks: StockList = client.call_api_as(request).await?;
    
    println!("获取到 {} 只股票", stocks.len());
    
    // 显示前5只股票
    for stock in stocks.iter().take(5) {
        println!(
            "{}: {} ({}), 行业: {}, 地区: {}, 上市日期: {}",
            stock.ts_code,
            stock.name,
            stock.symbol,
            stock.industry.as_deref().unwrap_or("N/A"),
            stock.area.as_deref().unwrap_or("N/A"),
            stock.list_date.as_deref().unwrap_or("N/A")
        );
    }

    println!("\n=== 获取股票日线数据 ===");
    
    // 获取某只股票的日线数据
    if let Some(first_stock) = stocks.first() {
        let daily_request = TushareRequest::new(
            Api::Daily,
            vec![
                ("ts_code".into(), first_stock.ts_code.clone()),
                ("start_date".into(), "20240101".into()),
                ("end_date".into(), "20240131".into()),
            ],
            vec![
                "ts_code".into(),
                "trade_date".into(),
                "open".into(),
                "high".into(),
                "low".into(),
                "close".into(),
                "pre_close".into(),
                "change".into(),
                "pct_chg".into(),
                "vol".into(),
                "amount".into(),
            ]
        );

        let daily_data: StockDailyList = client.call_api_as(daily_request).await?;
        
        println!("获取到 {} 条日线数据", daily_data.len());
        
        for daily in daily_data.iter().take(5) {
            println!(
                "{} {}: 开盘={:.2}, 最高={:.2}, 最低={:.2}, 收盘={:.2}, 涨跌幅={:.2}%",
                daily.ts_code,
                daily.trade_date,
                daily.open.unwrap_or(0.0),
                daily.high.unwrap_or(0.0),
                daily.low.unwrap_or(0.0),
                daily.close.unwrap_or(0.0),
                daily.pct_chg.unwrap_or(0.0)
            );
        }
    }

    println!("\n=== 字段映射示例 ===");
    
    // 使用字段映射的示例
    let info_request = TushareRequest::new(
        Api::StockBasic,
        vec![("list_status".into(), "L".into())],
        vec!["ts_code".into(), "symbol".into(), "name".into()]
    );

    let stock_infos: StockInfoList = client.call_api_as(info_request).await?;
    
    println!("获取到 {} 条股票信息", stock_infos.len());
    
    for info in stock_infos.iter().take(3) {
        println!(
            "代码: {}, 符号: {}, 名称: {}, 计算值: {}",
            info.ts_code,
            info.stock_symbol,
            info.stock_name,
            info.calculated_value
        );
    }

    Ok(())
}
