# 📉 Impermanent Loss Calculator

A Rust CLI tool that calculates impermanent loss for liquidity providers in AMM (Automated Market Maker) pools.

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
