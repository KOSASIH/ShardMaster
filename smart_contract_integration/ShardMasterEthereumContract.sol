pragma solidity ^0.8.0;

import "https://github.com/OpenZeppelin/openzeppelin-solidity/contracts/access/Roles.sol";
import "https://github.com/OpenZeppelin/openzeppelin-solidity/contracts/math/SafeMath.sol";

contract ShardMasterEthereumContract {
    using Roles for address;
    using SafeMath for uint256;

    // Mapping of shard IDs to shard contracts
    mapping (uint256 => address) public shardContracts;

    // Mapping of user IDs to shard IDs
    mapping (address => uint256) public userShardMappings;

    // Event emitted when a new shard is created
    event NewShardCreated(uint256 shardId, address shardContract);

    // Event emitted when a user is assigned to a shard
    event UserAssignedToShard(address userId, uint256 shardId);

    // Create a new shard and assign it to a user
    function createShardAndAssignUser(address userId) public {
        // Create a new shard contract
        address newShardContract = address(new ShardContract());

        // Assign the shard contract to the user
        userShardMappings[userId] = shardContracts.push(newShardContract) - 1;

        // Emit events
        emit NewShardCreated(shardContracts.length - 1, newShardContract);
        emit UserAssignedToShard(userId, shardContracts.length - 1);
    }

    // Get the shard contract for a given user
    function getShardContractForUser(address userId) public view returns (address) {
        return shardContracts[userShardMappings[userId]];
    }
}
