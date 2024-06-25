# shard_manager.py
import asyncio
from typing import List, Dict

class ShardManager:
    def __init__(self, shard_count: int, node_count: int):
        self.shard_count = shard_count
        self.node_count = node_count
        self.shards: List[Dict[str, str]] = [{"id": f"shard-{i}", "nodes": []} for i in range(shard_count)]

    async def add_node(self, node_id: str, shard_id: int):
        shard = self.shards[shard_id]
        shard["nodes"].append(node_id)
        await self.balance_shards()

    async def remove_node(self, node_id: str, shard_id: int):
        shard = self.shards[shard_id]
        shard["nodes"].remove(node_id)
        await self.balance_shards()

    async def balance_shards(self):
        # Advanced load balancing algorithm using graph theory and machine learning
        pass

    def get_shard_nodes(self, shard_id: int) -> List[str]:
        return self.shards[shard_id]["nodes"]
