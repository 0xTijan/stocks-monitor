# Custom Financial Scripting Language Documentation

## Overview
This scripting language allows users to query, filter, sort, plot, and backtest financial data. It is designed to be intuitive yet powerful, enabling the creation of data pipelines using a series of **commands** chained with the `&` operator.

Each command performs a specific task (filtering, sorting, plotting, or backtesting) and passes its output to the next command in the pipeline.

---

## 1. Program Structure

A **program** is a sequence of one or more commands separated by the `&` symbol:

```dsl
command & command & command
```

For example:

```dsl
FILTER(items=[stocks], conditions=[MA(36) > MA(58)]) & SORT(item=RSI(14), dir=asc, limit=10)
```

This will:
1. Filter all stocks where the 36-period moving average is above the 58-period moving average.
2. Sort the filtered list by their 14-period RSI in ascending order and return the top 10 results.

---

## 2. Commands

### 2.1 FILTER
**Purpose:** Selects a subset of items based on logical conditions.

**Syntax:**
```dsl
FILTER(items=[list_of_items], conditions=[logical_expression])
```

**Arguments:**
- `items` (**required**) – The set of instruments to filter (e.g., `[stocks]`, `[AAPL, MSFT]`).
- `conditions` (**required**) – A logical expression defining the filter criteria.

**Examples:**
```dsl
FILTER(items=[stocks], conditions=[MA(36) > MA(58)])
FILTER(items=[stocks], conditions=[market_cap <= 50000000])
FILTER(items=[stocks], conditions=[RSI(14) < 30 AND MA(50) > MA(200)])
```

---

### 2.2 SORT
**Purpose:** Sorts the result of a previous `FILTER` command.

**Syntax:**
```dsl
SORT(item=expression, dir=asc|desc, limit=number)
```

**Arguments:**
- `item` (**required**) – The metric to sort by (e.g., `RSI(14)`, `market_cap`).
- `dir` – Sort direction: `asc` (ascending) or `desc` (descending).
- `limit` – Number of results to return.

**Examples:**
```dsl
FILTER(items=[stocks], conditions=[market_cap > 100000000]) & SORT(item=market_cap, dir=desc, limit=5)
FILTER(items=[stocks], conditions=[RSI(14) > 50]) & SORT(item=RSI(14), dir=asc, limit=20)
```

> **Rule:** `SORT` can only be used after `FILTER`.

---

### 2.3 PLOT
**Purpose:** Visualizes items on a chart.

**Syntax:**
```dsl
PLOT(items=[list_of_items], from=date, to=date, timeframe=duration, rebase=yes|no)
```

**Arguments:**
- `items` – List of symbols or arithmetic expressions.
- `from` – Start date (`YYYY-MM-DD`), default: first available.
- `to` – End date, default: today.
- `timeframe` – Chart resolution: `1d`, `1w`, `1m`, etc.
- `rebase` – Rebase values to a common starting point.

**Examples:**
```dsl
PLOT(items=[AAPL, MSFT], from=2020-01-01, to=today, timeframe=1d)
PLOT(items=[(AAPL + MSFT) / 2], from=2015-01-01, timeframe=1w)
PLOT(items=[AAPL / MSFT, RSI(14, SPY)])
```

---

### 2.4 BACKTEST
**Purpose:** Tests a trading strategy over historical data.

**Syntax:**
```dsl
BACKTEST(items=[list_of_items], entry=[logical_expression], exit=[logical_expression], size=number, fee=number)
```

**Arguments:**
- `items` (**required**) – Instruments to test.
- `entry` (**required**) – Conditions to enter trades.
- `exit` (**required**) – Conditions to exit trades.
- `size` – Trade size per position.
- `fee` – Transaction cost (as a percentage).

**Examples:**
```dsl
BACKTEST(items=[KRKA], entry=[RSI(14) < 30], exit=[RSI(14) > 70], size=1000, fee=0.05)
BACKTEST(items=[AAPL], entry=[MA(36) > MA(58)], exit=[MA(36) < MA(58)], size=500, fee=0.1)
```

Backtesting can be combined with plotting:
```dsl
BACKTEST(items=[AAPL], entry=[RSI(14) < 30], exit=[RSI(14) > 70], size=1000, fee=0.05) & PLOT
```

---

## 3. Expressions

### 3.1 Logical Expressions
Used in conditions for filtering and backtesting:
```dsl
MA(36) > MA(58) AND market_cap <= 50000000
RSI(14) < 30 OR volume > 1000000
```

Operators:
- `AND`, `OR`
- Comparators: `>`, `<`, `>=`, `<=`, `=`
- Parentheses for grouping: `( ... )`

---

### 3.2 Arithmetic Expressions
Used for calculations:
```dsl
(AAPL + MSFT + GOOG) / 3
AAPL / MSFT
```

Operators:
- `+` (addition)
- `/` (division)

---

### 3.3 Data Types
- **Numbers:** `100`, `0.05`
- **Strings:** `"USD"`
- **Dates:** `2020-05-01`
- **Durations:** `1d`, `2w`, `6m`
- **Lists:** `[AAPL, MSFT, GOOG]`
- **Tuples:** `(AAPL + MSFT) / 2`
- **Keywords:** `today`

---

### 3.4 Keywords
| Keyword    | Purpose               |
|------------|----------------------|
| today     | Select instruments    |
| stocks       | Order results         |
| indexes       | Visualize data        |
| asc   | Run strategy tests    |
| desc   | Run strategy tests    |

---

## 4. Defaults
If not explicitly provided:
```
to = today
from = first available price
rebase = no rebase
timeframe = 1d
```

---

## 5. Examples of Full Pipelines

### Example 1: Filter → Sort
```dsl
FILTER(items=[stocks], conditions=[MA(36) > MA(58) AND market_cap <= 50000000]) & 
SORT(item=RSI(14), dir=asc, limit=10)
```

### Example 2: Filter → Sort → Plot
```dsl
FILTER(items=[stocks], conditions=[RSI(14) < 30]) & 
SORT(item=market_cap, dir=desc, limit=5) & 
PLOT(from=2018-01-01, to=today, timeframe=1d)
```

### Example 3: Backtest with Conditions
```dsl
BACKTEST(items=[KRKA], 
         entry=[RSI(14) < 30 AND MA(36, SBITOP) > MA(58, SBITOP)], 
         exit=[(MA(36, SBITOP) < MA(58, SBITOP)) OR RSI(14) > 70], 
         size=1000, fee=0.05) & 
PLOT
```

### Example 4: Plot Ratios
```dsl
PLOT(items=[AAPL / MSFT, RSI(14, SPY)], from=2010-01-01, to=today, timeframe=1d)
```

---

## 6. Best Practices

- Always ensure parentheses match correctly in logical and arithmetic expressions.
- Start simple and test each command before chaining.
- Use clear and explicit arguments to avoid ambiguity.
- Follow the pipeline rules: `FILTER → SORT → PLOT/BACKTEST`.

---

## 7. Quick Reference

| Command    | Purpose               |
|------------|----------------------|
| FILTER     | Select instruments    |
| SORT       | Order results         |
| PLOT       | Visualize data        |
| BACKTEST   | Run strategy tests    |

---
