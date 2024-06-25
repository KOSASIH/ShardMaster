// shard_master/transaction/raft_manager.go
package transaction

import (
	"fmt"
	"log"
	"net"
	"sync"

	"github.com/hashicorp/raft"
)

type RaftManager struct {
	raftNode *raft.Raft
}

func NewRaftManager() *RaftManager {
	rm := &RaftManager{}
	rm.raftNode = raft.NewRaftNode("shard_master", 8080)
	return rm
}

func (rm *RaftManager) Start() {
	rm.raftNode.Start()
}

func (rm *RaftManager) ProposeTransaction(tx []byte) error {
	return rm.raftNode.Propose(tx)
}

func (rm *RaftManager) CommitTransaction(tx []byte) error {
	return rm.raftNode.Commit(tx)
}
