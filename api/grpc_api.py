# grpc_api.py
import grpc
from shard_manager import ShardManager
from data_distribution import DataDistribution

class ShardMasterServicer(shard_master_pb2.ShardMasterServicer):
    def __init__(self, shard_manager, data_distribution):
        self.shard_manager = shard_manager
        self.data_distribution = data_distribution

    def WriteData(self, request, context):
        shard_id, node_id = self.data_distribution.distribute_data(request.data)
        return shard_master_pb2.WriteResponse(shard_id=shard_id, node_id=node_id)

def serve():
    server= grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    shard_master_pb2.add_ShardMasterServicer_to_server(ShardMasterServicer(shard_manager, data_distribution), server)
    server.add_insecure_port('[::]:50051')
    server.start()
    server.wait_for_termination()

if __name__ == '__main__':
    serve()
