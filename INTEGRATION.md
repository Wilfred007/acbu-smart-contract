# Contract Integration Guide

This guide explains how to integrate ACBU Soroban contracts with the backend services.

## Backend Integration

The backend provides TypeScript services for interacting with deployed contracts.

### Setup

1. **Install dependencies:**
   ```bash
   cd backend
   npm install
   ```

2. **Set environment variables:**
   ```bash
   export STELLAR_NETWORK="testnet"
   export STELLAR_SECRET_KEY="your-secret-key"
   export STELLAR_HORIZON_URL="https://horizon-testnet.stellar.org"
   export CONTRACT_ORACLE="<oracle-contract-id>"
   export CONTRACT_RESERVE_TRACKER="<reserve-tracker-contract-id>"
   export CONTRACT_MINTING="<minting-contract-id>"
   export CONTRACT_BURNING="<burning-contract-id>"
   ```

### Using Contract Services

#### Minting Service

```typescript
import { mintingService } from './services/contracts';

// Mint ACBU from USDC
const result = await mintingService.mintFromUsdc({
  usdcAmount: '10000000', // 10 USDC (7 decimals)
  recipient: 'G...', // Stellar address
});

console.log('Transaction:', result.transactionHash);
console.log('ACBU minted:', result.acbuAmount);

// Check if contract is paused
const isPaused = await mintingService.isPaused();

// Get fee rate
const feeRate = await mintingService.getFeeRate();
```

#### Burning Service

```typescript
import { burningService } from './services/contracts';

// Burn ACBU for currency redemption
const result = await burningService.burnForCurrency({
  acbuAmount: '10000000', // 10 ACBU
  currency: 'NGN',
  recipientAccount: {
    accountNumber: '1234567890',
    bankCode: '058',
    accountName: 'John Doe',
  },
});

console.log('Transaction:', result.transactionHash);
console.log('Local amount:', result.localAmount);
```

#### Oracle Service

```typescript
import { oracleService } from './services/contracts';

// Update exchange rate (validator function)
await oracleService.updateRate({
  currency: 'NGN',
  rate: '1234567', // 0.1234567 USD per NGN (7 decimals)
  sources: ['1230000', '1235000', '1239000'],
  timestamp: Date.now(),
});

// Get current rate
const rate = await oracleService.getRate('NGN');

// Get ACBU/USD rate
const acbuRate = await oracleService.getAcbuUsdRate();
```

#### Reserve Tracker Service

```typescript
import { reserveTrackerService } from './services/contracts';

// Update reserve (backend function)
await reserveTrackerService.updateReserve({
  currency: 'NGN',
  amount: '1000000000', // Reserve amount
  valueUsd: '123456700', // Value in USD
});

// Get reserve data
const reserve = await reserveTrackerService.getReserve('NGN');

// Verify reserves
const isValid = await reserveTrackerService.verifyReserves();

// Get total reserve value
const totalValue = await reserveTrackerService.getTotalReserveValue();
```

### Event Listening

Listen to contract events for off-chain processing:

```typescript
import { eventListener } from './services/stellar/eventListener';
import { mintingService, burningService } from './services/contracts';

// Listen to MintEvent
eventListener.on('mint', async (event) => {
  console.log('Mint event:', event);
  
  // Trigger USDC conversion worker
  // Process the mint event
});

// Listen to BurnEvent
eventListener.on('burn', async (event) => {
  console.log('Burn event:', event);
  
  // Trigger withdrawal processor
  // Process the burn event
});

// Start listening
await eventListener.start();
```

### Error Handling

All contract services throw errors that should be caught:

```typescript
try {
  const result = await mintingService.mintFromUsdc({
    usdcAmount: '10000000',
    recipient: 'G...',
  });
} catch (error) {
  if (error.message.includes('Insufficient reserves')) {
    // Handle insufficient reserves
  } else if (error.message.includes('Contract is paused')) {
    // Handle paused contract
  } else {
    // Handle other errors
  }
}
```

## Contract Interaction Flow

### Minting Flow

1. User deposits USDC or fiat
2. Backend calls `mintingService.mintFromUsdc()` or `mintingService.mintFromFiat()`
3. Contract verifies reserves and mints ACBU
4. Contract emits `MintEvent`
5. Backend event listener processes event
6. Backend triggers USDC conversion worker (if needed)

### Burning Flow

1. User requests ACBU redemption
2. Backend calls `burningService.burnForCurrency()` or `burningService.burnForBasket()`
3. Contract burns ACBU and emits `BurnEvent`
4. Backend event listener processes event
5. Backend triggers withdrawal processor
6. Fiat is disbursed to user's account

### Oracle Update Flow

1. Validator fetches rates from multiple sources
2. Validator calls `oracleService.updateRate()`
3. Contract calculates median and updates rate
4. Contract emits `RateUpdateEvent`
5. Backend updates database with new rates

### Reserve Update Flow

1. Backend tracks reserves from fintech partners
2. Backend calls `reserveTrackerService.updateReserve()`
3. Contract stores reserve data
4. Backend verifies reserves using `reserveTrackerService.verifyReserves()`

## Testing

### Unit Tests

```typescript
import { mintingService } from './services/contracts';

describe('MintingService', () => {
  it('should mint ACBU from USDC', async () => {
    const result = await mintingService.mintFromUsdc({
      usdcAmount: '10000000',
      recipient: 'G...',
    });
    
    expect(result.transactionHash).toBeDefined();
    expect(result.acbuAmount).toBeDefined();
  });
});
```

### Integration Tests

Test full flows with deployed contracts:

```typescript
describe('Mint-Burn Flow', () => {
  it('should complete full mint and burn cycle', async () => {
    // Mint
    const mintResult = await mintingService.mintFromUsdc({...});
    
    // Burn
    const burnResult = await burningService.burnForCurrency({...});
    
    expect(mintResult.transactionHash).toBeDefined();
    expect(burnResult.transactionHash).toBeDefined();
  });
});
```

## Best Practices

1. **Always check contract state** before operations (paused, fee rates, etc.)
2. **Handle errors gracefully** with appropriate user feedback
3. **Monitor events** for off-chain processing
4. **Verify reserves** before minting operations
5. **Use proper error handling** for all contract calls
6. **Log all transactions** for audit purposes
7. **Test thoroughly** on testnet before mainnet

## Security Considerations

1. **Never expose secret keys** in client-side code
2. **Validate all inputs** before calling contracts
3. **Use rate limiting** for contract operations
4. **Monitor for suspicious activity**
5. **Implement circuit breakers** for contract failures
6. **Use multisig** for admin operations
