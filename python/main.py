import logging
from ethertrace.client import EthClient
from ethertrace.event_parser import EventParser

# 配置日志
logging.basicConfig(level=logging.INFO, format="%(asctime)s - %(levelname)s - %(message)s")

# 常量
RPC_URL = "https://rpc.mevblocker.io"
CONTRACT_ADDRESS = "0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1"
TX_HASH_EXAMPLE = "0xfd26d40e17213bcafcf94bab9af92343302df9df970f20e1c9d515525e86e23e"
START_BLOCK = 20483831
END_BLOCK = 20483833

def main():
    # 初始化客户端
    try:
        client = EthClient(RPC_URL)
    except Exception as e:
        logging.error(f"Failed to initialize client: {e}")
        return

    # 初始化事件解析器
    parser = EventParser(CONTRACT_ADDRESS)

    # 示例 1: 获取交易收据并解析日志
    try:
        receipt = client.get_tx_receipt(TX_HASH_EXAMPLE)
        tx_results = parser.parse_logs(receipt["logs"])
        for result in tx_results:
            logging.info(f"Tx Receipt - DataStoreID: {result['dataStoreId']}, HeaderHash: {result['headerHash']}")
    except Exception as e:
        logging.error(f"Failed to process tx receipt: {e}")

    # 示例 2: 获取区块范围内的日志并解析
    try:
        logs = client.get_logs(START_BLOCK, END_BLOCK, [CONTRACT_ADDRESS])
        log_results = parser.parse_logs(logs)
        for result in log_results:
            logging.info(f"Logs - DataStoreID: {result['dataStoreId']}, HeaderHash: {result['headerHash']}")
    except Exception as e:
        logging.error(f"Failed to process logs: {e}")

if __name__ == "__main__":
    main()