// File: script.js

const contractHash = 'hash-1234567890abcdef'; // Replace with your contract hash
const casperUrl = 'https://testnet.cspr.live/rpc'; // Replace with your Casper network URL

const incButton = document.getElementById('inc-button');
const countElement = document.getElementById('count');

incButton.addEventListener('click', async () => {
  try {
    // Call the `counter_inc` function
    const response = await fetch(`${casperUrl}/call`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        contract_hash: contractHash,
        entry_point: 'counter_inc',
        args: []
      })
    });

    // Get the current count
    const countResponse = await fetch(`${casperUrl}/query`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        contract_hash: contractHash,
        entry_point: 'counter_get',
        args: []
      })
    });

    const count = await countResponse.json();
    countElement.textContent = `Current count: ${count.result}`;
  } catch (error) {
    console.error(error);
  }
});
