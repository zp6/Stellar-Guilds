/**
 * Token Balance Display Component. Fixes GalactiGuild/Stellar-Guilds#336.
 * Aesthetic wallet balance card for XLM and USDC with localization.
 */
import React from 'react';

interface Balances {
  XLM: number;
  USDC: number;
}

interface TokenBalanceCardProps {
  balances: Balances;
  loading?: boolean;
}

function formatBalance(value: number, currency: string): string {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: currency === 'XLM' ? 'USD' : 'USD',
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }).format(value).replace('$', currency === 'XLM' ? 'XLM ' : 'USDC ');
}

export function TokenBalanceCard({ balances, loading = false }: TokenBalanceCardProps) {
  if (loading) {
    return (
      <div className="token-balance-card loading" data-testid="balance-card-loading">
        <div className="skeleton skeleton-title" />
        <div className="skeleton skeleton-balance" />
        <div className="skeleton skeleton-balance" />
      </div>
    );
  }

  return (
    <div className="token-balance-card" data-testid="balance-card">
      <h3 className="card-title">Wallet Balance</h3>
      <div className="balance-row">
        <svg className="token-icon xlm-icon" viewBox="0 0 32 32" aria-label="Stellar XLM">
          <circle cx="16" cy="16" r="14" fill="#1d1d1d" />
          <path d="M16 4 L20 12 L28 12 L22 18 L24 28 L16 22 L8 28 L10 18 L4 12 L12 12 Z" fill="#6b6b6b" />
        </svg>
        <div className="balance-info">
          <span className="token-name">Stellar (XLM)</span>
          <span className="balance-amount">{formatBalance(balances.XLM, 'XLM')}</span>
        </div>
      </div>
      <div className="balance-row">
        <svg className="token-icon usdc-icon" viewBox="0 0 32 32" aria-label="USD Coin">
          <circle cx="16" cy="16" r="14" fill="#2775CA" />
          <text x="16" y="20" textAnchor="middle" fill="white" fontSize="14" fontWeight="bold">$</text>
        </svg>
        <div className="balance-info">
          <span className="token-name">USD Coin (USDC)</span>
          <span className="balance-amount">{formatBalance(balances.USDC, 'USDC')}</span>
        </div>
      </div>
    </div>
  );
}

export default TokenBalanceCard;

// Tests
import assert from 'assert';

function testFormatBalance() {
  const result = formatBalance(1500.50, 'XLM');
  assert(result.includes('1,500.50'));
}

function testFormatBalanceZero() {
  const result = formatBalance(0, 'USDC');
  assert(result.includes('0.00'));
}

testFormatBalance();
testFormatBalanceZero();
console.log('TokenBalanceCard: tests passed');
