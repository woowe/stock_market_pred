
enum Pricing {
    Ask,
    Bid,
    AskRealtime,
    BidRealtime,
    PreviousClose,
    Open,
}

enum Dividends {
    DividendYield,
    DividendPerShare,
    DividendPayDate,
    ExDividendDate,
}

enum Date {
    Change,
    ChangeAndPercentChange,
    ChangeRealtime,
    ChangePercentRealtime,
    ChangeInPercent,
    LastTradeDate,
    TradeDate,
    LastTradeTime,
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
    TwoHundredDayMovingAverage,
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
    Revenue,
}

enum WeekPricing {
    FiftyTwoWeekHigh,
    FiftyTwoWeekLow,
    ChangeFrom52WeekLow,
    ChangeFrom52WeekHigh,
    PercentChangeFrom52WeekLow,
    PercentChangeFrom52WeekHigh,
    FiftyTwoWeekRange,
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
    SharesOutstanding,
}

enum Volume {
    Volume,
    AskSize,
    BidSize,
    LastTradeSize,
    AverageDailyVolume,
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
    ShortRatio,
}

trait YahooApiOption {
    fn translate(&self) -> &'static str;
}

impl YahooApiOption for Pricing {
    fn translate(&self) -> &'static str {
        match *self {
            Pricing::Ask => "a",
            Pricing::Bid => "b",
            Pricing::AskRealtime => "b2",
            Pricing::BidRealtime => "b3",
            Pricing::PreviousClose => "p",
            Pricing::Open => "o",
        }
    }
}

impl YahooApiOption for Dividends {
    fn translate(&self) -> &'static str {
        match *self {
            Dividends::DividendYield => "y",
            Dividends::DividendPerShare => "d",
            Dividends::DividendPayDate => "r1",
            Dividends::ExDividendDate => "q",
        }
    }
}

impl YahooApiOption for Date {
    fn translate(&self) -> &'static str {
        match *self {
            Date::Change => "c1",
            Date::ChangeAndPercentChange => "c",
            Date::ChangeRealtime => "c6",
            Date::ChangePercentRealtime => "k2",
            Date::ChangeInPercent => "p2",
            Date::LastTradeDate => "d1",
            Date::TradeDate => "d2",
            Date::LastTradeTime => "t1",
        }
    }
}

impl YahooApiOption for Averages {
    fn translate(&self) -> &'static str {
        match *self {
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
            Averages::TwoHundredDayMovingAverage => "m4",
        }
    }
}

impl YahooApiOption for Misc {
    fn translate(&self) -> &'static str {
        match *self {
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
            Misc::Revenue => "s6",
        }
    }
}

impl YahooApiOption for WeekPricing {
    fn translate(&self) -> &'static str {
        match *self {
            WeekPricing::FiftyTwoWeekHigh => "k",
            WeekPricing::FiftyTwoWeekLow => "j",
            WeekPricing::ChangeFrom52WeekLow => "j5",
            WeekPricing::ChangeFrom52WeekHigh => "k4",
            WeekPricing::PercentChangeFrom52WeekLow => "j6",
            WeekPricing::PercentChangeFrom52WeekHigh => "k5",
            WeekPricing::FiftyTwoWeekRange => "w",
        }
    }
}

impl YahooApiOption for SymbolInfo {
    fn translate(&self) -> &'static str {
        match *self {
            SymbolInfo::MoreInfo => "i",
            SymbolInfo::MarketCapitalization => "j1",
            SymbolInfo::MarketCapRealTime => "j3",
            SymbolInfo::FloatShares => "f6",
            SymbolInfo::Name => "n",
            SymbolInfo::Notes => "n4",
            SymbolInfo::Symbol => "s",
            SymbolInfo::SharesOwned => "s1",
            SymbolInfo::StockExchange => "x",
            SymbolInfo::SharesOutstanding => "j2",
        }
    }
}

impl YahooApiOption for Volume {
    fn translate(&self) -> &'static str {
        match *self {
            Volume::Volume => "v",
            Volume::AskSize => "a5",
            Volume::BidSize => "b6",
            Volume::LastTradeSize => "k3",
            Volume::AverageDailyVolume => "a2",
        }
    }
}

impl YahooApiOption for Ratios {
    fn translate(&self) -> &'static str {
        match *self {
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
            Ratios::ShortRatio => "s7",
        }
    }
}
