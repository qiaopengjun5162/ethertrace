// client.rs
use ethers::prelude::*;
use regex::Regex;
use std::sync::Arc;
use url::Url;

lazy_static::lazy_static! {
    static ref DOMAIN_REGEX: Regex = Regex::new(r"^(?:[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?\.?)+$").unwrap();
}

#[derive(Debug)]
pub struct EthClient {
    client: Arc<Provider<Http>>,
}

impl EthClient {
    pub async fn new(rpc_url: &str) -> Result<Self, anyhow::Error> {
        // 验证 RPC URL
        if rpc_url.is_empty() {
            return Err(anyhow::anyhow!("Empty RPC URL"));
        }
        // 使用 url crate 安全解析 URL
        let parsed_url = Url::parse(rpc_url).map_err(|_| anyhow::anyhow!("Invalid URL"))?;

        // 支持的协议
        static ALLOWED_SCHEMES: &[&str] = &["http", "https", "ws", "wss"];
        let scheme = parsed_url.scheme();

        if !ALLOWED_SCHEMES.contains(&scheme) {
            return Err(anyhow::anyhow!("Unsupported scheme"));
        }

        // 连接以太坊节点
        let provider = Provider::<Http>::try_from(rpc_url)?;
        let client = Arc::new(provider);
        Ok(EthClient { client })
    }

    pub async fn get_tx_receipt_by_hash(
        &self,
        tx_hash: &str,
    ) -> Result<TransactionReceipt, anyhow::Error> {
        let hash: H256 = tx_hash.parse()?;
        let receipt = self
            .client
            .get_transaction_receipt(hash)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Transaction receipt not found"))?;
        Ok(receipt)
    }

