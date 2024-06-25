package cross_shard_comm

import (
	"fmt"
	"net"

	"github.com/KOSASIH/ShardMaster/config"
	"github.com/KOSASIH/ShardMaster/utils"
)

type CrossShardComm struct {
	config config.CrossShardCommConfig
	conn net.Conn
}

func NewCrossShardComm(config config.CrossShardCommConfig) (*CrossShardComm, error) {
	conn, err := net.Dial("tcp", config.Protocol+"://"+config.BufferSize)
	if err != nil {
		return nil, err
	}
	return &CrossShardComm{config: config, conn: conn}, nil
}

func (c *CrossShardComm) Send(tx []byte) error {
	_, err :=c.conn.Write(tx)
	return err
}

func (c *CrossShardComm) Receive() ([]byte, error) {
	buf := make([]byte, c.config.BufferSize)
	_, err := c.conn.Read(buf)
	return buf, err
}
