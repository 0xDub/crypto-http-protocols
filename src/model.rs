

#[derive(Debug)]
pub enum MT {
    SPOT,
    SWAP,
    COIN,
    DELIVERY,
    OPTIONS,
    INCLUSIVE,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Exchange {
    KuCoin,
    Gemini,
    GateIO,
    Deribit,
    CryptoCom,
    Whitebit,
    WooX,
    Poloniex,
    AscendEx,
    Binance,
    BinanceUS,
    BingX,
    Bitfinex,
    Bitget,
    BitMart,
    BitMex,
    BloFin,
    Bybit,
    CEX,
    Coinbase,
    CoinEx,
    HTX,
    Kraken,
    OKX,
    OkCoin,
}

pub struct Endpoint<'a> {
    pub market_type: MT,
    pub uri: &'a str,
    pub h3: bool,
    pub h2: bool,
    pub h1: bool,
}

impl<'a> Endpoint<'a> {
    pub fn new(market_type: MT, uri: &'a str) -> Self {
        Self {
            market_type,
            uri,
            h3: false,
            h2: false,
            h1: false,
        }
    }
}