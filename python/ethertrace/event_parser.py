from web3 import Web3
from eth_abi import decode
import logging

logger = logging.getLogger(__name__)

class EventParser:
    def __init__(self, contract_address: str):
        """初始化 ConfirmDataStore 事件解析器"""
        self.contract_address = Web3.to_checksum_address(contract_address)
        self.event_signature = Web3.keccak(text="ConfirmDataStore(uint32,bytes32)").hex()
        self.abi_types = ["uint32", "bytes32"]
        self.abi_names = ["dataStoreId", "headerHash"]

    def parse_logs(self, logs: list) -> list:
        """解析日志并提取 ConfirmDataStore 事件数据"""
        results = []
        for log in logs:
            # 过滤合约地址和事件签名
            if log["address"].lower() != self.contract_address.lower():
                continue
            if not log.get("topics") or log["topics"][0].hex() != self.event_signature:
                continue

            # 解码日志数据
            try:
                decoded_data = decode(self.abi_types, log["data"])
                result = dict(zip(self.abi_names, decoded_data))
                results.append({
                    "dataStoreId": result["dataStoreId"],
                    "headerHash": Web3.to_hex(result["headerHash"])
                })
            except Exception as e:
                logger.error(f"Failed to unpack log data: {e}")
                continue

        return results