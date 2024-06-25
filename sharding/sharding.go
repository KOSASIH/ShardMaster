package sharding

import (
	"fmt"
	"math/rand"

	"github.com/KOSASIH/ShardMaster/config"
	"github.com/KOSASIH/ShardMaster/utils"
)

type Sharding struct {
	config config.ShardingConfig
	shards []Shard
}

func NewSharding(config config.ShardingConfig) (*Sharding, error) {
	shards := make([]Shard, config.NumShards)
	for i := range shards {
		shards[i] = NewShard(i, config.Algorithm)
	}
	return &Sharding{config: config, shards: shards}, nil
}

func (s *Sharding) GetShard(id uint64) (*Shard, error) {
	shardId := id % uint64(s.config.NumShards)
	return &s.shards[shardId], nil
}

func (s *Sharding) ProcessTransaction(tx []byte) error {
	shard, err := s.GetShard(rand.Uint64())
	if err != nil {
		return err
	}
	return shard.ProcessTransaction(tx)
}
