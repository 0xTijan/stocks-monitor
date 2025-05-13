# Custom Scripting Language

### Structure:
- `/parser-core`: core parser and lexer
- `/interpreter-core`: core interpreter

#### Examples:
```js
// gets 10 stocks in uptrend and mc below 50m, sorted by rsi - returns stocks objects
FILTER(items=stocks, conditions=[MA(36) > MA(58), market_cap <= 50000000])
    & SORT(item=RSI(14), dir=asc, limit=10)  

// gets 10 stocks in uptrend and mc below 50m, sorted by rsi and plots them to chart - returns table and chart data
FILTER(items=stocks, conditions=[MA(36) > MA(58), market_cap <= 50000000])
    & SORT(item=RSI(14), dir=asc, limit=10)
    & PLOT(from=2019-01-01, to=today)

// plots basket od stocks from date to today on daily timeframe
PLOT(items=[(AAPL + MSFT + GOOG + AMZN) / 4], from=2010-01-01, to=today, timeframe=1d)

// plots aapl msft ration and rsi for spy
PLOT(items=[AAPL / MSFT, RSI(14, SPY) AS "SPY RSI"], ...) // plots ratio of aapl and msft and rsi for spy

// filters all stocks and backtests all slovenian stocks with entry condition (ma crossed above other ma) and exit condition, tests with size 1000 and fee 0.05% and plots these slovenian stocks on chart (as well as mas)
FILTER(items=[stocks], conditions=[country=si]) & BACKTEST(entry=[MA(36) > MA(58)], exit=[MA(36) > MA(58)], size=1000, fee=0.05%) & PLOT

// gets all stocks that have mas crossed on (this day!) and sorts them by market cap
FILTER(items=[stocks], condition=[CROSSES(MA(36), MA(58), up)]) & SORT(property=market_cap, dir=desc)

// backtests krka with this conditions and size, fee and plots krka and mas charts
BACKTEST(items=[KRKA], entry=[MA(36) > MA(58)], exit=[MA(36) > MA(58)], size=1000, fee=0.05) & PLOT
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