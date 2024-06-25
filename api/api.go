// api.go
package main

import (
	"encoding/json"
	"fmt"
	"net/http"

	"github.com/gorilla/mux"
)

type ShardMasterAPI struct {
	shardMaster *ShardMaster
}

func (api *ShardMasterAPI) Start() {
	router := mux.NewRouter()
	router.HandleFunc("/shards", api.getShards).Methods("GET")
	router.HandleFunc("/shards/{id}", api.getShard).Methods("GET")
	//...
	http.ListenAndServe(":8080", router)
}

func (api *ShardMasterAPI) getShards(w http.ResponseWriter, r *http.Request) {
	shards := api.shardMaster.getShards()
	json.NewEncoder(w).Encode(shards)
}

func (api *ShardMasterAPI) getShard(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	id, err := strconv.Atoi(vars["id"])
	if err!= nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	shard := api.shardMaster.getShard(id)
	json.NewEncoder(w).Encode(shard)
}

func main() {
	shardMaster := ShardMaster{}
	api := ShardMasterAPI{&shardMaster}
	api.Start()
}
