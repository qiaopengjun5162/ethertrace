import pytest
from unittest.mock import Mock, patch
from web3 import Web3
from ethertrace.client import EthClient, load_config
from web3.exceptions import Web3Exception

RPC_URL = "https://rpc.mevblocker.io"
INVALID_RPC_URL = "https://invalid-rpc-url"
TX_HASH = "0xfd26d40e17213bcafcf94bab9af92343302df9df970f20e1c9d515525e86e23e"
INVALID_TX_HASH = "0x" + "0" * 64
CONTRACT_ADDRESS = "0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1"
START_BLOCK = 20483831
END_BLOCK = 20483833

@pytest.fixture
def client():
    return EthClient(RPC_URL)

def test_client_initialization_success():
    client = EthClient(RPC_URL)
    assert client.w3.is_connected()

def test_client_initialization_failure():
    with pytest.raises(ConnectionError):
        EthClient(INVALID_RPC_URL)


def test_get_tx_receipt_success(client):
    receipt = client.get_tx_receipt(TX_HASH)
    assert receipt is not None
    assert "logs" in receipt
    assert receipt["transactionHash"].hex() == TX_HASH.lstrip("0x")  # 规范化比较

def test_get_tx_receipt_invalid_hash(client):
    with pytest.raises(Web3Exception):
        client.get_tx_receipt(INVALID_TX_HASH)  # 覆盖行 18-21

@patch("web3.eth.Eth.get_logs")
def test_get_logs_success(mock_get_logs, client):
    mock_log = {
        "address": CONTRACT_ADDRESS,
        "topics": [Web3.keccak(text="ConfirmDataStore(uint32,bytes32)")],
        "data": "0x000000000000000000000000000000000000000000000000000000000000007b" +
                "0000000000000000000000000000000000000000000000000000000000000123"
    }
    mock_get_logs.return_value = [mock_log]
    logs = client.get_logs(START_BLOCK, END_BLOCK, [CONTRACT_ADDRESS])
    print(logs, type(logs))
    assert isinstance(logs, list)
    assert len(logs) == 1
    assert logs[0]["address"] == CONTRACT_ADDRESS

def test_get_logs_invalid_block_range(client):
    with pytest.raises(Web3Exception):
        client.get_logs(END_BLOCK, START_BLOCK, [CONTRACT_ADDRESS])


@patch("yaml.safe_load")
def test_load_config(mock_yaml):
    mock_yaml.return_value = {"rpc_url": "https://rpc.mevblocker.io"}
    with patch("builtins.open", create=True) as mock_open:
        mock_open.return_value.__enter__.return_value = Mock()
        config = load_config("config.yaml")
        assert config == {"rpc_url": "https://rpc.mevblocker.io"}

@patch("yaml.safe_load")
def test_load_config_file_not_found(mock_yaml):
    with patch("builtins.open", side_effect=FileNotFoundError):
        with pytest.raises(FileNotFoundError):
            load_config("config.yaml")

