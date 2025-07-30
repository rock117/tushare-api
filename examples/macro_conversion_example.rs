use tushare_api::{TushareClient, Api, request, TushareEntityList, TushareRequest, params, fields};
use tushare_api::DeriveFromTushareData;

/// 股票基本信息
#[derive(Debug, Clone, DeriveFromTushareData)]
struct Stock {
    /// 股票代码
    ts_code: String,
    /// 股票简称
    symbol: String,
    /// 股票名称
    name: String,
    /// 地区
    area: Option<String>,
    /// 行业
    industry: Option<String>,
    /// 市场类型
    market: String,
    /// 上市日期
    list_date: Option<String>,
}

/// 股票日线数据
#[derive(Debug, Clone, DeriveFromTushareData)]
struct StockDaily {
    /// 股票代码
    ts_code: String,
    /// 交易日期
    trade_date: String,
    /// 开盘价
    open: Option<f64>,
    /// 最高价
    high: Option<f64>,
    /// 最低价
    low: Option<f64>,
    /// 收盘价
    close: Option<f64>,
    /// 昨收价
    pre_close: Option<f64>,
    /// 涨跌额
    change: Option<f64>,
    /// 涨跌幅
    pct_chg: Option<f64>,
    /// 成交量
    vol: Option<f64>,
    /// 成交额
    amount: Option<f64>,
}

/// 简单股票信息
#[derive(Debug, Clone, DeriveFromTushareData)]
struct SimpleStock {
    ts_code: String,
    symbol: String,
    name: String,
}

/// 基金基本信息
#[derive(Debug, Clone, DeriveFromTushareData)]
struct Fund {
    /// 基金代码
    ts_code: String,
    /// 基金简称
    name: String,
    /// 管理人
    management: Option<String>,
    /// 托管人
    custodian: Option<String>,
    /// 基金类型
    fund_type: Option<String>,
    /// 成立日期
    found_date: Option<String>,
    /// 到期日期
    due_date: Option<String>,
    /// 上市日期
    list_date: Option<String>,
    /// 发行日期
    issue_date: Option<String>,
    /// 退市日期
    delist_date: Option<String>,
    /// 基金状态
    status: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建客户端
    let client = TushareClient::from_env()?;

    println!("=== 示例1: 使用 FromTushareData 派生宏获取股票基本信息 ===");
    
    let request = request!(Api::StockBasic, {
        "list_status" => "L"
    }, [
        "ts_code", "symbol", "name", "area", "industry", "market", "list_date"
    ]);

    // 直接获取 TushareEntityList<Stock> 类型
    let stock_list: TushareEntityList<Stock> = client.call_api_as(request).await?;
    
    println!("找到 {} 只股票:", stock_list.len());
    for (i, stock) in stock_list.iter().take(5).enumerate() {
        println!("{}. {} ({}) - {} [{}] {}", 
            i + 1, 
            stock.ts_code, 
            stock.symbol, 
            stock.name,
            stock.area.as_deref().unwrap_or("未知"),
            stock.industry.as_deref().unwrap_or("未知行业")
        );
    }
    
    // 显示分页信息
    println!("总记录数: {}, 是否有更多页面: {}", stock_list.count(), stock_list.has_more());

    println!("\n=== 示例2: 获取简单股票信息 ===");
    
    let simple_request = request!(Api::StockBasic, {
        "list_status" => "L"
    }, [
        "ts_code", "symbol", "name"
    ]);

    let simple_stock_list: TushareEntityList<SimpleStock> = client.call_api_as(simple_request).await?;
    
    println!("找到 {} 只股票 (简化版):", simple_stock_list.len());
    for (i, stock) in simple_stock_list.iter().take(3).enumerate() {
        println!("{}. {} ({}) - {}", i + 1, stock.ts_code, stock.symbol, stock.name);
    }

    println!("\n=== 示例3: 获取基金信息 ===");
    
    let fund_request = request!(Api::FundBasic, {
        "market" => "E"
    }, [
        "ts_code", "name", "management", "custodian", "fund_type", "found_date", "list_date", "status"
    ]);

    let fund_list: TushareEntityList<Fund> = client.call_api_as(fund_request).await?;
    
    println!("找到 {} 只基金:", fund_list.len());
    for (i, fund) in fund_list.iter().take(3).enumerate() {
        println!("{}. {} - {} [{}] 管理人: {}", 
            i + 1, 
            fund.ts_code, 
            fund.name,
            fund.fund_type.as_deref().unwrap_or("未知类型"),
            fund.management.as_deref().unwrap_or("未知")
        );
    }

    Ok(())
}
