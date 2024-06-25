// storage/edge_storage.go
package main

import (
	"context"
	"fmt"
	"log"

	edge "github.com/edgelesssys/edge"
)

type EdgeStorage struct {
	edgeClient *edge.Client
}

func NewEdgeStorage() *EdgeStorage {
	edgeClient, err := edge.NewClient(context.Background())
	if err!= nil {
		log.Fatal(err)
	}
	return &EdgeStorage{edgeClient: edgeClient}
}

func (es *EdgeStorage) StoreData(data []byte) error {
	// Store data on edge devices using the Edgeless library
	return es.edgeClient.Put(context.Background(), "data", data)
}

func (es *EdgeStorage) RetrieveData() ([]byte, error) {
	// Retrieve data from edgedevices using the Edgeless library
	return es.edgeClient.Get(context.Background(), "data")
}
