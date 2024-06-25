-- ShardMaster.hs
{-# LANGUAGE OverloadedStrings #-}

import Control.Concurrent.STM
import Control.Monad.IO.Class
import Data.Aeson
import Data.Text
import Network.HTTP.Client
import Network.HTTP.Client.TLS

data Shard = Shard
  { shardId :: Int
 , shardName :: Text
 , shardConfig :: Config
  } deriving (Eq, Show)

data Config = Config
  { configDatabase :: Text
 , configCache :: Text
  } deriving (Eq, Show)

newtype ShardMaster = ShardMaster (TVar [Shard])

instance ToJSON Shard where
  toJSON (Shard id name config) =
    object
      [ "id".= id
     , "name".= name
     , "config".= config
      ]

instance FromJSON Shard where
  parseJSON = withObject "Shard" $ \o ->
    Shard <$> o.: "id"
          <*> o.: "name"
          <*> o.: "config"

startShardMaster :: IO ShardMaster
startShardMaster = do
  shards <- atomically $ newTVar []
  return $ ShardMaster shards

addShard :: ShardMaster -> Shard -> IO ()
addShard (ShardMaster shards) shard =
  atomically $ modifyTVar' shards (shard:)

getShards :: ShardMaster -> IO [Shard]
getShards (ShardMaster shards) =
  atomically $ readTVar shards

main :: IO ()
main = do
  shardMaster <- startShardMaster
  addShard shardMaster (Shard 1 "Shard 1" (Config "db1" "cache1"))
  addShard shardMaster (Shard 2 "Shard 2" (Config "db2" "cache2"))
  shards <- getShards shardMaster
  print shards
