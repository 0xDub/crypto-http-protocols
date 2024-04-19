// Objective: Iterate through exchanges and their endpoints, ping their servers to see which HTTP protocols are supported
use tokio::sync::Mutex;
use dashmap::DashMap;
use dotenv::dotenv;
use std::sync::Arc;
use log::info;

mod cl;
use cl::CL;

mod utils;
use utils::*;

mod model;
use model::*;


fn main() {
    dotenv().ok();
    env_logger::init();

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async move {
        let experiments: DashMap<Exchange, Arc<Mutex<Vec<Endpoint>>>> = DashMap::new();

        let mut endpoints = Vec::new();
        endpoints.push(Endpoint::new(MT::SPOT, "https://api.kucoin.com/api/v3/currencies/BTC"));
        endpoints.push(Endpoint::new(MT::SWAP, "https://api-futures.kucoin.com/api/v1/level2/snapshot?symbol=XBTUSDM"));
        experiments.insert(Exchange::KuCoin, Arc::new(Mutex::new(endpoints)));

        let mut endpoints = Vec::new();
        endpoints.push(Endpoint::new(MT::SPOT, "https://api.gemini.com/v1/book/btcusd"));
        experiments.insert(Exchange::Gemini, Arc::new(Mutex::new(endpoints)));

        let mut endpoints = Vec::new();
        endpoints.push(Endpoint::new(MT::SPOT, "https://api.gateio.ws/api/v4/spot/order_book?currency_pair=BTC_USDT"));
        endpoints.push(Endpoint::new(MT::DELIVERY, "https://api.gateio.ws/api/v4/delivery/usdt/contracts"));
        endpoints.push(Endpoint::new(MT::SWAP, "https://api.gateio.ws/api/v4/futures/usdt/order_book?contract=BTC_USDT"));
        endpoints.push(Endpoint::new(MT::OPTIONS, "https://api.gateio.ws/api/v4/options/underlyings"));
        experiments.insert(Exchange::GateIO, Arc::new(Mutex::new(endpoints)));

        let mut endpoints = Vec::new();
        endpoints.push(Endpoint::new(MT::OPTIONS, "https://www.deribit.com/api/v2/public/get_order_book?depth=5&instrument_name=BTC-PERPETUAL"));
        experiments.insert(Exchange::Deribit, Arc::new(Mutex::new(endpoints)));

        let mut endpoints = Vec::new();
        endpoints.push(Endpoint::new(MT::SPOT, "https://api.crypto.com/v2/public/get-book?instrument_name=BTCUSDT&depth=10"));
        endpoints.push(Endpoint::new(MT::SWAP, "https://api.crypto.com/v2/public/get-book?instrument_name=BTCUSD-PERP&depth=10"));
        experiments.insert(Exchange::CryptoCom, Arc::new(Mutex::new(endpoints)));

        let mut endpoints = Vec::new();
        endpoints.push(Endpoint::new(MT::INCLUSIVE, "https://whitebit.com/api/v1/public/depth/result?market=BTC_USDT"));
        endpoints.push(Endpoint::new(MT::INCLUSIVE, "https://whitebit.com/api/v2/public/depth/BTC_USDT"));
        endpoints.push(Endpoint::new(MT::INCLUSIVE, "https://whitebit.com/api/v4/public/orderbook/BTC_USDT?limit=100&level=2"));
        experiments.insert(Exchange::Whitebit, Arc::new(Mutex::new(endpoints)));

        let mut endpoints = Vec::new();
        endpoints.push(Endpoint::new(MT::SPOT, "https://api.woo.network/v1/public/orderbook/SPOT_BTC_USDT"));
        experiments.insert(Exchange::WooX, Arc::new(Mutex::new(endpoints)));

        let mut endpoints = Vec::new();
        endpoints.push(Endpoint::new(MT::SPOT, "https://api.poloniex.com/markets/BTC_USDT/orderBook"));
        endpoints.push(Endpoint::new(MT::SWAP, "https://futures-api.poloniex.com/api/v1/level2/snapshot?symbol=BTCUSDTPERP"));
        experiments.insert(Exchange::Poloniex, Arc::new(Mutex::new(endpoints)));

        let mut endpoints = Vec::new();
        endpoints.push(Endpoint::new(MT::SPOT, "https://ascendex.com/api/pro/v1/depth?symbol=BTC/USDT"));
        endpoints.push(Endpoint::new(MT::SWAP, "https://ascendex.com/api/pro/v2/futures/pricing-data"));
        experiments.insert(Exchange::AscendEx, Arc::new(Mutex::new(endpoints)));

        let mut endpoints = Vec::new();
        endpoints.push(Endpoint::new(MT::SPOT, "https://api.binance.com/api/v3/depth?symbol=BTCUSDT"));
        endpoints.push(Endpoint::new(MT::SPOT, "https://api-gcp.binance.com/api/v3/depth?symbol=BTCUSDT"));
        endpoints.push(Endpoint::new(MT::SPOT, "https://api1.binance.com/api/v3/depth?symbol=BTCUSDT"));
        endpoints.push(Endpoint::new(MT::SPOT, "https://api2.binance.com/api/v3/depth?symbol=BTCUSDT"));
        endpoints.push(Endpoint::new(MT::SPOT, "https://api3.binance.com/api/v3/depth?symbol=BTCUSDT"));
        endpoints.push(Endpoint::new(MT::SPOT, "https://api4.binance.com/api/v3/depth?symbol=BTCUSDT"));
        endpoints.push(Endpoint::new(MT::SWAP, "https://fapi.binance.com/fapi/v1/depth?symbol=BTCUSDT"));
        endpoints.push(Endpoint::new(MT::COIN, "https://dapi.binance.com/dapi/v1/depth?symbol=BTCUSDT"));
        endpoints.push(Endpoint::new(MT::OPTIONS, "https://eapi.binance.com/eapi/v1/depth?symbol=BTCUSDT"));
        experiments.insert(Exchange::Binance, Arc::new(Mutex::new(endpoints)));

        let mut endpoints = Vec::new();
        endpoints.push(Endpoint::new(MT::SPOT, "https://api.binance.us/api/v3/depth?symbol=BTCUSD"));
        experiments.insert(Exchange::BinanceUS, Arc::new(Mutex::new(endpoints)));

        let mut endpoints = Vec::new();
        endpoints.push(Endpoint::new(MT::SPOT, "https://open-api.bingx.com/openApi/spot/v1/market/depth?symbol=BTC-USDT"));
        endpoints.push(Endpoint::new(MT::SWAP, "https://open-api.bingx.com/openApi/swap/v2/quote/depth?symbol=BTC-USDT"));
        experiments.insert(Exchange::BingX, Arc::new(Mutex::new(endpoints)));

        let mut endpoints = Vec::new();
        endpoints.push(Endpoint::new(MT::INCLUSIVE, "https://api.bitfinex.com/v1/book/BTCUSD"));
        endpoints.push(Endpoint::new(MT::INCLUSIVE, "https://api-pub.bitfinex.com/v2/book/tBTCUSD/P0"));
        experiments.insert(Exchange::Bitfinex, Arc::new(Mutex::new(endpoints)));

        let mut endpoints = Vec::new();
        endpoints.push(Endpoint::new(MT::SPOT, "https://api.bitget.com/api/v2/spot/market/orderbook?symbol=BTCUSDT&type=step0&limit=100"));
        endpoints.push(Endpoint::new(MT::SWAP, "https://api.bitget.com/api/v2/mix/market/merge-depth?productType=usdt-futures&symbol=BTCUSDT"));
        experiments.insert(Exchange::Bitget, Arc::new(Mutex::new(endpoints)));

        let mut endpoints = Vec::new();
        endpoints.push(Endpoint::new(MT::SPOT, "https://api-cloud.bitmart.com/spot/quotation/v3/books?symbol=BTC_USDT&limit=1"));
        endpoints.push(Endpoint::new(MT::SWAP, "https://api-cloud.bitmart.com/contract/public/depth"));
        experiments.insert(Exchange::BitMart, Arc::new(Mutex::new(endpoints)));

        let mut endpoints = Vec::new();
        endpoints.push(Endpoint::new(MT::INCLUSIVE, "https://www.bitmex.com/api/v1/orderBook/L2?symbol=XBTUSD&depth=1"));
        experiments.insert(Exchange::BitMex, Arc::new(Mutex::new(endpoints)));

        let mut endpoints = Vec::new();
        endpoints.push(Endpoint::new(MT::INCLUSIVE, "https://openapi.blofin.com/api/v1/market/books?instId=BTC-USDT"));
        experiments.insert(Exchange::BloFin, Arc::new(Mutex::new(endpoints)));

        let mut endpoints = Vec::new();
        endpoints.push(Endpoint::new(MT::INCLUSIVE, "https://api.bybit.com/v5/market/orderbook?category=spot&symbol=BTCUSDT"));
        endpoints.push(Endpoint::new(MT::INCLUSIVE, "https://api.bytick.com/v5/market/orderbook?category=spot&symbol=BTCUSDT"));
        experiments.insert(Exchange::Bybit, Arc::new(Mutex::new(endpoints)));

        let mut endpoints = Vec::new();
        endpoints.push(Endpoint::new(MT::INCLUSIVE, "https://trade.cex.io/api/spot/rest-public/get_order_book?pair=BTC-USD"));
        experiments.insert(Exchange::CEX, Arc::new(Mutex::new(endpoints)));

        let mut endpoints = Vec::new();
        endpoints.push(Endpoint::new(MT::SPOT, "https://api.exchange.coinbase.com/products/BTC-USD/book"));
        endpoints.push(Endpoint::new(MT::INCLUSIVE, "https://api.coinbase.com/api/v3/brokerage/time"));
        experiments.insert(Exchange::Coinbase, Arc::new(Mutex::new(endpoints)));

        let mut endpoints = Vec::new();
        endpoints.push(Endpoint::new(MT::SPOT, "https://api.coinex.com/v2/spot/depth?market=BTCUSDT&limit=100&interval=0"));
        endpoints.push(Endpoint::new(MT::SWAP, "https://api.coinex.com/v2/futures/depth?market=BTCUSDT&limit=100&interval=0"));
        experiments.insert(Exchange::CoinEx, Arc::new(Mutex::new(endpoints)));

        let mut endpoints = Vec::new();
        endpoints.push(Endpoint::new(MT::SPOT, "https://api.huobi.pro/market/depth?symbol=btcusdt&type=step0"));
        endpoints.push(Endpoint::new(MT::SPOT, "https://api-aws.huobi.pro/market/depth?symbol=btcusdt&type=step0"));
        endpoints.push(Endpoint::new(MT::DELIVERY, "https://api.hbdm.com/market/depth?symbol=BTC_CQ&type=step5"));
        endpoints.push(Endpoint::new(MT::DELIVERY, "https://api.hbdm.vn/market/depth?symbol=BTC_CQ&type=step5"));
        endpoints.push(Endpoint::new(MT::COIN, "https://api.hbdm.com/swap-ex/market/depth?contract_code=BTC-USD&type=step5"));
        endpoints.push(Endpoint::new(MT::COIN, "https://api.hbdm.vn/swap-ex/market/depth?contract_code=BTC-USD&type=step5"));
        endpoints.push(Endpoint::new(MT::SWAP, "https://api.hbdm.com/linear-swap-ex/market/depth?contract_code=BTC-USDT&type=step5"));
        endpoints.push(Endpoint::new(MT::SWAP, "https://api.hbdm.vn/linear-swap-ex/market/depth?contract_code=BTC-USDT&type=step5"));
        experiments.insert(Exchange::HTX, Arc::new(Mutex::new(endpoints)));

        let mut endpoints = Vec::new();
        endpoints.push(Endpoint::new(MT::SPOT, "https://api.kraken.com/0/public/Depth?pair=XBTUSD"));
        endpoints.push(Endpoint::new(MT::INCLUSIVE, "https://futures.kraken.com/derivatives/api/v3/orderbook?symbol=PI_XBTUSD"));
        experiments.insert(Exchange::Kraken, Arc::new(Mutex::new(endpoints)));

        let mut endpoints = Vec::new();
        endpoints.push(Endpoint::new(MT::INCLUSIVE, "https://www.okx.com/api/v5/market/books?instId=BTC-USDT"));
        experiments.insert(Exchange::OKX, Arc::new(Mutex::new(endpoints)));

        let mut endpoints = Vec::new();
        endpoints.push(Endpoint::new(MT::INCLUSIVE, "https://www.okcoin.com/api/v5/market/books?instId=BTC-USD"));
        experiments.insert(Exchange::OkCoin, Arc::new(Mutex::new(endpoints)));


        

        info!("{}=-------------------------------------------------------------------={}", CL::Purple.get(), CL::End.get());
        info!("{}[+][+][+][+][+][+][+][+] Starting Experiment [+][+][+][+][+][+][+][+]{}", CL::Purple.get(), CL::End.get());
        info!("{}=-------------------------------------------------------------------={}", CL::Purple.get(), CL::End.get());
        info!("{}[-] Please wait ~30sec while we gather data from exchanges...{}", CL::Dull.get(), CL::End.get());
        info!("");
        info!("");




        let mut tasks = Vec::new();
        let keys: Vec<Exchange> = experiments.iter().map(|pair| pair.key().clone()).collect();
        for key in keys {
            let endpoints = experiments.get_mut(&key).unwrap();

            let endpoints = Arc::clone(&endpoints);
            let task = tokio::spawn(async move {
                let roots = get_roots();
                let tls_config: rustls::ClientConfig = get_config(roots).await;

                let h1_client = reqwest::Client::builder()
                    .http1_only()
                    .use_rustls_tls()
                    .timeout(std::time::Duration::from_secs(30))
                    .build().expect("[!] Failed to build client");

                let h2_client = reqwest::Client::builder()
                    .http2_prior_knowledge()
                    .use_rustls_tls()
                    .timeout(std::time::Duration::from_secs(30))
                    .build().expect("[!] Failed to build client");

                for endpoint in endpoints.lock().await.iter_mut() {
                    let uri = endpoint.uri;

                    if let Ok(res) = h1_client.get(uri).send().await {
                        if res.version() == reqwest::Version::HTTP_11 {
                            endpoint.h1 = true;
                        }
                    }

                    if let Ok(res) = h2_client.get(uri).send().await {
                        if res.version() == reqwest::Version::HTTP_2 {
                            endpoint.h2 = true;
                        }
                    }

                    if let Some((version, status_code)) = test_h3(uri, tls_config.clone()).await {
                        endpoint.h3 = true;
                    }

                }
            });
            tasks.push(task);

        }

        for task in tasks {
            task.await.expect("[!] Failed to spawn task");
        }



        // now we can iterate through all the experiments and print out the results

        for entry in experiments.iter() {
            let (exchange, endpoints) = entry.pair();

            info!("{}=------------------= Exchange: {:?} =------------------={}", CL::Teal.get(), exchange, CL::End.get());
            for endpoint in endpoints.lock().await.iter() {
                info!("{}-> {:?} | Endpoint: {} =-={}", CL::Dull.get(), endpoint.market_type, endpoint.uri, CL::End.get());

                let (mut h1_color, mut h2_color, mut h3_color) = (CL::Red, CL::Red, CL::Red);
                if endpoint.h1 { h1_color = CL::Green }
                if endpoint.h2 { h2_color = CL::Green }
                if endpoint.h3 { h3_color = CL::Green }

                info!("{}h1{} | {}h2{} | {}h3{}", h1_color.get(), CL::End.get(), h2_color.get(), CL::End.get(), h3_color.get(), CL::End.get());
            }
            info!("");
        }

    });
}