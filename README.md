# Crypto Exchange HTTP Protocol Scanner

Rust project for determining HTTP protocol support across these exchanges listed below. Most endpoints were order_book related since that's likely to be optimized and the exchange infra is inherently black-boxed. Private endpoints weren't checked, however, based on the results it looks like exchanges tend to keep the same protocol supports respective to the hostname. Hope y'all enjoy :)

## Setup
- `rustup default stable`
- `cargo run --release`
- wait 30sec
- `BOOM, results`

### Protocols monitored:
- `h1` (HTTP/1.1)
- `h2` (HTTP/2)
- `h3` (HTTP/3 - QUIC)

### Exchanges monitored:
- Coinbase
- Binance
- BinanceUS
- OKX
- Bybit
- Kraken
- Bitfinex
- BitMex
- HTX (Huobi)
- KuCoin
- Gemini
- GateIO
- Deribit
- Crypto.com
- Poloniex
- AscendEx
- Whitebit
- WooX
- BingX
- Bitget
- BitMart
- BloFin
- CEX
- CoinEx
- OkCoin

### Disclaimer:
- Exchanges listed are *not* endorsements