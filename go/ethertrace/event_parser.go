package ethertrace

import (
	"errors"
	"strings"

	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/log"
)

// EventParser 事件解析器
type EventParser struct {
	eventABI     abi.Arguments
	eventHash    common.Hash
	contractAddr common.Address
}

// ConfirmDataStoreData 存储解析后的事件数据
type ConfirmDataStoreData struct {
	DataStoreID uint32
	HeaderHash  common.Hash
}

// NewEventParser 创建 ConfirmDataStore 事件解析器
func NewEventParser(contractAddr string) (*EventParser, error) {
	// 验证合约地址
	if contractAddr == "" {
		return nil, errors.New("合约地址不能为空")
	}
	if !strings.HasPrefix(contractAddr, "0x") || len(contractAddr) != 42 {
		return nil, errors.New("无效的合约地址格式")
	}
	addr := common.HexToAddress(contractAddr)
	if addr.Hex() == "0x0000000000000000000000000000000000000000" {
		return nil, errors.New("合约地址不能为零地址")
	}

	// 定义 ConfirmDataStore 事件的 ABI
	uint32Type, _ := abi.NewType("uint32", "uint32", nil)
	//if err != nil {
	//	return nil, err
	//}
	bytes32Type, _ := abi.NewType("bytes32", "bytes32", nil)
	//if err != nil {
	//	return nil, err
	//}

	eventABI := abi.Arguments{
		{Name: "dataStoreId", Type: uint32Type},
		{Name: "headerHash", Type: bytes32Type},
	}

	// 计算事件签名哈希
	eventHash := crypto.Keccak256Hash([]byte("ConfirmDataStore(uint32,bytes32)"))

	return &EventParser{
		eventABI:     eventABI,
		eventHash:    eventHash,
		contractAddr: addr,
	}, nil
}

// EventHash returns the event signature hash
func (ep *EventParser) EventHash() common.Hash {
	return ep.eventHash
}

// ParseLogs 解析日志并提取 ConfirmDataStore 事件数据
func (ep *EventParser) ParseLogs(logs []*types.Log) ([]ConfirmDataStoreData, error) {
	var results []ConfirmDataStoreData

	for _, l := range logs {
		// 过滤合约地址和事件签名
		if !strings.EqualFold(l.Address.String(), ep.contractAddr.String()) {
			continue
		}
		if len(l.Topics) == 0 || l.Topics[0] != ep.eventHash {
			continue
		}

		// 解码日志数据
		dataMap := make(map[string]interface{})
		if err := ep.eventABI.UnpackIntoMap(dataMap, l.Data); err != nil {
			log.Error("Failed to unpack log data", "error", err)
			continue
		}

		// 提取字段
		dataStoreID, _ := dataMap["dataStoreId"].(uint32)
		//dataStoreID, ok := dataMap["dataStoreId"].(uint32)
		//if !ok {
		//	log.Warn("Invalid dataStoreId type")
		//	continue
		//}
		headerHash, _ := dataMap["headerHash"].([32]byte)
		//headerHash, ok := dataMap["headerHash"].([32]byte)
		//if !ok {
		//	log.Warn("Invalid headerHash type")
		//	continue
		//}

		results = append(results, ConfirmDataStoreData{
			DataStoreID: dataStoreID,
			HeaderHash:  common.Hash(headerHash),
		})
	}

	return results, nil
}
