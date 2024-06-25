// shard_monitor.js
import { GraphQLSchema } from 'graphql';
import { ShardManager } from './shard_manager';

const schema = new GraphQLSchema({
  typeDefs: `
    type Shard {
      id: ID!
      nodes: [Node!]!
    }

    type Node {
      id: ID!
      performanceScore: Float!
    }

    type Query {
      shards: [Shard!]!
    }
  `,
  resolvers: {
    Query: {
      shards: () => {
        // Real-time shard monitoring using GraphQL
        return ShardManager.getShards();
      },
    },
  },
});

export default schema;
