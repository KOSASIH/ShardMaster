// shard_master_ai_cli.go
package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"
	"os"
)

type Prediction struct {
	Prediction []int `json:"prediction"`
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: shard_master_ai_cli <input_data>")
		return
	}
	inputData := os.Args[1]
	url := "http://localhost:5000/predict"
	req, err := http.NewRequest("POST", url, strings.NewReader(inputData))
	if err!= nil {
		fmt.Println(err)
		return
	}
	req.Header.Set("Content-Type", "application/json")
	client := &http.Client{}
	resp, err := client.Do(req)
	if err!= nil {
		fmt.Println(err)
		return
	}
	defer resp.Body.Close()
	body, err := ioutil.ReadAll(resp.Body)
	if err!= nil {
		fmt.Println(err)
		return
	}
	var prediction Prediction
	err = json.Unmarshal(body, &prediction)
	if err!= nil {
		fmt.Println(err)
		return
	}
	fmt.Println(prediction.Prediction)
}
