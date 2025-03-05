'use client';

import React, { useState, useEffect } from 'react';
import crypto from 'crypto';

interface RawTransaction {
  nonce: string;
  from: string;
  to: string;
  value: string;
  data: string;
  gas_limit: string;
  gas_price: string;
  chain_id: number;
  l1_block_number: number;
  submission_fee: string;
}

export default function SendTransactionButton() {
  const [status, setStatus] = useState('');
  const [error, setError] = useState('');
  const [transactions, setTransactions] = useState<any[]>([]);
  const [nonce, setNonce] = useState(0);
  const [transactionCount, setTransactionCount] = useState(0);
  const [transactionPreview, setTransactionPreview] =
    useState<RawTransaction | null>(null);

  const generateRandomAddress = () => {
    const bytes = crypto.randomBytes(20);
    return `0x${Array.from(bytes)
      .map((byte) => byte.toString(16).padStart(2, '0'))
      .join('')}`;
  };

  const generateRandomValue = () =>
    `0x${Math.floor(Math.random() * 1000000).toString(16)}`;
  const generateRandomData = () =>
    `0x${Array.from(crypto.randomBytes(5))
      .map((byte) => byte.toString(16).padStart(2, '0'))
      .join('')}`;
  const generateRandomGasLimit = () =>
    `0x${(Math.floor(Math.random() * (100000 - 21000 + 1)) + 21000).toString(
      16
    )}`;
  const generateRandomGasPrice = () =>
    `0x${Math.floor(Math.random() * 1000000000).toString(16)}`;
  const generateRandomChainId = () => Math.floor(Math.random() * 100) + 1;
  const generateRandomL1BlockNumber = () =>
    Math.floor(Math.random() * 10000000);
  const generateRandomSubmissionFee = () =>
    `0x${Math.floor(Math.random() * 1000000).toString(16)}`;

  function generatePreview(): RawTransaction {
    return {
      nonce: `0x${nonce.toString(16)}`,
      from: '0x1111111111111111111111111111111111111111',
      to: generateRandomAddress(),
      value: generateRandomValue(),
      data: generateRandomData(),
      gas_limit: generateRandomGasLimit(),
      gas_price: generateRandomGasPrice(),
      chain_id: generateRandomChainId(),
      l1_block_number: generateRandomL1BlockNumber(),
      submission_fee: generateRandomSubmissionFee(),
    };
  }

  useEffect(() => {
    // Set initial preview on client-side mount
    setTransactionPreview(generatePreview());

    const ws = new WebSocket('ws://0.0.0.0:3001/transaction_feed');

    ws.onopen = () => setTransactionCount(0);
    ws.onmessage = (event) => {
      try {
        const transaction = JSON.parse(event.data);
        setTransactions((prev) => [transaction, ...prev].slice(0, 10));
        setTransactionCount((prevCount) => prevCount + 1);
      } catch (err) {
        console.error('WebSocket error:', err);
      }
    };

    return () => ws.close();
  }, []);

  const sendTransaction = async () => {
    if (!transactionPreview) return;

    const payload = {
      raw_tx: transactionPreview,
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
      const data = JSON.parse(responseText);
      if (!response.ok) throw new Error(data.error || 'Transaction failed');

      setStatus(data.status);
      setNonce(nonce + 1);
      setTransactionPreview(generatePreview());
    } catch (err: unknown) {
      setError(err instanceof Error ? err.message : String(err));
    }
  };

  return (
    <div className="flex flex-row gap-6 justify-center p-6 bg-black text-green-400 min-h-screen font-mono space-y-6">
      <div className="flex flex-col items-center justify-center gap-6">
        <button
          onClick={sendTransaction}
          className="px-6 py-3 text-lg font-bold uppercase border-2 border-green-400 bg-black hover:bg-green-400 hover:text-black transition shadow-[0_0_10px_#00ff00] hover:shadow-[0_0_20px_#00ff00]"
        >
          SEND TRANSACTION
        </button>
        <div className="h-6 text-sm">
          {status && <p className="text-green-400">[STATUS]::{status}</p>}
          {error && <p className="text-red-500">[ERROR]::{error}</p>}
        </div>
        <div className="w-full max-w-lg p-4 border border-green-400 rounded-lg bg-black shadow-[0_0_15px_#00ff00]">
          <h4 className="text-md uppercase mb-2">Transaction Preview</h4>
          <pre className="whitespace-pre-wrap text-sm">
            {transactionPreview
              ? JSON.stringify(transactionPreview, null, 2)
              : 'Loading preview...'}
          </pre>
        </div>
      </div>

      <div className="w-full max-w-lg p-4 border border-green-400 rounded-lg shadow-[0_0_15px_#00ff00]">
        <h3 className="text-lg uppercase text-green-400 mb-2">
          Transaction Feed
        </h3>
        <ul className="space-y-2 text-sm">
          {transactions.map((tx, index) => (
            <li
              key={index}
              className="break-all hover:text-green-500 transition-colors"
            >
              <span className="text-purple-500">
                [TX#{transactionCount - index}]
              </span>
              <div className="flex justify-between">
                <span className="font-bold">FROM:</span> <span>{tx.from}</span>
              </div>
              <div className="flex justify-between">
                <span className="font-bold">TO:</span> <span>{tx.to}</span>
              </div>
              <div className="flex justify-between">
                <span className="font-bold">VAL:</span> <span>{tx.value}</span>
              </div>
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
}
