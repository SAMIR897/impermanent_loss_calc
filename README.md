# 📉 Impermanent Loss Calculator

An advanced DeFi calculator designed to help Liquidity Providers (LPs) estimate potential impermanent loss and overall profitability within AMMs. By simulating price volatility and fee accrual, it provides clear visualizations of risk-to-reward ratios, enabling users to make data-driven decisions when providing liquidity to decentralized exchanges and protocols.

## Features

- Calculates IL based on initial and final token price
- Shows the price ratio and percentage loss
- Provides a $100 deposit breakdown comparing holding vs. providing liquidity

## Usage

```bash
cargo run -- --initial-price 1000 --final-price 2000
```

### Sample Output

```
Initial Price: $1000.00
Final Price:   $2000.00
Price Ratio:   2.00x
Impermanent Loss: 5.72%
If you deposited $100:
- Held value: $200.00
- Pool value: $188.56
- Loss vs holding: $11.44
```

## Tech Stack

- `clap` — CLI argument parsing
- Pure Rust math — no external blockchain dependencies needed
