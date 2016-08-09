
mod yahoo_finance_api;

pub struct YahooCSVApi<T: YahooApiOption> {
    symbols: Vec<String>,
    options: Vec<T>
}

impl YahooCSVApi {
    fn new<T: YahooApiOption>(symbols: Vec<String>, options: Vec<T>) -> YahooCSVApi {
        YahooCSVApi {
            symbols: symbols,
            options: options
        }
    }
}
