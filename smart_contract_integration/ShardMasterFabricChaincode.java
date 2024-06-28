package org.shardmaster.fabric.chaincode;

import org.hyperledger.fabric.chaincode.ChaincodeBase;
import org.hyperledger.fabric.chaincode.ChaincodeStub;
import org.hyperledger.fabric.chaincode.annotation.Chaincode;
import org.hyperledger.fabric.chaincode.annotation.Init;
import org.hyperledger.fabric.chaincode.annotation.Transaction;

@Chaincode(name = "ShardMasterFabricChaincode")
public class ShardMasterFabricChaincode extends ChaincodeBase {

    @Init
    public void init(ChaincodeStub stub) {
        // Initialize the shard master
        ShardMaster shardMaster = new ShardMaster();
        shardMaster.init(stub);
    }

    @Transaction
    public void createShard(ChaincodeStub stub, String shardId) {
        // Create a new shard
        Shard shard = new Shard(shardId);
        shardMaster.createShard(stub, shard);
    }

    @Transaction
    public void assignUserToShard(ChaincodeStub stub, String userId, String shardId) {
        // Assign a user to a shard
        shardMaster.assignUserToShard(stub, userId, shardId);
    }

    @Transaction
    public String getShardForUser(ChaincodeStub stub, String userId) {
        // Get the shard for a user
        return shardMaster.getShardForUser(stub, userId);
    }
}
