use tushare_api::{TushareResponse, TushareData};
use serde_json::json;

// 使用更新后的过程宏定义股票结构体
#[derive(Debug, Clone)]
pub struct Stock {
    pub ts_code: String,
    pub symbol: String,
    pub name: String,
    pub area: Option<String>,
}

// 手动实现 FromTushareData trait 用于演示
impl tushare_api::traits::FromTushareData for Stock {
    fn from_row(
        fields: &[String],
        values: &[serde_json::Value],
    ) -> Result<Self, tushare_api::error::TushareError> {
        Ok(Self {
            ts_code: tushare_api::utils::get_string_field(fields, values, "ts_code")?,
            symbol: tushare_api::utils::get_string_field(fields, values, "symbol")?,
            name: tushare_api::utils::get_string_field(fields, values, "name")?,
            area: tushare_api::utils::get_optional_string_field(fields, values, "area")?,
        })
    }
}

// 生成包含分页信息的 StockList
#[derive(Debug, Clone)]
pub struct StockList {
    /// The list of converted data items
    pub items: Vec<Stock>,
    /// Whether there are more pages available
    pub has_more: bool,
    /// Total number of records available
    pub count: i64,
}

impl StockList {
    /// Create a new instance with items and pagination info
    pub fn new(items: Vec<Stock>, has_more: bool, count: i64) -> Self {
        Self { items, has_more, count }
    }
    
    /// Get the items as a slice
    pub fn items(&self) -> &[Stock] {
        &self.items
    }
    
    /// Check if there are more pages available
    pub fn has_more(&self) -> bool {
        self.has_more
    }
    
    /// Get the total count of records
    pub fn count(&self) -> i64 {
        self.count
    }
    
    /// Get the number of items in current page
    pub fn len(&self) -> usize {
        self.items.len()
    }
    
    /// Check if the current page is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl std::ops::Deref for StockList {
    type Target = Vec<Stock>;
    
    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl IntoIterator for StockList {
    type Item = Stock;
    type IntoIter = std::vec::IntoIter<Stock>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<'a> IntoIterator for &'a StockList {
    type Item = &'a Stock;
    type IntoIter = std::slice::Iter<'a, Stock>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

impl<'a> IntoIterator for &'a mut StockList {
    type Item = &'a mut Stock;
    type IntoIter = std::slice::IterMut<'a, Stock>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.items.iter_mut()
    }
}

impl TryFrom<TushareResponse> for StockList {
    type Error = tushare_api::error::TushareError;
    
    fn try_from(response: TushareResponse) -> Result<Self, Self::Error> {
        let items = tushare_api::utils::response_to_vec::<Stock>(response.clone())?;
        Ok(StockList {
            items,
            has_more: response.data.has_more,
            count: response.data.count,
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建模拟的 TushareResponse 数据，包含分页信息
    let mock_response = TushareResponse {
        request_id: "test_123".to_string(),
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
                    json!("深圳"),
                ],
            ],
            has_more: true,  // 还有更多页面
            count: 5000,     // 总共5000条记录
        },
    };

    // 转换为 StockList
    let stock_list: StockList = mock_response.try_into()?;

    println!("=== 分页信息演示 ===");
    println!("当前页股票数量: {}", stock_list.len());
    println!("是否还有更多页面: {}", stock_list.has_more());
    println!("总记录数: {}", stock_list.count());
    println!();

    println!("=== 股票列表 ===");
    for stock in &stock_list {
        println!("{}: {} ({})", 
                 stock.ts_code, 
                 stock.name, 
                 stock.area.as_deref().unwrap_or("未知"));
    }

    println!();
    println!("=== 通过方法访问 ===");
    println!("通过 items() 方法访问: {} 条记录", stock_list.items().len());
    
    Ok(())
}
