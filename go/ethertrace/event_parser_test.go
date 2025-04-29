package ethertrace

import (
	"fmt"
	"strings"
	"testing"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
)

func TestEventParser_ParseLogs(t *testing.T) {
	parser, err := NewEventParser("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1")
	if err != nil {
		t.Fatalf("Failed to create parser: %v", err)
	}

	// 模拟日志数据（需根据实际事件数据构造）
	logs := []*types.Log{
		{
			Address: common.HexToAddress("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1"),
			Topics:  []common.Hash{parser.EventHash()}, // 事件哈希
			Data:    common.Hex2Bytes("00000000000000000000000000000000000000000000000000000000000089ba27bc30064cc44c6aef26ca2d7e4ee667592949a50f4f01d8d4632461a12f2243"),
		},
	}

	results, err := parser.ParseLogs(logs)
	if err != nil {
		t.Fatalf("Failed to parse logs: %v", err)
	}
	if len(results) == 0 {
		t.Log("No valid logs parsed, ensure test data is correct")
	}
	fmt.Println("results", results)
}

// 测试 NewEventParser 的各种场景
func TestNewEventParser(t *testing.T) {
	tests := []struct {
		name        string
		address     string
		expectError string
	}{
		{
			name:        "有效地址",
			address:     "0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1",
			expectError: "",
		},
		{
			name:        "无效地址",
			address:     "0xInvalidAddress",
			expectError: "无效的合约地址格式",
		},
		{
			name:        "空地址",
			address:     "",
			expectError: "合约地址不能为空",
		},
		{
			name:        "零地址",
			address:     "0x0000000000000000000000000000000000000000",
			expectError: "合约地址不能为零地址",
		},
		{
			name:        "缺少0x前缀",
			address:     "5BD63a7ECc13b955C4F57e3F12A64c10263C14c1",
			expectError: "无效的合约地址格式",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			parser, err := NewEventParser(tt.address)
			if tt.expectError == "" {
				if err != nil {
					t.Errorf("NewEventParser(%q) 期望无错误，实际错误: %v", tt.address, err)
				}
				if parser == nil || parser.contractAddr != common.HexToAddress(tt.address) {
					t.Errorf("NewEventParser(%q) 返回的解析器不符合预期", tt.address)
				}
			} else {
				if err == nil || !strings.Contains(err.Error(), tt.expectError) {
					t.Errorf("NewEventParser(%q) 期望错误包含 %q，实际错误: %v", tt.address, tt.expectError, err)
				}
				if parser != nil {
					t.Errorf("NewEventParser(%q) 期望返回 nil 解析器，实际返回: %v", tt.address, parser)
				}
			}
		})
	}
}

// 测试单个有效日志
func TestEventParser_ParseLogs_SingleValidLog(t *testing.T) {
	parser, err := NewEventParser("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1")
	if err != nil {
		t.Fatalf("创建 EventParser 失败: %v", err)
	}

	logs := []*types.Log{
		{
			Address: common.HexToAddress("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1"),
			Topics:  []common.Hash{parser.EventHash()},
			Data:    common.Hex2Bytes("00000000000000000000000000000000000000000000000000000000000089ba27bc30064cc44c6aef26ca2d7e4ee667592949a50f4f01d8d4632461a12f2243"),
		},
	}
	expectResult := 1
	validate := func(results []ConfirmDataStoreData) error {
		if len(results) != expectResult {
			return fmt.Errorf("期望 %d 个结果，实际得到 %d 个", expectResult, len(results))
		}
		result := results[0]
		if result.DataStoreID != 35258 {
			return fmt.Errorf("DataStoreID 期望 35258，实际 %d", result.DataStoreID)
		}
		if result.HeaderHash != common.HexToHash("0x27bc30064cc44c6aef26ca2d7e4ee667592949a50f4f01d8d4632461a12f2243") {
			return fmt.Errorf("HeaderHash 不符合预期")
		}
		return nil
	}

	results, err := parser.ParseLogs(logs)
	if err != nil {
		t.Errorf("ParseLogs 期望无错误，实际错误: %v", err)
	}
	if len(results) != expectResult {
		t.Errorf("ParseLogs 期望 %d 个结果，实际得到 %d 个", expectResult, len(results))
	}
	if err := validate(results); err != nil {
		t.Errorf("结果验证失败: %v", err)
	}
	if len(results) > 0 {
		fmt.Printf("测试 '单个有效日志' 的结果: %v\n", results)
	}
}

