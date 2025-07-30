# 如何将 TushareResponse 转换为 Vec<Stock>

本指南展示了如何将 Tushare API 返回的 JSON 数据转换为结构化的 Rust 类型。

## JSON 数据格式

Tushare API 返回的数据格式如下：

```json
{
    "fields": [
        "ts_code",
        "symbol", 
        "name"
    ],
    "items": [
        [
            "000001.SZ",
            "000001",
            "平安银行"
        ],
        [
            "000002.SZ", 
            "000002",
            "万科A"
        ]
    ]
}
```

## 方法一：使用 FromTushareData trait（推荐）

```rust
use tushare_api::{TushareClient, TushareResponse, TushareError, Api, request, FromTushareData, get_string_field};
use serde_json::Value;

#[derive(Debug, Clone)]
struct Stock {
    ts_code: String,
    symbol: String,
    name: String,
}

impl FromTushareData for Stock {
    fn from_row(fields: &[String], values: &[Value]) -> Result<Self, TushareError> {
        Ok(Stock {
            ts_code: get_string_field(fields, values, "ts_code")?,
            symbol: get_string_field(fields, values, "symbol")?,
            name: get_string_field(fields, values, "name")?,
        })
    }
}

// 创建包装类型以避免孤儿规则
#[derive(Debug)]
struct StockList(Vec<Stock>);

impl TryFrom<TushareResponse> for StockList {
    type Error = TushareError;

    fn try_from(response: TushareResponse) -> Result<Self, Self::Error> {
        let stocks: Vec<Stock> = tushare_api::response_to_vec(response)?;
        Ok(StockList(stocks))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TushareClient::from_env()?;
    
    let request = request!(Api::StockBasic, {
        "list_status" => "L"
    }, [
        "ts_code", "symbol", "name"
    ]);
    
    // 使用泛型方法直接获取转换后的类型
    let stock_list: StockList = client.call_api_as(request).await?;
    
    for stock in &stock_list.0 {
        println!("{}: {} - {}", stock.ts_code, stock.symbol, stock.name);
    }
    
    Ok(())
}
```

## 方法二：手动实现 TryFrom

```rust
use tushare_api::{TushareClient, TushareResponse, TushareError, Api, request};

#[derive(Debug, Clone)]
struct Stock {
    ts_code: String,
    symbol: String,
    name: String,
}

#[derive(Debug)]
struct StockList(Vec<Stock>);

impl TryFrom<TushareResponse> for StockList {
    type Error = TushareError;

    fn try_from(response: TushareResponse) -> Result<Self, Self::Error> {
        let mut stocks = Vec::new();
        
        // 查找字段索引
        let ts_code_idx = response.data.fields.iter()
            .position(|f| f == "ts_code")
            .ok_or_else(|| TushareError::ParseError("Missing ts_code field".to_string()))?;
            
        let symbol_idx = response.data.fields.iter()
            .position(|f| f == "symbol")
            .ok_or_else(|| TushareError::ParseError("Missing symbol field".to_string()))?;
            
        let name_idx = response.data.fields.iter()
            .position(|f| f == "name")
            .ok_or_else(|| TushareError::ParseError("Missing name field".to_string()))?;

        // 转换每一行数据
        for item in response.data.items {
            if item.len() <= ts_code_idx || item.len() <= symbol_idx || item.len() <= name_idx {
                return Err(TushareError::ParseError("Item has insufficient fields".to_string()));
            }

            let ts_code = item[ts_code_idx].as_str()
                .ok_or_else(|| TushareError::ParseError("ts_code is not a string".to_string()))?
                .to_string();
                
            let symbol = item[symbol_idx].as_str()
                .ok_or_else(|| TushareError::ParseError("symbol is not a string".to_string()))?
                .to_string();
                
            let name = item[name_idx].as_str()
                .ok_or_else(|| TushareError::ParseError("name is not a string".to_string()))?
                .to_string();

            stocks.push(Stock {
                ts_code,
                symbol,
                name,
            });
        }

        Ok(StockList(stocks))
    }
}
```

## 方法三：使用传统方式 + 工具函数

```rust
use tushare_api::{TushareClient, Api, request, response_to_vec, FromTushareData, get_string_field};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TushareClient::from_env()?;
    
    let request = request!(Api::StockBasic, {
        "list_status" => "L"
    }, [
        "ts_code", "symbol", "name"
    ]);
    
    // 先获取原始响应
    let response = client.call_api(request).await?;
    
    // 然后使用工具函数转换
    let stocks: Vec<Stock> = response_to_vec(response)?;
    
    for stock in &stocks {
        println!("{}: {} - {}", stock.ts_code, stock.symbol, stock.name);
    }
    
    Ok(())
}
```

## 关键要点

1. **使用 FromTushareData trait**：这是最推荐的方式，提供了类型安全和错误处理
2. **避免孤儿规则**：不能直接为 `Vec<Stock>` 实现 `TryFrom<TushareResponse>`，需要创建包装类型
3. **字段映射**：使用字段名而不是索引来访问数据，更加健壮
4. **错误处理**：使用 `TushareError::ParseError` 来处理数据解析错误
5. **工具函数**：库提供了 `get_string_field`、`get_float_field` 等辅助函数

## 支持的工具函数

- `get_string_field()` - 获取字符串字段
- `get_optional_string_field()` - 获取可选字符串字段
- `get_float_field()` - 获取浮点数字段
- `get_optional_float_field()` - 获取可选浮点数字段
- `response_to_vec()` - 将响应转换为向量
