package smart_contract_integration

import (
	"fmt"
	"net/http"

	"github.com/KOSASIH/ShardMaster/config"
	"github.com/KOSASIH/ShardMaster/utils"
)

type SmartContractIntegration struct {
	config config.SmartContractIntegrationConfig
	client *http.Client
}

func NewSmartContractIntegration(config config.SmartContractIntegrationConfig) (*SmartContractIntegration, error) {
	client := &http.Client{}
	return &SmartContractIntegration{config: config, client: client}, nil
}

func (s *SmartContractIntegration) CallSmartContract(tx []byte) ([]byte, error) {
	req, err := http.NewRequest("POST", s.config.PiNetworkAPIURL, bytes.NewBuffer(tx))
	if err != nil {
		return nil, err
	}
	resp, err := s.client.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()
	return ioutil.ReadAll(resp.Body)
}
