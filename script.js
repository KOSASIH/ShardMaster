// File: script.js

const contractHash = 'hash-1234567890abcdef'; // Replace with your contract hash
const casperUrl = 'https://testnet.cspr.live/rpc'; // Replace with your Casper network URL
const ownerPrivateKey = 'your-owner-private-key'; // Replace with your owner private key

const incButton = document.getElementById('inc-button');
const countElement = document.getElementById('count');
const ownerElement = document.getElementById('owner');
const resetButton = document.getElementById('reset-button');
const transferForm = document.getElementById('transfer-form');

incButton.addEventListener('click', async () => {
  //...
});

resetButton.addEventListener('click', async () => {
  try {
    // Sign the transaction with the owner private key
    const signedTx = await signTransaction(ownerPrivateKey, {
      contract_hash: contractHash,
      entry_point: 'eset',
      args: []
    });

    // Call the `reset` function
    const response = await fetch(`${casperUrl}/call`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(signedTx)
    });

    // Update the count element
    countElement.textContent = 'Current count: 0';
  } catch (error) {
    console.error(error);
  }
});

transferForm.addEventListener('submit', async (event) => {
  //...
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
