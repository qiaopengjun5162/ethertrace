package main

import (
	"fmt"
	"math/big"

	"github.com/ethereum/go-ethereum/common"

	"github.com/qiaopengjun5162/ethertrace/go/ethertrace"
)

const (
	rpcURL        = "https://rpc.mevblocker.io"
	contractAddr  = "0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1"
	txHashExample = "0xfd26d40e17213bcafcf94bab9af92343302df9df970f20e1c9d515525e86e23e"
	startBlock    = 20483831
	endBlock      = 20483833
)

func main() {
	// 初始化客户端
	client, err := ethertrace.NewEthClient(rpcURL)
	if err != nil {
		fmt.Printf("Failed to create eth client: %v\n", err)
		return
	}
	defer client.Close()

	// 初始化事件解析器
	parser, err := ethertrace.NewEventParser(contractAddr)
	if err != nil {
		fmt.Printf("Failed to create event parser: %v\n", err)
		return
	}

	// 示例 1: 获取交易收据并解析日志
	receipt, err := client.GetTxReceiptByHash(txHashExample)
	if err != nil {
		fmt.Printf("Failed to get tx receipt: %v\n", err)
		return
	}
	txResults, err := parser.ParseLogs(receipt.Logs)
	if err != nil {
		fmt.Printf("Failed to parse tx logs: %v\n", err)
		return
	}
	for _, result := range txResults {
		fmt.Printf("Tx Receipt - DataStoreID: %d, HeaderHash: %s\n", result.DataStoreID, result.HeaderHash.Hex())
	}

	// 示例 2: 获取区块范围内的日志并解析
	logs, err := client.GetLogs(big.NewInt(int64(startBlock)), big.NewInt(int64(endBlock)), []common.Address{common.HexToAddress(contractAddr)})
	if err != nil {
		fmt.Printf("Failed to get logs: %v\n", err)
		return
	}
	logResults, err := parser.ParseLogs(logs)
	if err != nil {
		fmt.Printf("Failed to parse logs: %v\n", err)
		return
	}
	for _, result := range logResults {
		fmt.Printf("Logs - DataStoreID: %d, HeaderHash: %s\n", result.DataStoreID, result.HeaderHash.Hex())
	}
}
