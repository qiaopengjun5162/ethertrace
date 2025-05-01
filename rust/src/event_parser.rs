use ethers::{
    abi::{Event, EventParam, ParamType, RawLog},
    types::{Address, H256, Log},
};
use std::str::FromStr;
use anyhow::anyhow;

#[derive(Debug, Clone)]
pub struct ConfirmDataStoreData {
    pub data_store_id: u32,
    pub header_hash: H256,
}

#[derive(Debug)]
pub struct EventParser {
    event_abi: Event,
    event_hash: H256,
    contract_addr: Address,
}

impl EventParser {
    pub fn new(contract_addr: &str) -> Result<Self, anyhow::Error> {
        // 验证合约地址
        if contract_addr.is_empty() {
            return Err(anyhow::anyhow!("Contract address cannot be empty"));
        }
        if !contract_addr.starts_with("0x") || contract_addr.len() != 42 {
            return Err(anyhow::anyhow!("Invalid contract address format"));
        }

        // 检查是否是有效的十六进制字符串
        let hex_part = &contract_addr[2..];
        if hex_part.as_bytes().iter().any(|b| !b.is_ascii_hexdigit()) {
            return Err(anyhow::anyhow!("Contract address contains invalid hex characters"));
        }

        let addr = Address::from_str(contract_addr)?;
        if addr == Address::zero() {
            return Err(anyhow::anyhow!("Contract address cannot be zero"));
        }

        // 定义 ConfirmDataStore 事件 ABI
        let event_abi = Event {
            name: "ConfirmDataStore".to_string(),
            inputs: vec![
                EventParam {
                    name: "dataStoreId".to_string(),
                    kind: ParamType::Uint(32),
                    indexed: false,
                },
                EventParam {
                    name: "headerHash".to_string(),
                    kind: ParamType::FixedBytes(32),
                    indexed: false,
                },
            ],
            anonymous: false,
        };

        // 计算事件签名哈希
        let event_hash = H256::from(ethers::utils::keccak256("ConfirmDataStore(uint32,bytes32)"));

        Ok(EventParser {
            event_abi,
            event_hash,
            contract_addr: addr,
        })
    }

