# Tushare API - Rust 库

> ⚠️ **开发状态**: 本项目还在开发中，API 可能会发生变化。请谨慎在生产环境中使用。

这是一个使用 Rust 编写的库，提供通过 Tushare API 获取 A 股股票数据的功能。

## 功能特性

- 获取所有上市状态的 A 股股票信息
- 显示股票代码、简称、公司名称、地区、行业、市场和上市日期
- 异步处理，高效的网络请求
- 错误处理和用户友好的提示信息

## 前置要求

1. **Rust 环境**: 确保已安装 Rust (推荐使用 rustup)
2. **Tushare API Token**: 需要在 [Tushare官网](https://tushare.pro/) 注册并获取 API Token

## 安装和使用

### 1. 作为依赖添加到您的项目

在您的 `Cargo.toml` 文件中添加：

```toml
[dependencies]
tushare-api = { path = "path/to/tushare-api" }
# 或者如果发布到 crates.io:
# tushare-api = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

### 2. 基本使用示例

```rust
use tushare_api::{TushareClient, Stock};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建客户端
    let client = TushareClient::new("your_token_here");
    
    // 获取股票列表
    let stocks = client.get_stock_list().await?;
    println!("获取到 {} 只股票", stocks.len());
    
    // 获取特定股票信息
    if let Some(stock) = client.get_stock_by_code("000001.SZ").await? {
        println!("股票: {} - {}", stock.ts_code, stock.name);
    }
    
    Ok(())
}
```

### 3. 运行示例

项目包含一个完整的使用示例，您可以这样运行：

```bash
# 设置环境变量
set TUSHARE_TOKEN=your_actual_token_here  # Windows
# 或
export TUSHARE_TOKEN=your_actual_token_here  # Linux/Mac

# 运行示例
cargo run --example basic_usage
```

## 输出示例

程序运行后会显示类似以下的输出：

```
正在获取A股股票列表...
成功获取到 4000+ 只A股股票:
股票代码      简称     公司名称              地区     行业            市场     上市日期  
------------------------------------------------------------------------------------------
000001.SZ    平安银行  平安银行股份有限公司      深圳     银行            主板     19910403
000002.SZ    万科A    万科企业股份有限公司      深圳     房地产开发       主板     19910129
...
```

## API 文档

### TushareClient

主要的客户端结构体，用于与 Tushare API 交互。

#### 方法

- `new(token: &str) -> Self`: 创建新的客户端实例
- `get_stock_list() -> Result<Vec<Stock>, Box<dyn std::error::Error>>`: 获取所有上市状态的 A 股股票列表
- `get_stock_by_code(ts_code: &str) -> Result<Option<Stock>, Box<dyn std::error::Error>>`: 根据股票代码获取特定股票信息

### Stock

股票信息结构体，包含以下字段：

- `ts_code`: 股票代码（如：000001.SZ）
- `symbol`: 股票简称
- `name`: 公司名称
- `area`: 地区
- `industry`: 行业
- `market`: 市场类型
- `list_date`: 上市日期

## 依赖项

- `reqwest`: HTTP 客户端库，用于发送 API 请求
- `tokio`: 异步运行时
- `serde`: 序列化/反序列化库
- `serde_json`: JSON 处理

## API 说明

本项目使用 Tushare 的 `stock_basic` 接口获取股票基础信息：

- **接口**: `stock_basic`
- **参数**: `list_status=L` (获取上市状态的股票)
- **字段**: `ts_code,symbol,name,area,industry,market,list_date`

## 错误处理

程序包含完善的错误处理机制：

- 检查环境变量是否设置
- 处理网络请求错误
- 处理 API 响应错误
- 提供用户友好的错误信息

## 注意事项

1. 请确保您的 Tushare API Token 有效且有足够的调用次数
2. 免费用户可能有 API 调用频率限制
3. 程序默认只显示前 20 条记录以避免输出过多内容

## 许可证

MIT License
