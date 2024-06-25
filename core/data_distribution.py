# data_distribution.py
import random

class DataDistribution:
    def __init__(self, shard_manager, cluster_manager):
        self.shard_manager = shard_manager
        self.cluster_manager = cluster_manager

    def distribute_data(self, data):
        shard_id = self.shard_manager.get_shard(data['key'])
        node_id = random.choice(self.cluster_manager.get_nodes())
        # Send data to the selected node
        return shard_id, node_id
