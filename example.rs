extern crate stock_market_pred;

use stock_market_pred::yahoo_finance_api::yahoo_finance_enums::{Pricing, YahooApiOption};

fn main() {
    let opt = Pricing::Ask;
    println!("TEST: {}", opt.translate());
}
