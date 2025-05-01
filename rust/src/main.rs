#![feature(coverage_attribute)]

use dotenv::dotenv;
use ethers::types::{Address, BlockNumber, FilterBlockOption};
use log::{debug, info};
use serde::Deserialize;
use std::str::FromStr;
use std::{env, fs};
use tracing::instrument;

mod client;
mod event_parser;

use client::EthClient;
use event_parser::EventParser;

#[derive(Debug, Deserialize)]
struct Config {
    rpc_url: String,
    contract_addr: String,
    tx_hash_example: String,
    start_block: u64,
    end_block: u64,
}

trait ConfigLoader {
    fn load(&self) -> Result<Config, anyhow::Error>;
}

struct YamlConfigLoader;
impl ConfigLoader for YamlConfigLoader {
    fn load(&self) -> Result<Config, anyhow::Error> {
        let content = fs::read_to_string("config.yaml")?;
        serde_yaml::from_str(&content).map_err(Into::into)
    }
}

struct EnvConfigLoader;
impl ConfigLoader for EnvConfigLoader {
    fn load(&self) -> Result<Config, anyhow::Error> {
        dotenv().ok();
        Ok(Config {
            rpc_url: env::var("RPC_URL").map_err(|_| anyhow::anyhow!("RPC_URL not set in .env"))?,
            contract_addr: env::var("CONTRACT_ADDR")
                .unwrap_or("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1".to_string()),
            tx_hash_example: env::var("TX_HASH_EXAMPLE").unwrap_or(
                "0xfd26d40e17213bcafcf94bab9af92343302df9df970f20e1c9d515525e86e23e".to_string(),
            ),
            start_block: env::var("START_BLOCK")
                .map(|s| s.parse::<u64>().expect("Invalid START_BLOCK"))
                .unwrap_or(20483831),
            end_block: env::var("END_BLOCK")
                .map(|s| s.parse::<u64>().expect("Invalid END_BLOCK"))
                .unwrap_or(20483833),
        })
    }
}

fn load_config() -> Result<Config, anyhow::Error> {
    let loader = YamlConfigLoader;
    loader.load().or_else(|_| EnvConfigLoader.load())
}

#[instrument(skip(config, client, parser))]
async fn run(config: Config, client: EthClient, parser: EventParser) -> Result<(), anyhow::Error> {
    info!("Connected to Ethereum node at {}", config.rpc_url);

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
                hex::encode(result.header_hash)
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
        .await?;

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
                hex::encode(result.header_hash)
            );
        }
    }

    Ok(())
}

#[tokio::main]
#[coverage(off)]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();
    let config = load_config().map_err(|e| anyhow::anyhow!("Failed to load config: {}", e))?;
    info!("Loaded config: {:?}", config);

    let client = EthClient::new(&config.rpc_url)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to initialize client: {}", e))?;
    let parser = EventParser::new(&config.contract_addr)
        .map_err(|e| anyhow::anyhow!("Failed to initialize parser: {}", e))?;

    run(config, client, parser).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_main_run() {
        let config = Config {
            rpc_url: "https://rpc.mevblocker.io".to_string(),
            contract_addr: String::from("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1"),
            tx_hash_example: String::from(
                "0xfd26d40e17213bcafcf94bab9af92343302df9df970f20e1c9d515525e86e23e",
            ),
            start_block: 20483831,
            end_block: 20483833,
        };
        let client = EthClient::new(&config.rpc_url).await.unwrap();
        let parser = EventParser::new(&config.contract_addr).unwrap();
        let result = run(config, client, parser).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_load_config_from_yaml() {
        let yaml_content = r#"
            rpc_url: "https://rpc.yaml.io"
            contract_addr: "0xabcdef1234567890abcdef1234567890abcdef12"
            tx_hash_example: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
            start_block: 5000
            end_block: 6000
        "#;
        fs::write("config.yaml", yaml_content).unwrap();
        let config = YamlConfigLoader.load().unwrap();
        println!("config: {:#?}", config);
        assert_eq!(config.rpc_url, "https://rpc.yaml.io");
        assert_eq!(
            config.contract_addr,
            "0xabcdef1234567890abcdef1234567890abcdef12"
        );
        assert_eq!(config.start_block, 5000);
        assert_eq!(config.end_block, 6000);
        fs::remove_file("config.yaml").unwrap();
    }

    #[tokio::test]
    async fn test_run_empty_receipt() {
        let config = Config {
            rpc_url: "https://rpc.mevblocker.io".to_string(),
            contract_addr: "0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1".to_string(),
            tx_hash_example: "0x0000000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
            start_block: 20483831,
            end_block: 20483833,
        };
        let client = EthClient::new(&config.rpc_url).await.unwrap();
        let parser = EventParser::new(&config.contract_addr).unwrap();
        let result = run(config, client, parser).await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Transaction receipt not found")
        );
    }

    #[tokio::test]
    async fn test_run_valid_receipt() {
        let config = Config {
            rpc_url: "https://rpc.mevblocker.io".to_string(),
            contract_addr: "0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1".to_string(),
            tx_hash_example: "0xfd26d40e17213bcafcf94bab9af92343302df9df970f20e1c9d515525e86e23e"
                .to_string(),
            start_block: 20483831,
            end_block: 20483833,
        };
        let client = EthClient::new(&config.rpc_url).await.unwrap();
        let parser = EventParser::new(&config.contract_addr).unwrap();
        let result = run(config, client, parser).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_run_empty_logs() {
        let config = Config {
            rpc_url: "https://rpc.mevblocker.io".to_string(),
            contract_addr: "0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1".to_string(),
            tx_hash_example: "0x0000000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
            start_block: 0,
            end_block: 0,
        };
        let client = EthClient::new(&config.rpc_url).await.unwrap();
        let parser = EventParser::new(&config.contract_addr).unwrap();
        let result = run(config, client, parser).await;

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Failed to get receipt for tx")
        );
    }

    #[test]
    fn test_load_config_invalid_yaml() {
        let yaml_content = r#"
            rpc_url: "https://rpc.yaml.io"
            contract_addr: "0xabcdef1234567890abcdef1234567890abcdef12"
            tx_hash_example: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
            start_block: "invalid" # 无效的 u64
        "#;
        fs::write("config.yaml", yaml_content).unwrap();
        let result = YamlConfigLoader.load();
        println!("result: {:#?}", result);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("invalid type"));
        fs::remove_file("config.yaml").unwrap();
    }

    #[test]
    fn test_load_config() {
        let result = load_config();
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_run_get_logs_error() {
        let config = Config {
            rpc_url: "https://invalid-rpc.example.com".to_string(),
            contract_addr: "0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1".to_string(),
            tx_hash_example: "0x0000000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
            start_block: 20483831,
            end_block: 20483833,
        };
        let client = EthClient::new(&config.rpc_url).await.unwrap();
        let parser = EventParser::new(&config.contract_addr).unwrap();
        let result = run(config, client, parser).await;
        println!("result: {:#?}", result);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Failed to get receipt")
        );
    }
}
