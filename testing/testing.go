package testing

import (
	"testing"

	"github.com/KOSASIH/ShardMaster/config"
	"github.com/KOSASIH/ShardMaster/node"
	"github.com/KOSASIH/ShardMaster/sharding"
	"github.com/KOSASIH/ShardMaster/cross_shard_comm"
	"github.com/KOSASIH/ShardMaster/smart_contract_integration"
)

func TestNode(t *testing.T) {
	config := config.Config{}
	node, err := node.NewNode(config.Node)
	if err != nil {
		t.Fatal(err)
	}
	defer node.Stop()
}

func TestSharding(t *testing.T) {
	config := config.Config{}
	sharding, err := sharding.NewSharding(config.Sharding)
	if err != nil {
		t.Fatal(err)
	}
	tx := []byte("test transaction")
	err = sharding.ProcessTransaction(tx)
	if err != nil {
		t.Fatal(err)
	}
}

func TestCrossShardComm(t *testing.T) {
	config := config.Config{}
	crossShardComm, err := cross_shard_comm.NewCrossShardComm(config.CrossShardComm)
	if err != nil {
		t.Fatal(err)
	}
	tx := []byte("test transaction")
	err = crossShardComm.Send(tx)
	if err != nil {
		t.Fatal(err)
	}
}

func TestSmartContractIntegration(t *testing.T) {
	config := config.Config{}
	smartContractIntegration, err := smart_contract_integration.NewSmartContractIntegration(config.SmartContractIntegration)
	if err != nil {
		t.Fatal(err)
	}
	tx := []byte("test transaction")
	resp, err := smartContractIntegration.CallSmartContract(tx)
	if err != nil {
		t.Fatal(err)
	}
	fmt.Println(resp)
}
