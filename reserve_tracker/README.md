# Reserve Tracker Contract

The Reserve Tracker contract tracks reserve balances and verifies overcollateralization.

## Functions

### `initialize`
Initialize the contract.

**Parameters:**
- `admin`: Admin address (backend)
- `acbu_token`: ACBU token contract address
- `min_ratio_bps`: Minimum ratio in basis points (10200 = 102%)
- `target_ratio_bps`: Target ratio in basis points (10500 = 105%)

### `update_reserve`
Update reserve for a currency (backend function).

**Parameters:**
- `currency`: Currency code
- `amount`: Reserve amount in native currency (7 decimals)
- `value_usd`: Reserve value in USD (7 decimals)

**Access:** Admin only (backend)

### `get_reserve`
Get reserve data for a currency.

**Parameters:**
- `currency`: Currency code

**Returns:** ReserveData struct

### `verify_reserves`
Verify reserves meet overcollateralization requirements.

**Returns:** Boolean (true if reserves >= min ratio)

### `get_total_reserve_value`
Get total reserve value in USD.

**Returns:** Total value in 7 decimals

### `get_min_ratio` / `get_target_ratio`
Get minimum/target ratios.

**Returns:** Ratio in basis points

## Access Control

- **Update reserves:** Admin only (backend)
- **Read reserves:** Public
- **Verification:** Public

## Reserve Verification

The contract verifies that:
- Total reserve value >= ACBU supply × min_ratio
- Example: If ACBU supply = 1M and min_ratio = 102%, reserves must be >= 1.02M USD

## Integration

The contract is used by:
- **Minting Contract:** For reserve verification before minting
- **Burning Contract:** For reserve verification before burning
- **Backend:** For reserve updates and monitoring

## Security Features

- Overcollateralization checks
- Minimum ratio enforcement
- Real-time reserve tracking
- Public transparency
