
enum Pricing {
    Ask,
    Bid,
    AskRealtime,
    BidRealtime,
    PreviousClose,
    Open
}

fn translate_pricing(opt: Pricing) -> &'static str {
    match opt {
        Pricing::Ask => "a",
        Pricing::Bid => "b",
        Pricing::AskRealtime => "b2",
        Pricing::BidRealtime => "b3",
        Pricing::PreviousClose => "p",
        Pricing::Open => "o"
    }
}

enum Dividends {
    DividendYield,
    DividendPerShare,
    DividendPayDate,
    ExDividendDate
}

fn translate_dividens(opt: Dividends) -> &'static str {
    match opt {
        Dividends::DividendYield => "y",
        Dividends::DividendPerShare => "d",
        Dividends::DividendPayDate => "r1",
        Dividends::ExDividendDate => "q"
    }
}

enum Date {
    Change,
    ChangeAndPercentChange,
    ChangeRealtime,
    ChangePercentRealtime,
    ChangeInPercent,
    LastTradeDate,
    TradeDate,
    LastTradeTime
}

fn translate_date(opt: Date) -> &'static str {
    match opt {
        Date::Change => "c1",
        Date::ChangeAndPercentChange => "c",
        Date::ChangeRealtime => "c6",
        Date::ChangePercentRealtime => "k2",
        Date::ChangeInPercent => "p2",
        Date::LastTradeDate => "d1",
        Date::TradeDate => "d2",
        Date::LastTradeTime => "t1"
    }
}

enum Averages {
    AfterHoursChangeRealtime,
    Commission,
    DaysLow,
    DaysHigh,
    LastTradeRealtimeWithTime,
    LastTradeWithTime,
    LastTradePriceOnly,
    OneYearTargetPrice,
    ChangeFrom200DayMovingAverage,
    PercentChangeFrom200DayMovingAverage,
    ChangeFrom50DayMovingAverage,
    PercentChangeFrom50DayMovingAverage,
    FiftyDayMovingAverage,
    TwoHundredDayMovingAverage
}

fn translate_averages(opt: Averages) -> &'static str {
    match opt {
        Averages::AfterHoursChangeRealtime => "c8",
        Averages::Commission => "c3",
        Averages::DaysLow => "g",
        Averages::DaysHigh => "h",
        Averages::LastTradeRealtimeWithTime => "k1",
        Averages::LastTradeWithTime => "l",
        Averages::LastTradePriceOnly => "l1",
        Averages::OneYearTargetPrice => "t8",
        Averages::ChangeFrom200DayMovingAverage => "m5",
        Averages::PercentChangeFrom200DayMovingAverage => "m6",
        Averages::ChangeFrom50DayMovingAverage => "m7",
        Averages::PercentChangeFrom50DayMovingAverage => "m8",
        Averages::FiftyDayMovingAverage => "m3",
        Averages::TwoHundredDayMovingAverage => "m4"
    }
}

enum Misc {
    DaysValueChange,
    HoldingsGainPercent,
    DaysValueChangeRealtime,
    AnnualizedGain,
    PricePaid,
    HoldingsGain,
    DaysRange,
    HoldingsGainPercentRealtime,
    DaysRangeRealtime,
    HoldingsGainRealtime,
    TickerTrend,
    TradeLinks,
    OrderBookRealtime,
    HighLimit,
    LowLimit,
    HoldingsValue,
    HoldingsValueRealtime,
    Revenue
}

