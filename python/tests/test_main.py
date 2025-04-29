from unittest.mock import Mock, patch
from main import main

@patch("ethertrace.client.EthClient")
@patch("ethertrace.event_parser.EventParser")
def test_main(mock_parser, mock_client):
    mock_client.return_value.get_tx_receipt.return_value = {"logs": []}
    mock_client.return_value.get_logs.return_value = []
    mock_parser.return_value.parse_logs.return_value = []
    main()  # 确保无异常

@patch("ethertrace.client.EthClient")
@patch("ethertrace.event_parser.EventParser")
def test_main_tx_receipt_failure(mock_parser, mock_client):
    mock_client.return_value.get_tx_receipt.side_effect = Exception("Transaction not found")
    mock_client.return_value.get_logs.return_value = []
    mock_parser.return_value.parse_logs.return_value = []
    main()

@patch("ethertrace.client.EthClient")
@patch("ethertrace.event_parser.EventParser")
def test_main_logs_failure(mock_parser, mock_client):
    mock_client.return_value.get_tx_receipt.return_value = {"logs": []}
    mock_client.return_value.get_logs.side_effect = Exception("Logs fetch failed")
    mock_parser.return_value.parse_logs.return_value = []
    main()

