package config

import (
	"encoding/json"
	"io/ioutil"
	"log"
)

type Config struct {
	Node NodeConfig `json:"node"`
	Sharding ShardingConfig `json:"sharding"`
	CrossShardComm CrossShardCommConfig `json:"cross_shard_comm"`
	SmartContractIntegration SmartContractIntegrationConfig `json:"smart_contract_integration"`
}

type NodeConfig struct {
	ListenAddr string `json:"listen_addr"`
	DataDir string `json:"data_dir"`
}

type ShardingConfig struct {
	Algorithm string `json:"algorithm"`
	NumShards int `json:"num_shards"`
}

type CrossShardCommConfig struct {
	Protocol string `json:"protocol"`
	BufferSize int `json:"buffer_size"`
}

type SmartContractIntegrationConfig struct {
	PiNetworkAPIURL string `json:"pi_network_api_url"`
}

func LoadConfig(file string) (*Config, error) {
	var config Config
	data, err := ioutil.ReadFile(file)
	if err != nil {
		return nil, err
	}
	err = json.Unmarshal(data, &config)
	if err != nil {
		return nil, err
	}
	return &config, nil
}