// 测试空日志列表
func TestEventParser_ParseLogs_EmptyLogs(t *testing.T) {
	parser, err := NewEventParser("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1")
	if err != nil {
		t.Fatalf("创建 EventParser 失败: %v", err)
	}

	logs := []*types.Log{}
	expectResult := 0
	validate := func(results []ConfirmDataStoreData) error {
		if len(results) != expectResult {
			return fmt.Errorf("期望 %d 个结果，实际得到 %d 个", expectResult, len(results))
		}
		return nil
	}

	results, err := parser.ParseLogs(logs)
	if err != nil {
		t.Errorf("ParseLogs 期望无错误，实际错误: %v", err)
	}
	if len(results) != expectResult {
		t.Errorf("ParseLogs 期望 %d 个结果，实际得到 %d 个", expectResult, len(results))
	}
	if err := validate(results); err != nil {
		t.Errorf("结果验证失败: %v", err)
	}
	if len(results) > 0 {
		fmt.Printf("测试 '空日志列表' 的结果: %v\n", results)
	}
}

// 测试无效日志（错误地址）
func TestEventParser_ParseLogs_InvalidAddress(t *testing.T) {
	parser, err := NewEventParser("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1")
	if err != nil {
		t.Fatalf("创建 EventParser 失败: %v", err)
	}

	logs := []*types.Log{
		{
			Address: common.HexToAddress("0xWrongAddress"),
			Topics:  []common.Hash{parser.EventHash()},
			Data:    common.Hex2Bytes("00000000000000000000000000000000000000000000000000000000000089ba27bc30064cc44c6aef26ca2d7e4ee667592949a50f4f01d8d4632461a12f2243"),
		},
	}
	expectResult := 0
	validate := func(results []ConfirmDataStoreData) error {
		if len(results) != expectResult {
			return fmt.Errorf("期望 %d 个结果，实际得到 %d 个", expectResult, len(results))
		}
		return nil
	}

	results, err := parser.ParseLogs(logs)
	if err != nil {
		t.Errorf("ParseLogs 期望无错误，实际错误: %v", err)
	}
	if len(results) != expectResult {
		t.Errorf("ParseLogs 期望 %d 个结果，实际得到 %d 个", expectResult, len(results))
	}
	if err := validate(results); err != nil {
		t.Errorf("结果验证失败: %v", err)
	}
	if len(results) > 0 {
		fmt.Printf("测试 '无效日志（错误地址）' 的结果: %v\n", results)
	}
}

// 测试主题不匹配
func TestEventParser_ParseLogs_TopicMismatch(t *testing.T) {
	parser, err := NewEventParser("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1")
	if err != nil {
		t.Fatalf("创建 EventParser 失败: %v", err)
	}

	logs := []*types.Log{
		{
			Address: common.HexToAddress("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1"),
			Topics:  []common.Hash{common.HexToHash("0xWrongHash")},
			Data:    common.Hex2Bytes("00000000000000000000000000000000000000000000000000000000000089ba27bc30064cc44c6aef26ca2d7e4ee667592949a50f4f01d8d4632461a12f2243"),
		},
	}
	expectResult := 0
	validate := func(results []ConfirmDataStoreData) error {
		if len(results) != expectResult {
			return fmt.Errorf("期望 %d 个结果，实际得到 %d 个", expectResult, len(results))
		}
		return nil
	}

	results, err := parser.ParseLogs(logs)
	if err != nil {
		t.Errorf("ParseLogs 期望无错误，实际错误: %v", err)
	}
	if len(results) != expectResult {
		t.Errorf("ParseLogs 期望 %d 个结果，实际得到 %d 个", expectResult, len(results))
	}
	if err := validate(results); err != nil {
		t.Errorf("结果验证失败: %v", err)
	}
	if len(results) > 0 {
		fmt.Printf("测试 '主题不匹配' 的结果: %v\n", results)
	}
}

