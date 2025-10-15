import React, { useState } from 'react';
import axios from 'axios';

function App() {
  const [wallet, setWallet] = useState(null);
  const [loading, setLoading] = useState(false);

  const createWallet = async () => {
    setLoading(true);
    try {
      const response = await axios.post('http://localhost:3030/create_wallet', {
        user_id: 'user123'
      });
      setWallet(response.data);
    } catch (error) {
      console.error("Error creating wallet:", error);
    }
    setLoading(false);
  };

  return (
    <div className="container">
      <h1>Quantum Crypto Wallet Demo</h1>
      <button onClick={createWallet} disabled={loading}>
        {loading ? "Creating Wallet..." : "Create Wallet"}
      </button>

      {wallet && (
        <div className="wallet-info">
          <p><strong>Public Key:</strong> {wallet.public_key}</p>
          <p><strong>Encrypted Private Key:</strong> {wallet.encrypted_private_key}</p>
        </div>
      )}
    </div>
  );
}

export default App;


