use serde::{Deserialize, Serialize};

/// Tushare API enum types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Api {
    AdjFactor,
    StockBasic,
    FundBasic,
    FundDaily,
    FundPortfolio,
    Daily,      
    DailyBasic,
    MoneyflowMktDc,
    Weekly,
    Monthly,
    IndexDaily,
    IndexWeekly,
    IndexMonthly,
    TradeCal,
    Margin,
    StockCompany,
    MarginDetail,
    StkHoldernumber,
    ThsIndex, 
    ThsMember,
    ThsDaily,
    ThsHot,
    FinaMainbz,
    FinaMainbzVip,
    FinaIndicator,
    Balancesheet,
    Income,
    Cashflow,
    IndexBasic,
    IndexDailyBasic,
    Moneyflow,
    MoneyflowIndustryThs,

    UsBasic,
    UsDaily,
    Custom(String), // other apis specified by name
}

impl Api {
    pub fn name(&self) -> String {
        match self {
            Api::AdjFactor => "adj_factor".to_string(),
            Api::StockBasic => "stock_basic".to_string(),
            Api::FundBasic => "fund_basic".to_string(),
            Api::FundDaily => "fund_daily".to_string(),
            Api::FundPortfolio => "fund_portfolio".to_string(),
            Api::Daily => "daily".to_string(),
            Api::DailyBasic => "daily_basic".to_string(),
            Api::MoneyflowMktDc => "moneyflow_mkt_dc".to_string(),
            Api::Weekly => "weekly".to_string(),
            Api::Monthly => "monthly".to_string(),
            Api::IndexDaily => "index_daily".to_string(),
            Api::IndexWeekly => "index_weekly".to_string(),
            Api::IndexMonthly => "index_monthly".to_string(),
            Api::TradeCal => "trade_cal".to_string(),
            Api::Margin => "margin".to_string(),
            Api::StockCompany => "stock_company".to_string(),
            Api::MarginDetail => "margin_detail".to_string(),
            Api::StkHoldernumber => "stk_holdernumber".to_string(),
            Api::ThsIndex => "ths_index".to_string(),
            Api::ThsMember => "ths_member".to_string(),
            Api::ThsDaily => "ths_daily".to_string(),
            Api::ThsHot => "ths_hot".to_string(),
            Api::FinaMainbz => "fina_mainbz".to_string(),
            Api::FinaMainbzVip => "fina_mainbz_vip".to_string(),
            Api::FinaIndicator => "fina_indicator".to_string(),
            Api::Balancesheet => "balancesheet".to_string(),
            Api::Income => "income".to_string(),
            Api::Cashflow => "cashflow".to_string(),
            Api::IndexBasic => "index_basic".to_string(),
            Api::IndexDailyBasic => "index_daily_basic".to_string(),
            Api::Moneyflow => "moneyflow".to_string(),
            Api::MoneyflowIndustryThs => "moneyflow_industry_ths".to_string(),
            Api::UsBasic => "us_basic".to_string(),
            Api::UsDaily => "us_daily".to_string(),
            Api::Custom(name) => name.clone(),
        }
    }
}

/// Serialize Api enum to string
pub fn serialize_api_name<S>(api: &Api, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&api.name())
}