// 测试空主题
func TestEventParser_ParseLogs_EmptyTopics(t *testing.T) {
	parser, err := NewEventParser("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1")
	if err != nil {
		t.Fatalf("创建 EventParser 失败: %v", err)
	}

	logs := []*types.Log{
		{
			Address: common.HexToAddress("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1"),
			Topics:  []common.Hash{},
			Data:    common.Hex2Bytes("00000000000000000000000000000000000000000000000000000000000089ba27bc30064cc44c6aef26ca2d7e4ee667592949a50f4f01d8d4632461a12f2243"),
		},
	}
	expectResult := 0
	validate := func(results []ConfirmDataStoreData) error {
		if len(results) != expectResult {
			return fmt.Errorf("期望 %d 个结果，实际得到 %d 个", expectResult, len(results))
		}
		return nil
	}

	results, err := parser.ParseLogs(logs)
	if err != nil {
		t.Errorf("ParseLogs 期望无错误，实际错误: %v", err)
	}
	if len(results) != expectResult {
		t.Errorf("ParseLogs 期望 %d 个结果，实际得到 %d 个", expectResult, len(results))
	}
	if err := validate(results); err != nil {
		t.Errorf("结果验证失败: %v", err)
	}
	if len(results) > 0 {
		fmt.Printf("测试 '空主题' 的结果: %v\n", results)
	}
}

// 测试无效数据格式
func TestEventParser_ParseLogs_InvalidDataFormat(t *testing.T) {
	parser, err := NewEventParser("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1")
	if err != nil {
		t.Fatalf("创建 EventParser 失败: %v", err)
	}

	logs := []*types.Log{
		{
			Address: common.HexToAddress("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1"),
			Topics:  []common.Hash{parser.EventHash()},
			Data:    common.Hex2Bytes("invalid"),
		},
	}
	expectResult := 0
	validate := func(results []ConfirmDataStoreData) error {
		if len(results) != expectResult {
			return fmt.Errorf("期望 %d 个结果，实际得到 %d 个", expectResult, len(results))
		}
		return nil
	}

	results, err := parser.ParseLogs(logs)
	if err != nil {
		t.Errorf("ParseLogs 期望无错误，实际错误: %v", err)
	}
	if len(results) != expectResult {
		t.Errorf("ParseLogs 期望 %d 个结果，实际得到 %d 个", expectResult, len(results))
	}
	if err := validate(results); err != nil {
		t.Errorf("结果验证失败: %v", err)
	}
	if len(results) > 0 {
		fmt.Printf("测试 '无效数据格式' 的结果: %v\n", results)
	}
}

