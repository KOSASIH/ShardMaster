// ShardMasterAIWeb.tsx
import React, { useState, useEffect } from 'react';
import axios from 'axios';

interface Prediction {
  prediction: number[];
}

const ShardMasterAIWeb: React.FC = () => {
  const [inputData, setInputData] = useState<number[]>([]);
  const [prediction, setPrediction] = useState<Prediction | null>(null);

  useEffect(() => {
    axios.post('/predict', inputData)
      .then(response => {
        setPrediction(response.data);
      })
      .catch(error => {
        console.error(error);
      });
  }, [inputData]);

  const handleInputChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const value = event.target.value;
    const inputArray = value.split(',').map(Number);
    setInputData(inputArray);
  };

  return (
    <div>
      <h1>ShardMaster AI Web Interface</h1>
      <input type="text" value={inputData.join(',')} onChange={handleInputChange} />
      <button onClick={() => axios.post('/predict', inputData)}>Make Prediction</button>
      {prediction && (
        <div>
          <h2>Prediction: {prediction.prediction.join(',')}</h2>
        </div>
      )}
    </div>
  );
};

export default ShardMasterAIWeb;
