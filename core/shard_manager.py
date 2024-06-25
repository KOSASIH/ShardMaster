# shard_manager.py
import hashlib
from utils.hashing import consistent_hash

class ShardManager:
    def __init__(self, shard_config):
        self.shard_config = shard_config
        self.shards = {}

    def add_shard(self, shard_id, shard_host, shard_port):
        self.shards[shard_id] = (shard_host, shard_port)

    def get_shard(self, key):
        hash_value = consistent_hash(key)
        shard_id = self.shard_config.get_shard_id(hash_value)
        return self.shards[shard_id]

    def remove_shard(self, shard_id):
        del self.shards[shard_id]
