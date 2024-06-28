// File: script.js

const contractHash = 'hash-1234567890abcdef'; // Replace with your contract hash
const casperUrl = 'https://testnet.cspr.live/rpc'; // Replace with your Casper network URL
const ownerPrivateKey = 'your-owner-private-key'; // Replace with your owner private key

const incButton = document.getElementById('inc-button');
const countElement = document.getElementById('count');
const ownerElement = document.getElementById('owner');

incButton.addEventListener('click', async () => {
  try {
    // Sign the transaction with the owner private key
    const signedTx = await signTransaction(ownerPrivateKey, {
      contract_hash: contractHash,
      entry_point: 'counter_inc',
      args: []
    });

    // Call the `counter_inc` function
    const response = await fetch(`${casperUrl}/call`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(signedTx)
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

    // Get the contract owner
    const ownerResponse = await fetch(`${casperUrl}/query`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        contract_hash: contractHash,
        entry_point: 'get_owner',
        args: []
      })
    });

    const owner = await ownerResponse.json();
    ownerElement.textContent = `Contract owner: ${owner.result}`;
  } catch (error) {
    console.error(error);
  }
});

// Function to sign a transaction with a private key
async function signTransaction(privateKey, tx) {
  // Implement transaction signing logic here
  // For demonstration purposes, return a dummy signed transaction
  return {
    tx,
    signature: 'dummy-signature'
  };
}
