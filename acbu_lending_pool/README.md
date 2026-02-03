# acbu_lending_pool

Deposits, borrows, collateral, liquidation. P2P lending protocol. Part of ACBU smart-contract-first protocols.

## Functions

- `initialize(admin, acbu_token, fee_rate_bps)` — Initialize the pool
- `deposit(lender, amount)` — Supply ACBU to the pool
- `withdraw(lender, amount)` — Withdraw ACBU from the pool
- `get_balance(lender)` — Get lender balance
- `pause` / `unpause` — Admin only

## Events

- `LoanCreatedEvent` — On loan creation (extend for borrow/repay/liquidate)
- `RepaymentEvent` — On repayment
