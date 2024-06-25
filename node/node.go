package node

import (
	"fmt"
	"net"

	"github.com/KOSASIH/ShardMaster/config"
	"github.com/KOSASIH/ShardMaster/utils"
)

type Node struct {
	config config.NodeConfig
	listener net.Listener
}

func NewNode(config config.NodeConfig) (*Node, error) {
	listener, err := net.Listen("tcp", config.ListenAddr)
	if err != nil {
		return nil, err
	}
	return &Node{config: config, listener: listener}, nil
}

func (n *Node) Start() error {
	utils.Logger.Infof("Node started on %s", n.config.ListenAddr)
	return nil
}

func (n *Node) Stop() error {
	utils.Logger.Infof("Node stopped")
	return nil
}