    pub fn event_hash(&self) -> H256 {
        self.event_hash
    }
    pub fn parse_logs(&self, logs: Vec<Log>) -> Result<Vec<ConfirmDataStoreData>, anyhow::Error> {
        let mut results = Vec::with_capacity(logs.len() / 2);

        for log in logs {
            // 检查日志是否来自正确的合约地址和事件哈希
            if log.address != self.contract_addr ||
                log.topics.get(0) != Some(&self.event_hash) {
                continue;
            }

            let raw_log = RawLog {
                topics: log.topics,
                data: log.data.to_vec(),
            };

            let tokens = self.event_abi.parse_log(raw_log)?;

            let data_store_id = tokens.params.get(0)
                .ok_or(anyhow!("Missing dataStoreId"))?
                .value.clone()
                .into_uint()
                .ok_or(anyhow!("Invalid dataStoreId type"))?
                .as_u32();

            let bytes = tokens.params.get(1)
                .ok_or(anyhow!("Missing headerHash"))?
                .value.clone()
                .into_fixed_bytes()
                .ok_or(anyhow!("Invalid headerHash type"))?;

            // let mut arr = [0u8; 32];
            // arr.copy_from_slice(&bytes);
            // let header_hash = H256::from(arr);
            let header_hash = H256::from_slice(&bytes);

            results.push(ConfirmDataStoreData {
                data_store_id,
                header_hash,
            });
        }
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::types::{Address, H256, Log};
    use std::str::FromStr;

    #[test]
    fn test_new_event_parser() {
        let parser = EventParser::new("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1");
        assert!(parser.is_ok());
    }

    #[test]
    fn test_new_event_parser_invalid_address() {
        let parser = EventParser::new("0xInvalidAddress");
        assert!(parser.is_err());
        assert_eq!(parser.unwrap_err().to_string(), "Invalid contract address format");
    }

    #[test]
    fn test_parse_logs_single_valid_log() {
        let parser = EventParser::new("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1").unwrap();
        let logs = vec![Log {
            address: Address::from_str("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1").unwrap(),
            topics: vec![parser.event_hash()],
            data: hex::decode("00000000000000000000000000000000000000000000000000000000000089ba27bc30064cc44c6aef26ca2d7e4ee667592949a50f4f01d8d4632461a12f2243").unwrap().into(),
            ..Default::default()
        }];
        let results = parser.parse_logs(logs).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].data_store_id, 35258);
        assert_eq!(
            results[0].header_hash,
            H256::from_str("0x27bc30064cc44c6aef26ca2d7e4ee667592949a50f4f01d8d4632461a12f2243").unwrap()
        );
    }

    #[test]
    fn test_parse_logs_empty() {
        let parser = EventParser::new("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1").unwrap();
        let logs = vec![].into();
        let results = parser.parse_logs(logs).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_new_event_parser_missing_prefix() {
        let parser = EventParser::new("InvalidAddress");
        assert!(parser.is_err());
        assert_eq!(parser.unwrap_err().to_string(), "Invalid contract address format");
    }

    #[test]
    fn test_event_hash() {
        let parser = EventParser::new("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1").unwrap();
        let hash = parser.event_hash();
        assert_eq!(
            hash,
            H256::from(ethers::utils::keccak256("ConfirmDataStore(uint32,bytes32)"))
        );
    }

    #[test]
    fn test_parse_logs_non_matching() {
        let parser = EventParser::new("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1").unwrap();
        let logs = vec![Log {
            address: Address::from_str("0x0000000000000000000000000000000000000001").unwrap(),
            topics: vec![H256::zero()],
            data: vec![].into(),
            ..Default::default()
        }];
        let results = parser.parse_logs(logs).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_parse_logs_invalid_data() -> Result<(), anyhow::Error> {
        let parser = EventParser::new("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1").unwrap();
        let logs = vec![Log {
            address: Address::from_str("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1").unwrap(),
            topics: vec![parser.event_hash()],
            data: vec![0u8; 32].into(), // 无效数据
            ..Default::default()
        }];
        let results = parser.parse_logs(logs);
        println!("results: {:#?}", results);
        assert!(results.is_err());
        assert!(results.unwrap_err().to_string().contains("Invalid data"));
        // assert_eq!(results.unwrap_err().to_string(), "Invalid data");
        Ok(())
    }

    #[test]
    fn test_new_event_parser_empty_address() {
        let parser = EventParser::new("");
        assert!(parser.is_err());
        assert_eq!(parser.unwrap_err().to_string(), "Contract address cannot be empty");
    }

    #[test]
    fn test_new_event_parser_invalid_format() {
        let parser = EventParser::new("0xInvalidAddress");
        assert!(parser.is_err());
        assert_eq!(parser.unwrap_err().to_string(), "Invalid contract address format");
    }

    #[test]
    fn test_new_event_parser_invalid_hex() {
        let parser = EventParser::new("0xGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG");
        assert!(parser.is_err());
        assert_eq!(parser.unwrap_err().to_string(), "Contract address contains invalid hex characters");
    }

    #[test]
    fn test_new_event_parser_zero_address() {
        let parser = EventParser::new("0x0000000000000000000000000000000000000000");
        assert!(parser.is_err());
        assert_eq!(parser.unwrap_err().to_string(), "Contract address cannot be zero");
    }

    #[test]
    fn test_parse_logs_multiple_logs_mixed() {
        let parser = EventParser::new("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1").unwrap();
        let logs = vec![
            Log {
                address: Address::from_str("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1").unwrap(),
                topics: vec![parser.event_hash()],
                data: hex::decode("00000000000000000000000000000000000000000000000000000000000089ba27bc30064cc44c6aef26ca2d7e4ee667592949a50f4f01d8d4632461a12f2243").unwrap().into(),
                ..Default::default()
            },
            Log {
                address: Address::from_str("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1").unwrap(),
                topics: vec![H256::zero()], // 非匹配主题
                data: vec![].into(),
                ..Default::default()
            },
            Log {
                address: Address::from_str("0x0000000000000000000000000000000000000001").unwrap(), // 非匹配地址
                topics: vec![parser.event_hash()],
                data: vec![].into(),
                ..Default::default()
            },
            Log {
                address: Address::from_str("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1").unwrap(),
                topics: vec![parser.event_hash()],
                data: hex::decode("00000000000000000000000000000000000000000000000000000000000089bb27bc30064cc44c6aef26ca2d7e4ee667592949a50f4f01d8d4632461a12f2244").unwrap().into(),
                ..Default::default()
            },
        ];
        let results = parser.parse_logs(logs).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].data_store_id, 35258);
        assert_eq!(results[1].data_store_id, 35259);
    }

}