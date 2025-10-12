# Custom Scripting Language

### Structure:
- `/parser_core`: core parser and lexer
- `/evaluator_core`: core evaluator

#### Examples:
```js
// gets 10 stocks in uptrend and mc below 50m, sorted by rsi - returns stocks objects
FILTER(items=[stocks], conditions=[MA(36) > MA(58) AND market_cap <= 50000000])
    & SORT(property=RSI(14), dir=asc, limit=10)  

// gets 10 stocks in uptrend and mc below 50m, sorted by rsi and CHARTs them to chart - returns table and chart data
FILTER(items=[stocks], conditions=[MA(36) > MA(58) AND market_cap <= 50000000])
    & SORT(property=RSI(14), dir=asc, limit=10)
    & CHART(from=2019-01-01, to=today)

// CHARTs basket od stocks from date to today on daily timeframe
CHART(items=[(AAPL + MSFT + GOOG + AMZN) / 4], from=2010-01-01, to=today, timeframe=1d)

// CHARTs aapl msft ration and rsi for spy
CHART(items=[AAPL / MSFT, RSI(14, SPY)], ...) // CHARTs ratio of aapl and msft and rsi for spy

// filters all stocks and backtests all slovenian stocks with entry condition (ma crossed above other ma) and exit condition, tests with size 1000 and fee 0.05% and CHARTs these slovenian stocks on chart (as well as mas)
FILTER(items=[stocks], conditions=[country=si]) & BACKTEST(entry=[MA(36) > MA(58)], exit=[MA(36) > MA(58)], size=1000, fee=0.05) & CHART

// gets all stocks that have mas crossed on (this day!) and sorts them by market cap
FILTER(items=[stocks], conditions=[CROSSES(MA(36), MA(58), up)]) & SORT(property=market_cap, dir=desc)

// backtests krka with this conditions and size, fee and CHARTs krka and mas charts
BACKTEST(items=[KRKA], entry=[MA(36) > MA(58)], exit=[MA(36) > MA(58)], size=1000, fee=0.05) & CHART

// backtests krka entry is only when SBITOP is in uptrend and rsi below 30, exit only on rsi above 70 or sbitop in downtrend, and CHARTs 
BACKTEST(items=[KRKA], entry=[RSI(14) < 30 AND MA(36, SBITOP) > MA(58, SBITOP)], exit=[(MA(36, SBITOP) < MA(58, SBITOP)) OR RSI(14) > 70], size=1000, fee=0.05) & CHART

// takes only slovenian stocks with market cap under 1B, sorts them by volatility (desc) and groups them into 2 groups: bull and bear, and CHARTs only the bull trend stocks
FILTER(items=[stocks], conditions=[country=si AND market_cap <= 1000000000])
    & SORT(property=BBWP(), dir=asc)
    & GROUP(conditions=[MA(36) > MA(58)], name="bull trend")
    & GROUP(conditions=[MA(36) < MA(58)], name="bear trend")
    & CHART(items=["bull trend"], from=2024-01-01, to=today)
```

#### Defaults:
```
to=today
from=first price in db
rebase=no rebase
timeframe=1d
```

#### Rules:
- SORT can only come AFTER FILTER


### Response Examples
```js
FILTER(items=[stocks], conditions=[MA(36) > MA(58) AND market_cap <= 50000000])
    & SORT(property=RSI(14), dir=asc, limit=10)
    & CHART(from=2024-01-01, to=today)

// response
{
    // sorted and filtered (Stock | Index)[]
    "matching_items": [
        Stock {}, Stock {}, ...
    ],
    // because of CHART this is added
    "charts": {
        "{stock_1_id}": {x, y}[],   
        "{stock_1_volume}": {x, y}[],
        "{stock_1_ma_36}": {x, y}[],
        "{stock_1_ma_58}": {x, y}[],
        "{stock_1_rsi_14}": {x, y}[],
        ... // for each stock
    }
}
```

#### Command to update parser to frontend
wasm-pack build --target web --out-dir ../../frontend/public/wasm