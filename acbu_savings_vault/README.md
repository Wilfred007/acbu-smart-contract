# acbu_savings_vault

Lock ACBU for fixed/rolling terms; yield accrual. Part of ACBU smart-contract-first protocols.

## Functions

- `initialize(admin, acbu_token, fee_rate_bps)` — Initialize the vault
- `deposit(user, amount, term_seconds)` — Lock ACBU for a term
- `withdraw(user, term_seconds, amount)` — Unlock ACBU after term
- `get_balance(user, term_seconds)` — Get locked balance
- `pause` / `unpause` — Admin only

## Events

- `DepositEvent` — On deposit
- `WithdrawEvent` — On withdraw
