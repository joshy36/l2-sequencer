'use client';

import React, { useState, useEffect } from 'react';

export default function SendTransactionButton() {
  const [status, setStatus] = useState('');
  const [error, setError] = useState('');
  const [transactions, setTransactions] = useState<any[]>([]);

  useEffect(() => {
    const ws = new WebSocket('ws://0.0.0.0:3001/transaction_feed');

    ws.onopen = () => {
      console.log('Connected to transaction feed');
    };

    ws.onmessage = (event) => {
      try {
        const transaction = JSON.parse(event.data);
        setTransactions((prev) => [...prev, transaction].slice(-10));
      } catch (err) {
        console.error('Failed to parse WebSocket message:', err);
      }
    };

    ws.onerror = (error) => {
      console.error('WebSocket error:', error);
    };

    ws.onclose = () => {
      console.log('Disconnected from transaction feed');
    };

    return () => {
      ws.close();
    };
  }, []);

  const sendTransaction = async () => {
    const payload = {
      raw_tx: {
        nonce: '0x0',
        from: '0x1111111111111111111111111111111111111111',
        to: '0x2222222222222222222222222222222222222222',
        value: '0xde0b6b3a7640000',
        data: '0x68656c6c6f',
        gas_limit: '0x5208',
        gas_price: '0x6fc23ac00',
        chain_id: 42161,
        l1_block_number: 0,
        submission_fee: '0xf4240',
      },
    };

    try {
      setStatus('');
      setError('');

      const response = await fetch('http://0.0.0.0:3001/send_transaction', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `Bearer ${process.env.NEXT_PUBLIC_BEARER_TOKEN}`,
        },
        body: JSON.stringify(payload),
      });

      const responseText = await response.text();

      try {
        const data = JSON.parse(responseText);
        if (!response.ok) {
          throw new Error(data.error || 'Transaction failed');
        }
        setStatus(data.status);
      } catch (jsonError) {
        setError(`Unexpected response: ${responseText}`);
      }
    } catch (err: unknown) {
      setError(err instanceof Error ? err.message : String(err));
    }
  };

  return (
    <div className="flex flex-col items-center gap-4 p-4 bg-black text-green-400 min-h-screen font-mono">
      <button
        onClick={sendTransaction}
        className="relative px-5 py-3 text-base uppercase tracking-wider text-green-400 border-2 border-green-400 bg-black hover:bg-green-400 hover:text-black transition-all duration-300 ease-in-out shadow-[0_0_10px_#00ff00] hover:shadow-[0_0_20px_#00ff00]"
      >
        <span className="relative z-10">SEND_TRANSACTION</span>
        <div className="absolute inset-0 bg-green-400 opacity-0 hover:opacity-10 transition-opacity duration-300"></div>
      </button>

      <div className="h-6">
        {status && <p className="text-green-400 text-sm">[STATUS]::{status}</p>}
        {error && <p className="text-red-500 text-sm">[ERROR]::{error}</p>}
      </div>

      {transactions.length > 0 && (
        <div className="mt-6 w-full max-w-lg border border-green-400 p-4 shadow-[0_0_15px_#00ff00]">
          <h3 className="text-lg uppercase text-green-400 mb-2">
            TRANSACTION_FEED
          </h3>
          <ul className="space-y-2 text-sm text-green-400">
            {transactions.map((tx, index) => (
              <li
                key={index}
                className="break-all hover:text-green-500 transition-colors"
              >
                <span className="text-purple-500">[TX#{index}]</span> FROM:
                {tx.from} | TO:{tx.to} | VAL:{tx.value}
              </li>
            ))}
          </ul>
        </div>
      )}
    </div>
  );
}
