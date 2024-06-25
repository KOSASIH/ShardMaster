# cluster_manager.py
import etcd3

class ClusterManager:
    def __init__(self, cluster_config):
        self.cluster_config = cluster_config
        self.etcd_client = etcd3.client()

    def register_node(self, node_id, node_host, node_port):
        self.etcd_client.put(f'/nodes/{node_id}', value=f'{node_host}:{node_port}')

    def get_nodes(self):
        nodes = self.etcd_client.get_prefix('/nodes/')
        return [node.value.decode('utf-8') for node in nodes]

    def deregister_node(self, node_id):
        self.etcd_client.delete(f'/nodes/{node_id}')
