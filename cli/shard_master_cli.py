# shard_master_cli.py
import argparse
import requests

def get_shards():
    response = requests.get('http://localhost:8080/shards')
    return response.json()

def get_shard(shard_id):
    response = requests.get(f'http://localhost:8080/shards/{shard_id}')
    return response.json()

def main():
    parser = argparse.ArgumentParser(description='ShardMaster CLI')
    parser.add_argument('command', choices=['get-shards', 'get-shard'], help='Command to execute')
    parser.add_argument('--id', type=int, help='Shard ID')
    args = parser.parse_args()

    if args.command == 'get-shards':
        shards = get_shards()
        for shard in shards:
            print(shard)
    elif args.command == 'get-shard':
        shard = get_shard(args.id)
        print(shard)

if __name__ == '__main__':
    main()
