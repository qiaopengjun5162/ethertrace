# Ethertrace

## 实操笔记
```rust
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // 初始化控制台日志
    tracing_subscriber::fmt::init();

    // 加载 .env 文件
    dotenv().ok();

    // 从环境变量获取 RPC URL
    let rpc_url = env::var("RPC_URL").expect("RPC_URL environment variable not set");
    println!("RPC URL: {}", rpc_url);

    // 加载配置
    let config = load_config().map_err(|e| anyhow::anyhow!("Failed to load config: {}", e))?;
    info!("Loaded config: {:?}", config);

    // 初始化客户端
    let client = EthClient::new(&config.rpc_url)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to initialize client: {}", e))?;
    println!("Connected to Ethereum node at {}", rpc_url);
    info!("Connected to Ethereum node at {}", config.rpc_url);

    // 初始化事件解析器
    let parser = EventParser::new(&config.contract_addr)
        .map_err(|e| anyhow::anyhow!("Failed to initialize parser: {}", e))?;

    // 示例 1: 获取交易收据并解析日志
    let receipt = client
        .get_tx_receipt_by_hash(&config.tx_hash_example)
        .await
        .map_err(|e| {
            anyhow::anyhow!(
                "Failed to get receipt for tx {}: {}",
                config.tx_hash_example,
                e
            )
        })?;
    debug!(
        "Receipt details: logs_len={}, tx_hash={:?}, tx_index={:?}, block_hash={:?}, block_number={:?}",
        receipt.logs.len(),
        receipt.transaction_hash,
        receipt.transaction_index,
        receipt.block_hash,
        receipt.block_number
    );

    let tx_results = parser.parse_logs(receipt.logs)?;
    if tx_results.is_empty() {
        info!(
            "No ConfirmDataStore events found in transaction {}",
            config.tx_hash_example
        );
    } else {
        for (i, result) in tx_results.iter().enumerate() {
            info!(
                "Tx Receipt {} - DataStoreID: {}, HeaderHash: 0x{}",
                i + 1,
                result.data_store_id,
                result.header_hash
            );
        }
    }

    // 示例 2: 获取区块范围内的日志并解析
    let addresses = vec![Address::from_str(&config.contract_addr)?];
    let block_option = FilterBlockOption::Range {
        from_block: Some(BlockNumber::Number(config.start_block.into())),
        to_block: Some(BlockNumber::Number(config.end_block.into())),
    };

    let topics = vec![parser.event_hash()];
    let logs = client
        .get_logs(block_option, Some(addresses), Some(topics))
        .await
        .map_err(|e| {
            anyhow::anyhow!(
                "Failed to get logs for blocks {} to {}: {}",
                config.start_block,
                config.end_block,
                e
            )
        })?;

    debug!("Retrieved {} logs", logs.len());
    let log_results = parser.parse_logs(logs)?;
    if log_results.is_empty() {
        info!(
            "No ConfirmDataStore events found in blocks {} to {}",
            config.start_block, config.end_block
        );
    } else {
        for (i, result) in log_results.iter().enumerate() {
            info!(
                "Log {} - DataStoreID: {}, HeaderHash: 0x{}",
                i + 1,
                result.data_store_id,
                result.header_hash
            );
        }
    }

    Ok(())
}

```