
// use std::io::Read;

extern crate curl;
use curl::easy::Easy;

extern crate stock_market_pred;
// use stock_market_pred::yahoo_finance_api::yahoo_finance_enums::{Pricing, YahooApiOption};

fn main() {
    let mut dst = Vec::new();
    let mut easy = Easy::new();
    easy.url("http://download.finance.yahoo.com/d/quotes.csv?s=GOOG&f=a").unwrap();

    let mut transfer = easy.transfer();
    transfer.write_function(|data| {
            dst.extend_from_slice(data);
            println!("{:?}", std::str::from_utf8(&dst[..]));
            Ok(data.len())
        })
        .unwrap();

    transfer.perform().unwrap();
}