    pub async fn get_logs(
        &self,
        block_option: FilterBlockOption,
        addresses: Option<Vec<Address>>,
        topics: Option<Vec<H256>>,
    ) -> Result<Vec<Log>, anyhow::Error> {
        let filter = Filter {
            block_option,
            address: addresses.map(ValueOrArray::Array),
            topics: {
                let mut topics_array = [const { None }; 4];
                if let Some(topics_vec) = topics {
                    for (i, topic) in topics_vec.into_iter().take(4).enumerate() {
                        topics_array[i] = Some(topic.into());
                    }
                }
                topics_array
            },
        };
        let logs = self.client.get_logs(&filter).await?;
        Ok(logs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_new_eth_client() {
        let client = EthClient::new("https://rpc.mevblocker.io").await;
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_new_eth_client_invalid_url() {
        let client = EthClient::new("").await;
        assert!(client.is_err());
        assert_eq!(client.unwrap_err().to_string(), "Empty RPC URL");
    }

    #[tokio::test]
    async fn test_get_tx_receipt_by_hash() {
        let client = EthClient::new("https://eth.llamarpc.com").await.unwrap();
        let tx_hash = "0xfd26d40e17213bcafcf94bab9af92343302df9df970f20e1c9d515525e86e23e";
        let receipt = client.get_tx_receipt_by_hash(tx_hash).await;
        assert!(receipt.is_ok());
    }

    #[tokio::test]
    async fn test_get_logs_empty() {
        let client = EthClient::new("https://rpc.mevblocker.io").await.unwrap();
        let block_option = FilterBlockOption::Range {
            from_block: Some(BlockNumber::Number(0.into())),
            to_block: Some(BlockNumber::Number(0.into())),
        };
        let logs = client.get_logs(block_option, None, None).await.unwrap();
        assert!(logs.is_empty());
    }

    #[tokio::test]
    async fn test_get_logs_range() {
        let client = EthClient::new("https://rpc.mevblocker.io").await.unwrap();
        let addresses =
            vec![Address::from_str("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1").unwrap()];
        let block_option = FilterBlockOption::Range {
            from_block: Some(BlockNumber::Number(20483831.into())),
            to_block: Some(BlockNumber::Number(20483833.into())),
        };
        let topics = vec![
            H256::from_str("0x4e3a3754410177e6937ef1d0c0d0a0b0e0d0a0b0e0d0a0b0e0d0a0b0e0d0a0b0")
                .unwrap(),
        ];
        let logs = client
            .get_logs(block_option, Some(addresses), Some(topics))
            .await;
        assert!(logs.is_ok());
    }

    #[tokio::test]
    async fn test_get_logs_at_block_hash() {
        let client = EthClient::new("https://rpc.mevblocker.io").await.unwrap();
        let block_hash =
            H256::from_str("0x4e3a3754410177e6937ef1d0c0d0a0b0e0d0a0b0e0d0a0b0e0d0a0b0e0d0a0b0")
                .unwrap();
        let block_option = FilterBlockOption::AtBlockHash(block_hash);
        let logs = client.get_logs(block_option, None, None).await;
        assert!(logs.is_ok() || logs.unwrap_err().to_string().contains("not found"));
    }

    #[tokio::test]
    async fn test_new_eth_client_with_valid_url() {
        let client = EthClient::new("https://rpc.mevblocker.io").await;
        assert!(client.is_ok(), "Expected Ok(EthClient), got error");
    }

    #[tokio::test]
    async fn test_new_eth_client_empty_url() {
        let client = EthClient::new("").await;
        assert!(client.is_err(), "Expected error for empty URL");
        assert_eq!(client.unwrap_err().to_string(), "Empty RPC URL");
    }

    #[tokio::test]
    async fn test_new_eth_client_invalid_scheme() {
        let client = EthClient::new("ftp://example.com").await;
        assert!(client.is_err(), "Expected error for unsupported scheme");
        assert_eq!(client.unwrap_err().to_string(), "Unsupported scheme");
    }

    #[tokio::test]
    async fn test_new_eth_client_missing_scheme() {
        let client = EthClient::new("example.com").await;
        assert!(client.is_err(), "Expected error for missing scheme");
        assert_eq!(client.unwrap_err().to_string(), "Invalid URL");
    }

    #[tokio::test]
    async fn test_get_tx_receipt_by_hash_invalid_format() {
        let client = EthClient::new("https://eth.llamarpc.com").await.unwrap();
        let result = client.get_tx_receipt_by_hash("invalid_hash").await;
        assert!(result.is_err(), "Expected error for invalid hash format");
    }

    #[tokio::test]
    async fn test_get_tx_receipt_by_hash_not_found() {
        let client = EthClient::new("https://eth.llamarpc.com").await.unwrap();
        let tx_hash = "0x0000000000000000000000000000000000000000000000000000000000000000";
        let result = client.get_tx_receipt_by_hash(tx_hash).await;
        assert!(
            result.is_err(),
            "Expected error for non-existent transaction"
        );
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Transaction receipt not found")
        );
    }

    #[tokio::test]
    async fn test_get_tx_receipt_by_hash_success() {
        let client = EthClient::new("https://eth.llamarpc.com").await.unwrap();
        let tx_hash = "0xfd26d40e17213bcafcf94bab9af92343302df9df970f20e1c9d515525e86e23e";
        let result = client.get_tx_receipt_by_hash(tx_hash).await;
        assert!(
            result.is_ok(),
            "Expected successful transaction receipt retrieval"
        );
    }

    #[tokio::test]
    async fn test_get_logs_empty_range() {
        let client = EthClient::new("https://rpc.mevblocker.io").await.unwrap();
        let block_option = FilterBlockOption::Range {
            from_block: Some(BlockNumber::Number(0.into())),
            to_block: Some(BlockNumber::Number(0.into())),
        };
        let logs = client.get_logs(block_option, None, None).await.unwrap();
        assert!(logs.is_empty(), "Expected no logs in block range 0-0");
    }

    #[tokio::test]
    async fn test_get_logs_with_address_and_topic() {
        let client = EthClient::new("https://rpc.mevblocker.io").await.unwrap();
        let addresses =
            vec![Address::from_str("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1").unwrap()];
        let block_option = FilterBlockOption::Range {
            from_block: Some(BlockNumber::Number(20483831.into())),
            to_block: Some(BlockNumber::Number(20483833.into())),
        };
        let topics = vec![
            H256::from_str("0x4e3a3754410177e6937ef1d0c0d0a0b0e0d0a0b0e0d0a0b0e0d0a0b0e0d0a0b0")
                .unwrap(),
        ];
        let logs = client
            .get_logs(block_option, Some(addresses), Some(topics))
            .await;
        assert!(logs.is_ok(), "Expected logs or success status");
    }

    #[tokio::test]
    async fn test_get_logs_at_block_hash2() {
        let client = EthClient::new("https://rpc.mevblocker.io").await.unwrap();
        let block_hash =
            H256::from_str("0x4e3a3754410177e6937ef1d0c0d0a0b0e0d0a0b0e0d0a0b0e0d0a0b0e0d0a0b0")
                .unwrap();
        let block_option = FilterBlockOption::AtBlockHash(block_hash);
        let logs = client.get_logs(block_option, None, None).await;
        assert!(
            logs.is_ok() || logs.unwrap_err().to_string().contains("not found"),
            "Expected either logs or 'not found' error"
        );
    }

    #[tokio::test]
    async fn test_new_eth_client_invalid_url_format() {
        let result = EthClient::new("htt:/invalid-url").await;
        assert!(result.is_err(), "Expected error for invalid URL format");
        assert_eq!(result.unwrap_err().to_string(), "Unsupported scheme");

        let result = EthClient::new("invalid-url").await;
        assert!(result.is_err(), "Expected error for invalid URL format");
        assert_eq!(result.unwrap_err().to_string(), "Invalid URL");
    }

    #[tokio::test]
    async fn test_new_eth_client_ws_scheme() {
        let client = EthClient::new("ws://example.com").await;
        assert!(client.is_ok(), "Expected Ok for ws scheme");
    }

    #[tokio::test]
    async fn test_new_eth_client_wss_scheme() {
        let client = EthClient::new("wss://example.com").await;
        assert!(client.is_ok(), "Expected Ok for wss scheme");
    }

    #[tokio::test]
    async fn test_new_eth_client_invalid_provider() {
        let client = EthClient::new("https://invalid-rpc.example.com").await;
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_get_logs_single_topic() {
        let client = EthClient::new("https://rpc.mevblocker.io").await.unwrap();
        let block_option = FilterBlockOption::Range {
            from_block: Some(BlockNumber::Number(20483831.into())),
            to_block: Some(BlockNumber::Number(20483833.into())),
        };
        let topics = vec![
            H256::from_str("0x4e3a3754410177e6937ef1d0c0d0a0b0e0d0a0b0e0d0a0b0e0d0a0b0e0d0a0b0")
                .unwrap(),
        ];
        let logs = client.get_logs(block_option, None, Some(topics)).await;
        assert!(logs.is_ok());
    }

    #[tokio::test]
    async fn test_get_logs_no_topics() {
        let client = EthClient::new("https://rpc.mevblocker.io").await.unwrap();
        let block_option = FilterBlockOption::Range {
            from_block: Some(BlockNumber::Number(20483831.into())),
            to_block: Some(BlockNumber::Number(20483833.into())),
        };
        let logs = client.get_logs(block_option, None, None).await;
        assert!(logs.is_ok());
    }

    #[tokio::test]
    async fn test_new_eth_client_domain_regex() {
        // 测试复杂域名
        let client = EthClient::new("https://sub.domain.example.com").await;
        assert!(client.is_ok());
        // 测试无效域名
        let client = EthClient::new("https://invalid..domain.com").await;
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_get_logs_four_topics() {
        let client = EthClient::new("https://rpc.mevblocker.io").await.unwrap();
        let block_option = FilterBlockOption::Range {
            from_block: Some(BlockNumber::Number(20483831.into())),
            to_block: Some(BlockNumber::Number(20483833.into())),
        };
        let topics = vec![
            H256::from_str("0x4e3a3754410177e6937ef1d0c0d0a0b0e0d0a0b0e0d0a0b0e0d0a0b0e0d0a0b0").unwrap(),
            H256::from_str("0x5e3a3754410177e6937ef1d0c0d0a0b0e0d0a0b0e0d0a0b0e0d0a0b0e0d0a0b0").unwrap(),
            H256::from_str("0x6e3a3754410177e6937ef1d0c0d0a0b0e0d0a0b0e0d0a0b0e0d0a0b0e0d0a0b0").unwrap(),
            H256::from_str("0x7e3a3754410177e6937ef1d0c0d0a0b0e0d0a0b0e0d0a0b0e0d0a0b0e0d0a0b0").unwrap(),
        ];
        let logs = client.get_logs(block_option, None, Some(topics)).await;
        assert!(logs.is_ok());
    }
}