// 测试大量日志（压力测试）
func TestEventParser_ParseLogs_LargeNumberOfLogs(t *testing.T) {
	parser, err := NewEventParser("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1")
	if err != nil {
		t.Fatalf("创建 EventParser 失败: %v", err)
	}

	logs := func() []*types.Log {
		var logs []*types.Log
		for i := 0; i < 1000; i++ {
			logs = append(logs, &types.Log{
				Address: common.HexToAddress("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1"),
				Topics:  []common.Hash{parser.EventHash()},
				Data:    common.Hex2Bytes("00000000000000000000000000000000000000000000000000000000000089ba27bc30064cc44c6aef26ca2d7e4ee667592949a50f4f01d8d4632461a12f2243"),
			})
		}
		return logs
	}()
	expectResult := 1000
	validate := func(results []ConfirmDataStoreData) error {
		if len(results) != expectResult {
			return fmt.Errorf("期望 %d 个结果，实际得到 %d 个", expectResult, len(results))
		}
		for i, result := range results {
			if result.DataStoreID != 35258 {
				return fmt.Errorf("第 %d 个 DataStoreID 期望 35258，实际 %d", i, result.DataStoreID)
			}
			if result.HeaderHash != common.HexToHash("0x27bc30064cc44c6aef26ca2d7e4ee667592949a50f4f01d8d4632461a12f2243") {
				return fmt.Errorf("第 %d 个 HeaderHash 不符合预期", i)
			}
		}
		return nil
	}

	results, err := parser.ParseLogs(logs)
	if err != nil {
		t.Errorf("ParseLogs 期望无错误，实际错误: %v", err)
	}
	if len(results) != expectResult {
		t.Errorf("ParseLogs 期望 %d 个结果，实际得到 %d 个", expectResult, len(results))
	}
	if err := validate(results); err != nil {
		t.Errorf("结果验证失败: %v", err)
	}
	if len(results) > 0 {
		fmt.Printf("测试 '大量日志（压力测试）' 的结果: %v\n", results[:5])
	}
}

// 测试无效 dataStoreId 类型
func TestEventParser_ParseLogs_InvalidDataStoreIDType(t *testing.T) {
	parser, err := NewEventParser("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1")
	if err != nil {
		t.Fatalf("创建 EventParser 失败: %v", err)
	}

	// 构造一个 Data 字段，尝试使 dataStoreId 被解析为非 uint32 类型
	logs := []*types.Log{
		{
			Address: common.HexToAddress("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1"),
			Topics:  []common.Hash{parser.EventHash()},
			Data:    common.Hex2Bytes("000000000000000000000000000000000000000000000000000000010000000027bc30064cc44c6aef26ca2d7e4ee667592949a50f4f01d8d4632461a12f2243"), // dataStoreId 为 uint64
		},
	}
	expectResult := 0
	validate := func(results []ConfirmDataStoreData) error {
		if len(results) != expectResult {
			return fmt.Errorf("期望 %d 个结果，实际得到 %d 个", expectResult, len(results))
		}
		return nil
	}

	results, err := parser.ParseLogs(logs)
	if err != nil {
		t.Errorf("ParseLogs 期望无错误，实际错误: %v", err)
	}
	if len(results) != expectResult {
		t.Errorf("ParseLogs 期望 %d 个结果，实际得到 %d 个", expectResult, len(results))
	}
	if err := validate(results); err != nil {
		t.Errorf("结果验证失败: %v", err)
	}
	if len(results) > 0 {
		fmt.Printf("测试 '无效 dataStoreId 类型' 的结果: %v\n", results)
	}
}

// 测试 ParseLogs 的类型断言错误路径
func TestEventParser_ParseLogs_TypeErrors(t *testing.T) {
	parser, err := NewEventParser("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1")
	if err != nil {
		t.Fatalf("Failed to create parser: %v", err)
	}

	// 构造日志数据：uint64 和动态 bytes
	logs := []*types.Log{
		{
			Address: common.HexToAddress("0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1"),
			Topics:  []common.Hash{parser.eventHash},
			Data: common.Hex2Bytes(
				"00000000000000000000000000000000000000000000000000000000000089ba" + // uint64 dataStoreId = 35226
					"0000000000000000000000000000000000000000000000000000000000000020" + // bytes 偏移量
					"0000000000000000000000000000000000000000000000000000000000000020" + // bytes 长度 = 32
					"27bc30064cc44c6aef26ca2d7e4ee667592949a50f4f01d8d4632461a12f2243"), // bytes 数据
		},
	}

	results, err := parser.ParseLogs(logs)
	if err != nil {
		t.Fatalf("Failed to parse logs: %v", err)
	}
	if len(results) == 0 {
		t.Log("No valid logs parsed, ensure test data is correct")
	}
}
