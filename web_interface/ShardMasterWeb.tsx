// ShardMasterWeb.tsx
import React, { useState, useEffect } from 'eact';
import axios from 'axios';

interface Shard {
  id: number;
  name: string;
  //...
}

const ShardMasterWeb: React.FC = () => {
  const [shards, setShards] = useState<Shard[]>([]);
  const [selectedShard, setSelectedShard] = useState<Shard | null>(null);

  useEffect(() => {
    axios.get('/shards')
     .then(response => {
        setShards(response.data);
      })
     .catch(error => {
        console.error(error);
      });
  }, []);

  const handleShardSelect = (shard: Shard) => {
    setSelectedShard(shard);
  };

  return (
    <div>
      <h1>ShardMaster Web Interface</h1>
      <ul>
        {shards.map(shard => (
          <li key={shard.id}>
            <a href="#" onClick={() => handleShardSelect(shard)}>
              {shard.name}
            </a>
          </li>
        ))}
      </ul>
      {selectedShard && (
        <div>
          <h2>Selected Shard: {selectedShard.name}</h2>
          {/* Display shard-specific information and controls */}
        </div>
      )}
    </div>
  );
};

export default ShardMasterWeb;
