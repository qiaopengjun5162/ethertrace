import pytest
from web3 import Web3
from ethertrace.event_parser import EventParser

CONTRACT_ADDRESS = "0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1"
WRONG_ADDRESS = "0x0000000000000000000000000000000000000000"

@pytest.fixture
def parser():
    """提供 EventParser 实例"""
    return EventParser(CONTRACT_ADDRESS)

def test_parse_logs(parser):
    """测试解析有效日志"""
    log = {
        "address": Web3.to_checksum_address(CONTRACT_ADDRESS),
        "topics": [Web3.keccak(text="ConfirmDataStore(uint32,bytes32)")],
        "data": Web3.to_bytes(hexstr=(
            "0x" +
            "000000000000000000000000000000000000000000000000000000000000007b" +  # uint32: 123
            "0000000000000000000000000000000000000000000000000000000000000123"   # bytes32
        ))
    }
    results = parser.parse_logs([log])
    assert len(results) > 0, f"Expected parsed results, got {results}"
    assert results[0]["dataStoreId"] == 123
    assert results[0]["headerHash"] == "0x0000000000000000000000000000000000000000000000000000000000000123"

def test_parse_logs_wrong_address(parser):
    """测试错误合约地址"""
    log = {
        "address": WRONG_ADDRESS,
        "topics": [Web3.keccak(text="ConfirmDataStore(uint32,bytes32)")],
        "data": Web3.to_bytes(hexstr="0x" + "0" * 128)
    }
    results = parser.parse_logs([log])
    assert len(results) == 0

def test_parse_logs_wrong_topic(parser):
    """测试错误主题"""
    log = {
        "address": Web3.to_checksum_address(CONTRACT_ADDRESS),
        "topics": [Web3.keccak(text="InvalidEvent()")],
        "data": Web3.to_bytes(hexstr="0x" + "0" * 128)
    }
    results = parser.parse_logs([log])
    assert len(results) == 0


def test_parse_logs_invalid_data(parser):
    """测试无效数据"""
    log = {
        "address": Web3.to_checksum_address(CONTRACT_ADDRESS),
        "topics": [Web3.keccak(text="ConfirmDataStore(uint32,bytes32)")],
        "data": Web3.to_bytes(hexstr="0x1234")  # 无效长度
    }
    results = parser.parse_logs([log])
    assert len(results) == 0