fn translate_misc(opt: Misc) -> &'static str {
    match opt {
        Misc::DaysValueChange => "w1",
        Misc::HoldingsGainPercent => "g1",
        Misc::DaysValueChangeRealtime => "w4",
        Misc::AnnualizedGain => "g3",
        Misc::PricePaid => "p1",
        Misc::HoldingsGain => "g4",
        Misc::DaysRange => "m",
        Misc::HoldingsGainPercentRealtime => "g5",
        Misc::DaysRangeRealtime => "m2",
        Misc::HoldingsGainRealtime => "g6",
        Misc::TickerTrend => "t7",
        Misc::TradeLinks => "t6",
        Misc::OrderBookRealtime => "i5",
        Misc::HighLimit => "l2",
        Misc::LowLimit => "l3",
        Misc::HoldingsValue => "v1",
        Misc::HoldingsValueRealtime => "v7",
        Misc::Revenue => "s6"
    }
}

enum WeekPricing {
    FiftyTwoWeekHigh,
    FiftyTwoWeekLow,
    ChangeFrom52WeekLow,
    ChangeFrom52WeekHigh,
    PercentChangeFrom52WeekLow,
    PercentChangeFrom52WeekHigh,
    FiftyTwoWeekRange
}

fn translate_week_pricing(opt: WeekPricing) -> &'static str {
    match opt {
        WeekPricing::FiftyTwoWeekHigh => "k",
        WeekPricing::FiftyTwoWeekLow => "j",
        WeekPricing::ChangeFrom52WeekLow => "j5",
        WeekPricing::ChangeFrom52WeekHigh => "k4",
        WeekPricing::PercentChangeFrom52WeekLow => "j6",
        WeekPricing::PercentChangeFrom52WeekHigh => "k5",
        WeekPricing::FiftyTwoWeekRange => "w"
    }
}

enum SymbolInfo {
    MoreInfo,
    MarketCapitalization,
    MarketCapRealTime,
    FloatShares,
    Name,
    Notes,
    Symbol,
    SharesOwned,
    StockExchange,
    SharesOutstanding
}

fn translate_symbol_info(opt: SymbolInfo) -> &'static str {
    match opt {
        SymbolInfo::MoreInfo => "i",
        SymbolInfo::MarketCapitalization => "j1",
        SymbolInfo::MarketCapRealTime => "j3",
        SymbolInfo::FloatShares => "f6",
        SymbolInfo::Name => "n",
        SymbolInfo::Notes => "n4",
        SymbolInfo::Symbol => "s",
        SymbolInfo::SharesOwned => "s1",
        SymbolInfo::StockExchange => "x",
        SymbolInfo::SharesOutstanding => "j2"
    }
}

enum Volume {
    Volume,
    AskSize,
    BidSize,
    LastTradeSize,
    AverageDailyVolume
}

fn translate_volume(opt: Volume) -> &'static str {
    match opt {
        Volume::Volume => "v",
        Volume::AskSize => "a5",
        Volume::BidSize => "b6",
        Volume::LastTradeSize => "k3",
        Volume::AverageDailyVolume => "a2"
    }
}

enum Ratios {
    EarningsPerShare,
    EPSEstimateCurrentYear,
    EPSEstimateNextYear,
    EPSEstimateNextQuarter,
    BookValue,
    EBITDA,
    PricePerSale,
    PricePerBook,
    PERatio,
    PERatioRealtime,
    PriceEPSEstimateCurrentYear,
    PriceEPSEstimateNextYear,
    ShortRatio
}

fn translate_ratios(opt: Ratios) -> &'static str {
    match opt {
        Ratios::EarningsPerShare => "e",
        Ratios::EPSEstimateCurrentYear => "e7",
        Ratios::EPSEstimateNextYear => "e8",
        Ratios::EPSEstimateNextQuarter => "e9",
        Ratios::BookValue => "b4",
        Ratios::EBITDA => "j4",
        Ratios::PricePerSale => "p5",
        Ratios::PricePerBook => "p6",
        Ratios::PERatio => "r",
        Ratios::PERatioRealtime => "r2",
        Ratios::PriceEPSEstimateCurrentYear => "r6",
        Ratios::PriceEPSEstimateNextYear => "r7",
        Ratios::ShortRatio => "s7"
    }
}

fn translate_opt<T>(opt: T) -> &'static str {
    match opt {}
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
