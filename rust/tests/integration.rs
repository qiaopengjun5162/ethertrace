// tests/integration.rs
use ethers::types::{Address, BlockNumber, FilterBlockOption};
use std::str::FromStr;
use rust::client::EthClient;
use rust::event_parser::EventParser;

#[tokio::test]
async fn test_client_and_parser_integration() {
    let client = EthClient::new("https://rpc.mevblocker.io").await.unwrap();
    let parser = EventParser::new("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1").unwrap();
    let addresses = vec![Address::from_str("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1").unwrap()];
    let block_option = FilterBlockOption::Range {
        from_block: Some(BlockNumber::Number(20483831.into())),
        to_block: Some(BlockNumber::Number(20483833.into())),
    };
    let logs = client.get_logs(block_option, Some(addresses), Some(vec![parser.event_hash()])).await.unwrap();
    let results = parser.parse_logs(logs).unwrap();
    assert!(!results.is_empty());
}

