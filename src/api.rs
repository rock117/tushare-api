use serde::{Deserialize, Serialize};
use serde::de;
use std::fmt;

/// Tushare API enum types
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub enum Api {
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

const API_STOCK_BASIC: &str = "stock_basic";
const API_FUND_BASIC: &str = "fund_basic";
const API_FUND_DAILY: &str = "fund_daily";
const API_FUND_PORTFOLIO: &str = "fund_portfolio";
const API_DAILY: &str = "daily";
const API_DAILY_BASIC: &str = "daily_basic";
const API_MONEYFLOW_MKT_DC: &str = "moneyflow_mkt_dc";
const API_WEEKLY: &str = "weekly";
const API_MONTHLY: &str = "monthly";
const API_INDEX_DAILY: &str = "index_daily";
const API_INDEX_WEEKLY: &str = "index_weekly";
const API_INDEX_MONTHLY: &str = "index_monthly";
const API_TRADE_CAL: &str = "trade_cal";
const API_MARGIN: &str = "margin";
const API_STOCK_COMPANY: &str = "stock_company";
const API_MARGIN_DETAIL: &str = "margin_detail";
const API_STK_HOLDERNUMBER: &str = "stk_holdernumber";
const API_THS_INDEX: &str = "ths_index";
const API_THS_MEMBER: &str = "ths_member";
const API_THS_DAILY: &str = "ths_daily";
const API_THS_HOT: &str = "ths_hot";
const API_FINA_MAINBZ: &str = "fina_mainbz";
const API_FINA_MAINBZ_VIP: &str = "fina_mainbz_vip";
const API_FINA_INDICATOR: &str = "fina_indicator";
const API_BALANCESHEET: &str = "balancesheet";
const API_INCOME: &str = "income";
const API_CASHFLOW: &str = "cashflow";
const API_INDEX_BASIC: &str = "index_basic";
const API_INDEX_DAILY_BASIC: &str = "index_daily_basic";
const API_MONEYFLOW: &str = "moneyflow";
const API_MONEYFLOW_INDUSTRY_THS: &str = "moneyflow_industry_ths";
const API_US_BASIC: &str = "us_basic";
const API_US_DAILY: &str = "us_daily";

impl Api {
    fn from_api_str(value: &str) -> Option<Self> {
        let v = value.trim();
        if v.is_empty() {
            return None;
        }

        match v {
            API_STOCK_BASIC => Some(Api::StockBasic),
            API_FUND_BASIC => Some(Api::FundBasic),
            API_FUND_DAILY => Some(Api::FundDaily),
            API_FUND_PORTFOLIO => Some(Api::FundPortfolio),
            API_DAILY => Some(Api::Daily),
            API_DAILY_BASIC => Some(Api::DailyBasic),
            API_MONEYFLOW_MKT_DC => Some(Api::MoneyflowMktDc),
            API_WEEKLY => Some(Api::Weekly),
            API_MONTHLY => Some(Api::Monthly),
            API_INDEX_DAILY => Some(Api::IndexDaily),
            API_INDEX_WEEKLY => Some(Api::IndexWeekly),
            API_INDEX_MONTHLY => Some(Api::IndexMonthly),
            API_TRADE_CAL => Some(Api::TradeCal),
            API_MARGIN => Some(Api::Margin),
            API_STOCK_COMPANY => Some(Api::StockCompany),
            API_MARGIN_DETAIL => Some(Api::MarginDetail),
            API_STK_HOLDERNUMBER => Some(Api::StkHoldernumber),
            API_THS_INDEX => Some(Api::ThsIndex),
            API_THS_MEMBER => Some(Api::ThsMember),
            API_THS_DAILY => Some(Api::ThsDaily),
            API_THS_HOT => Some(Api::ThsHot),
            API_FINA_MAINBZ => Some(Api::FinaMainbz),
            API_FINA_MAINBZ_VIP => Some(Api::FinaMainbzVip),
            API_FINA_INDICATOR => Some(Api::FinaIndicator),
            API_BALANCESHEET => Some(Api::Balancesheet),
            API_INCOME => Some(Api::Income),
            API_CASHFLOW => Some(Api::Cashflow),
            API_INDEX_BASIC => Some(Api::IndexBasic),
            API_INDEX_DAILY_BASIC => Some(Api::IndexDailyBasic),
            API_MONEYFLOW => Some(Api::Moneyflow),
            API_MONEYFLOW_INDUSTRY_THS => Some(Api::MoneyflowIndustryThs),
            API_US_BASIC => Some(Api::UsBasic),
            API_US_DAILY => Some(Api::UsDaily),
            _ => None,
        }
    }

    pub fn name(&self) -> String {
        match self {
            Api::StockBasic => API_STOCK_BASIC.to_string(),
            Api::FundBasic => API_FUND_BASIC.to_string(),
            Api::FundDaily => API_FUND_DAILY.to_string(),
            Api::FundPortfolio => API_FUND_PORTFOLIO.to_string(),
            Api::Daily => API_DAILY.to_string(),
            Api::DailyBasic => API_DAILY_BASIC.to_string(),
            Api::MoneyflowMktDc => API_MONEYFLOW_MKT_DC.to_string(),
            Api::Weekly => API_WEEKLY.to_string(),
            Api::Monthly => API_MONTHLY.to_string(),
            Api::IndexDaily => API_INDEX_DAILY.to_string(),
            Api::IndexWeekly => API_INDEX_WEEKLY.to_string(),
            Api::IndexMonthly => API_INDEX_MONTHLY.to_string(),
            Api::TradeCal => API_TRADE_CAL.to_string(),
            Api::Margin => API_MARGIN.to_string(),
            Api::StockCompany => API_STOCK_COMPANY.to_string(),
            Api::MarginDetail => API_MARGIN_DETAIL.to_string(),
            Api::StkHoldernumber => API_STK_HOLDERNUMBER.to_string(),
            Api::ThsIndex => API_THS_INDEX.to_string(),
            Api::ThsMember => API_THS_MEMBER.to_string(),
            Api::ThsDaily => API_THS_DAILY.to_string(),
            Api::ThsHot => API_THS_HOT.to_string(),
            Api::FinaMainbz => API_FINA_MAINBZ.to_string(),
            Api::FinaMainbzVip => API_FINA_MAINBZ_VIP.to_string(),
            Api::FinaIndicator => API_FINA_INDICATOR.to_string(),
            Api::Balancesheet => API_BALANCESHEET.to_string(),
            Api::Income => API_INCOME.to_string(),
            Api::Cashflow => API_CASHFLOW.to_string(),
            Api::IndexBasic => API_INDEX_BASIC.to_string(),
            Api::IndexDailyBasic => API_INDEX_DAILY_BASIC.to_string(),
            Api::Moneyflow => API_MONEYFLOW.to_string(),
            Api::MoneyflowIndustryThs => API_MONEYFLOW_INDUSTRY_THS.to_string(),
            Api::UsBasic => API_US_BASIC.to_string(),
            Api::UsDaily => API_US_DAILY.to_string(),
            Api::Custom(name) => name.clone(),
        }
    }
}

impl<'de> Deserialize<'de> for Api {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct ApiVisitor;

        impl<'de> de::Visitor<'de> for ApiVisitor {
            type Value = Api;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string API name")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if let Some(api) = Api::from_api_str(value) {
                    return Ok(api);
                }
                Ok(Api::Custom(value.to_string()))
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                self.visit_str(&value)
            }
        }

        deserializer.deserialize_str(ApiVisitor)
    }
}

/// Serialize Api enum to string
pub fn serialize_api_name<S>(api: &Api, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&api.name())
}
