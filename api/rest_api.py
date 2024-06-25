# rest_api.py
from flask import Flask, request, jsonify
from shard_manager import ShardManager
from data_distribution import DataDistribution

app = Flask(__name__)

shard_manager = ShardManager(shard_config)
data_distribution = DataDistribution(shard_manager, cluster_manager)

@app.route('/write', methods=['POST'])
def write_data():
    data = request.get_json()
    shard_id, node_id = data_distribution.distribute_data(data)
    return jsonify({'shard_id': shard_id, 'node_id': node_id})

if __name__ == '__main__':
    app.run(debug=True)
