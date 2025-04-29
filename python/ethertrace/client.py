from web3 import Web3
import logging

logger = logging.getLogger(__name__)

import yaml

def load_config(config_path: str) -> dict:
    with open(config_path, "r") as f:
        return yaml.safe_load(f)

class EthClient:
    def __init__(self, rpc_url: str):
        """初始化以太坊客户端"""
        try:
            self.w3 = Web3(Web3.HTTPProvider(rpc_url))
            if not self.w3.is_connected():
                raise ConnectionError("Failed to connect to Ethereum node")
        except Exception as e:
            logger.error(f"Failed to connect to Ethereum node at {rpc_url}: {e}")
            raise

    def get_tx_receipt(self, tx_hash: str):
        """获取交易收据"""
        try:
            receipt = self.w3.eth.get_transaction_receipt(tx_hash)
            return receipt
        except Exception as e:
            logger.error(f"Failed to get transaction receipt for {tx_hash}: {e}")
            raise

    def get_logs(self, start_block: int, end_block: int, addresses: list):
        """获取指定区块范围内的日志"""
        try:
            filter_params = {
                "fromBlock": start_block,
                "toBlock": end_block,
                "address": addresses
            }
            logs = self.w3.eth.get_logs(filter_params)
            return logs
        except Exception as e:
            logger.error(f"Failed to get logs from {start_block} to {end_block}: {e}")
            raise