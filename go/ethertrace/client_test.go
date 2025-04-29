package ethertrace

import (
	"fmt"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	"math/big"
	"strings"
	"testing"

	"github.com/ethereum/go-ethereum/common"
)

func TestEthClient_GetTxReceiptByHash(t *testing.T) {
	client, err := NewEthClient("https://eth.llamarpc.com")
	if err != nil {
		t.Fatalf("Failed to create client: %v", err)
	}
	defer client.Close()

	txHash := "0xfd26d40e17213bcafcf94bab9af92343302df9df970f20e1c9d515525e86e23e"
	receipt, err := client.GetTxReceiptByHash(txHash)
	if err != nil {
		t.Fatalf("Failed to get receipt: %v", err)
	}
	if receipt == nil {
		t.Fatal("Receipt is nil")
	}
	fmt.Println("receipt:", receipt)
}

func TestEthClient_GetLogs(t *testing.T) {
	client, err := NewEthClient("https://rpc.mevblocker.io")
	if err != nil {
		t.Fatalf("Failed to create client: %v", err)
	}
	defer client.Close()

	logs, err := client.GetLogs(big.NewInt(20483831), big.NewInt(20483833), []common.Address{common.HexToAddress("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1")})
	if err != nil {
		t.Fatalf("Failed to get logs: %v", err)
	}
	if len(logs) == 0 {
		t.Log("No logs found, but test passes as query was successful")
	}
	fmt.Println("logs: ", logs)
}

func TestEthClient_GetTxReceiptByHash2(t *testing.T) {
	client, err := NewEthClient("https://eth.llamarpc.com") // 替换为测试节点
	if err != nil {
		t.Fatalf("创建 EthClient 失败: %v", err)
	}

	tests := []struct {
		name        string
		txHash      string
		expectError string
	}{
		{
			name:        "有效交易哈希",
			txHash:      "0xfd26d40e17213bcafcf94bab9af92343302df9df970f20e1c9d515525e86e23e",
			expectError: "",
		},
		{
			name:        "无效交易哈希",
			txHash:      "0x0000000000000000000000000000000000000000000000000000000000000000",
			expectError: "not found", // 假设节点返回此错误
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			_, err := client.GetTxReceiptByHash(tt.txHash)
			if tt.expectError == "" {
				if err != nil {
					t.Errorf("GetTxReceiptByHash 期望无错误，实际错误: %v", err)
				}
			} else {
				if err == nil || !strings.Contains(err.Error(), tt.expectError) {
					t.Errorf("GetTxReceiptByHash 期望错误包含 %q，实际错误: %v", tt.expectError, err)
				}
			}
		})
	}
}

func TestNewEthClient(t *testing.T) {
	client, err := NewEthClient("https://rpc.mevblocker.io")
	if err != nil {
		t.Fatalf("Failed to create client: %v", err)
	}
	defer client.Close()
}

func TestNewEthClientRpcUrl(t *testing.T) {
	client, err := NewEthClient("")
	t.Log("err:", err)
	assert.EqualError(t, err, "empty RPC URL")
	// 在错误情况下不要操作 client
	if client != nil {
		client.Close() // 如果测试需要，可以手动关闭
	}

}

func TestNewEthClient_InvalidURLs(t *testing.T) {
	tests := []struct {
		name        string
		rpcURL      string
		expectError string
	}{
		{
			name:        "空地址",
			rpcURL:      "",
			expectError: "empty RPC URL",
		},
		{
			name:        "无效协议",
			rpcURL:      "ftp://localhost",
			expectError: "no known transport for URL scheme \"ftp\"",
		},
		{
			name:        "<UNK>",
			rpcURL:      "http://localhost:12w222q",
			expectError: "invalid port",
		},
		{
			name:        "非法端口",
			rpcURL:      "http://localhost:abc",
			expectError: "invalid port",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			client, err := NewEthClient(tt.rpcURL)
			t.Log("client:", client)
			t.Log("err:", err)

			// 验证错误
			require.Error(t, err, "应该返回错误")
			assert.Contains(t, err.Error(), tt.expectError, "错误信息不匹配")

			// 安全处理 client
			if client != nil {
				t.Error("错误情况下应该返回 nil client")
				client.Close()
			}
		})
	}
}

// TestEthClient_GetLogs 测试 GetLogs，包括错误路径
func TestEthClient2(t *testing.T) {
	// 捕获日志
	_, err := NewEthClient("htrpc.mevlocke")
	t.Log("err:", err)
	require.Error(t, err, "应该返回错误")
}

func TestEthClient_GetLogs2(t *testing.T) {
	client, err := NewEthClient("https://rpc.mevblocker.io")
	if err != nil {
		t.Fatalf("Failed to create client: %v", err)
	}
	defer client.Close()

	logs, err := client.GetLogs(big.NewInt(20483831), big.NewInt(0), []common.Address{common.HexToAddress("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1")})
	t.Log("logs: ", logs)
	t.Log("err:", err)
	if len(logs) == 0 {
		t.Log("No logs found, but test passes as query was successful")
	}
	fmt.Println("logs: ", logs)
}
