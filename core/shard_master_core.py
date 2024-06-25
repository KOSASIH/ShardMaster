# shard_master_core.py
import asyncio
import json
import os
import sys
from typing import Dict, List

class ShardMaster:
    def __init__(self, config: Dict[str, str]):
        self.config = config
        self.shards: List[Shard] = []
        self.cluster_manager = ClusterManager(self.config)

    async def start(self):
        await self.cluster_manager.start()
        for shard in self.shards:
            await shard.start()

    async def stop(self):
        for shard in self.shards:
            await shard.stop()
        await self.cluster_manager.stop()

    def add_shard(self, shard: Shard):
        self.shards.append(shard)

class Shard:
    def __init__(self, id: int, config: Dict[str, str]):
        self.id = id
        self.config = config
        self.server = Server(self.config)

    async def start(self):
        await self.server.start()

    async def stop(self):
        await self.server.stop()

class Server:
    def __init__(self, config: Dict[str, str]):
        self.config = config
        self.http_server = HTTPServer(self.config)

    async def start(self):
        await self.http_server.start()

    async def stop(self):
        await self.http_server.stop()

class HTTPServer:
    def __init__(self, config: Dict[str, str]):
        self.config = config
        self.app = web.Application()
        self.app.router.add_get('/', self.index)
        self.app.router.add_post('/shard', self.create_shard)

    async def start(self):
        runner = web.AppRunner(self.app)
        await runner.setup()
        site = web.TCPSite(runner, '0.0.0.0', 8080)
        await site.start()

    async def stop(self):
        await self.app.shutdown()

    async def index(self, request: web.Request):
        return web.Response(text='ShardMaster is running!')

    async def create_shard(self, request: web.Request):
        data = await request.json()
        shard_id = data['shard_id']
        shard_config = data['shard_config']
        shard = Shard(shard_id, shard_config)
        shard_master.add_shard(shard)
        return web.Response(text=f'Shard {shard_id} created!')

class ClusterManager:
    def __init__(self, config: Dict[str, str]):
        self.config = config
        self.cluster = []

    async def start(self):
        for node in self.config['cluster_nodes']:
            self.cluster.append(Node(node))

    async def stop(self):
        for node in self.cluster:
            await node.stop()

class Node:
    def __init__(self, node_config: Dict[str, str]):
        self.config = node_config
        self.client = NodeClient(self.config)

    async def stop(self):
        await self.client.stop()

class NodeClient:
    def __init__(self, config: Dict[str, str]):
        self.config = config
        self.socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

    async def stop(self):
        self.socket.close()

if __name__ == '__main__':
    config = json.load(open('config.json'))
    shard_master = ShardMaster(config)
    asyncio.run(shard_master.start())
