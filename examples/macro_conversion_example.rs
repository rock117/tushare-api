use tushare_api::{TushareClient, TushareRequest, Api, request, params, fields, tushare_struct, quick_tushare_struct};

// 使用 tushare_struct! 宏定义复杂的股票结构体
tushare_struct! {
    /// 股票基本信息
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
}

// 使用 tushare_struct! 宏定义股票日线数据
tushare_struct! {
    /// 股票日线数据
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
}

// 使用 quick_tushare_struct! 宏定义简单结构体（所有字段都是String）
quick_tushare_struct! {
    /// 简单股票信息
    SimpleStock {
        ts_code,
        symbol,
        name,
    }
}

// 定义基金基本信息
tushare_struct! {
    /// 基金基本信息
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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建客户端
    let client = TushareClient::from_env()?;

    println!("=== 示例1: 使用 tushare_struct! 宏获取股票基本信息 ===");
    
    let request = request!(Api::StockBasic, {
        "list_status" => "L"
    }, [
        "ts_code", "symbol", "name", "area", "industry", "market", "list_date"
    ]);

    // 直接获取 StockList 类型，宏自动生成了包装类型和转换逻辑
    let stock_list: StockList = client.call_api_as(request).await?;
    
    println!("找到 {} 只股票:", stock_list.0.len());
    for (i, stock) in stock_list.0.iter().take(5).enumerate() {
        println!("{}. {} ({}) - {} [{}] {}", 
            i + 1, 
            stock.ts_code, 
            stock.symbol, 
            stock.name,
            stock.area.as_deref().unwrap_or("未知"),
            stock.industry.as_deref().unwrap_or("未知行业")
        );
    }

    println!("\n=== 示例2: 使用 quick_tushare_struct! 宏获取简单股票信息 ===");
    
    let simple_request = request!(Api::StockBasic, {
        "list_status" => "L"
    }, [
        "ts_code", "symbol", "name"
    ]);

    let simple_stock_list: SimpleStockList = client.call_api_as(simple_request).await?;
    
    println!("找到 {} 只股票 (简化版):", simple_stock_list.0.len());
    for (i, stock) in simple_stock_list.0.iter().take(3).enumerate() {
        println!("{}. {} ({}) - {}", i + 1, stock.ts_code, stock.symbol, stock.name);
    }

    println!("\n=== 示例3: 使用 tushare_struct! 宏获取基金信息 ===");
    
    let fund_request = request!(Api::FundBasic, {
        "market" => "E"
    }, [
        "ts_code", "name", "management", "custodian", "fund_type", "found_date", "list_date", "status"
    ]);

    let fund_list: FundList = client.call_api_as(fund_request).await?;
    
    println!("找到 {} 只基金:", fund_list.0.len());
    for (i, fund) in fund_list.0.iter().take(3).enumerate() {
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
