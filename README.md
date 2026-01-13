Program to implement a single liquidity pool for two SPL tokens, where swaps follow the invariant:
  x * y = k

```text
                           
                    ┌─────────────────────┐
                    │     user wallet     │
                    └─────────┬───────────┘
                              ▼
        ┌────────────────────────────────────────┐
        │        Anchor Program: token_swap      │
        │  ┌──────────────────────────────────┐  │
        │  │            Pool PDA              │  │
        │  │  seeds = ["pool", mint_a, mint_b]│  │
        │  │  mint_a  ─────────────┐          │  │
        │  │  mint_b  ─────────────┤  state   │  │
        │  │  vault_a ─────────────┤          │  │
        │  │  vault_b ─────────────┘          │  │
        │  └──────────────┬───────────────────┘  │
        │                 │ authority            │
        │                 ▼                      │
        │      ┌──────────────────────────┐      │
        │      │        Vault A           │      │
        │      │  SPL Token Account       │      │
        │      │  mint: Token A           │      │
        │      │  authority: Pool PDA     │      │
        │      └──────────────────────────┘      │
        │      ┌──────────────────────────┐      │
        │      │        Vault B           │      │
        │      │  SPL Token Account       │      │
        │      │  mint: Token B           │      │
        │      │  authority: Pool PDA     │      │
        │      └──────────────────────────┘      │
        └────────────────────────────────────────┘
                              ▲
                              │ CPI transfers
                              │
        ┌────────────────────────────────────────┐
        │          User Token Accounts (ATAs)    │
        │  ┌──────────────────────────┐          │
        │  │      User ATA (Token A)  │          │
        │  │  mint: Token A           │          │
        │  │  authority: user         │◄─ dx     │
        │  └──────────────────────────┘          │
        │  ┌──────────────────────────┐          │
        │  │      User ATA (Token B)  │          │
        │  │  mint: Token B           │          │
        │  │  authority: user         │◄─ dy     │
        │  └──────────────────────────┘          │
        └────────────────────────────────────────┘
```
