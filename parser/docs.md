# Custom Financial Scripting Language Documentation

## Overview
This scripting language allows users to query, filter, sort, chart, and backtest financial data. It is designed to be intuitive yet powerful, enabling the creation of data pipelines using a series of **commands** chained with the `&` operator.

Each command performs a specific task (filtering, sorting, CHARTING, or backtesting) and passes its output to the next command in the pipeline.

## Menu
| Section                                                        | Title                     | Description                                    |
| -------------------------------------------------------------- | ------------------------- | ---------------------------------------------- |
| [1. Program Structure](#1-program-structure)                   | Structure of DSL programs | How commands are chained using `&`.            |
| [2. Commands](#2-commands)                                     | Core command reference    | Overview of all available commands.            |
| [2.1 FILTER](#21-filter)                                       | Filter instruments        | Selects items based on logical conditions.     |
| [2.2 SORT](#22-sort)                                           | Sort results              | Sorts filtered items by metrics or indicators. |
| [2.3 CHART](#23-chart)                                         | Visualize data            | Plots instruments or expressions on charts.    |
| [2.4 BACKTEST](#24-backtest)                                   | Strategy testing          | Simulates trades over historical data.         |
| [2.5 GROUP](#25-group)                                         | Group instruments         | Divides instruments into named groups.         |
| [3. Expressions](#3-expressions)                               | Expression language       | Syntax for logic and arithmetic operations.    |
| [3.1 Logical Expressions](#31-logical-expressions)             | Conditional logic         | Used for filters and strategy rules.           |
| [3.2 Arithmetic Expressions](#32-arithmetic-expressions)       | Numeric operations        | Combine or compare instruments.                |
| [3.3 Data Types](#33-data-types)                               | Supported types           | Lists, numbers, strings, dates, etc.           |
| [3.4 Keywords](#34-keywords)                                   | Built-in keywords         | Predefined values like `today`, `stocks`.      |
| [3.5 Functions](#35-functions)                                 | Technical indicators      | Built-in functions like RSI, MACD, MA.         |
| [4. Defaults](#4-defaults)                                     | Default values            | What happens if arguments are omitted.         |
| [5. Examples of Full Pipelines](#5-examples-of-full-pipelines) | End-to-end examples       | Real-world DSL pipelines.                      |
| [6. Best Practices](#6-best-practices)                         | Writing robust scripts    | Guidelines and tips.                           |
| [7. Quick Reference](#7-quick-reference)                       | Command summary           | Compact lookup table of all commands.          |


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

### 2.3 CHART
**Purpose:** Visualizes items on a chart.

**Syntax:**
```dsl
CHART(items=[list_of_items], from=date, to=date, timeframe=duration, rebase=yes|no)
```

**Arguments:**
- `items` – List of symbols or arithmetic expressions.
- `from` – Start date (`YYYY-MM-DD`), default: first available.
- `to` – End date, default: today.
- `timeframe` – Chart resolution: `1d`, `1w`, `1m`, etc.
- `rebase` – Rebase values to a common starting point.

**Examples:**
```dsl
CHART(items=[AAPL, MSFT], from=2020-01-01, to=today, timeframe=1d)
CHART(items=[(AAPL + MSFT) / 2], from=2015-01-01, timeframe=1w)
CHART(items=[AAPL / MSFT, RSI(14, SPY)])
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

Backtesting can be combined with charting:
```dsl
BACKTEST(items=[AAPL], entry=[RSI(14) < 30], exit=[RSI(14) > 70], size=1000, fee=0.05) & CHART
```

---

### 2.5 GROUP
**Purpose:** Divides instruments into named groups based on logical conditions.

**Syntax:**
```dsl
GROUP(conditions=[logical_expression], name=string)
```

**Arguments:**
- `conditions` (**required**) – Logical expression used to determine group membership.  
  Instruments that satisfy the condition are included in this group.
- `name` (**required**) – Name assigned to the resulting group.  
  This name can be referenced by later commands (e.g., `CHART(items=["bull trend"])`).

**Examples:**
```dsl
GROUP(conditions=[MA(36) > MA(58)], name="bull trend")
GROUP(conditions=[MA(36) < MA(58)], name="bear trend")
```

**Usage in Pipelines:**
`GROUP` can be chained with `FILTER`, `SORT`, or visualization commands to organize and analyze subsets of instruments.

```dsl
FILTER(items=[stocks], conditions=[country=si AND market_cap <= 1000000000])
    & SORT(property=BBWP(), dir=asc)
    & GROUP(conditions=[MA(36) > MA(58)], name="bull trend")
    & GROUP(conditions=[MA(36) < MA(58)], name="bear trend")
    & CHART(items=["bull trend"], from=2024-01-01, to=today)
```

This example filters Slovenian stocks under €1B market cap, sorts them by volatility, groups them into *bull* and *bear* trends, and charts only the *bull trend* stocks.


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
| market_cap     | Select instruments    |
| quantity       | Order results         |
| indexes       | Visualize data        |
| asc   | Run strategy tests    |
| desc   | Run strategy tests    |

---

### 3.5 Functions
| Keyword       | Function Name                          |
| ------------- | -------------------------------------- |
| **RSI**       | Relative Strength Index                |
| **RSI_MA**    | Moving Average of RSI                  |
| **MACD**      | Moving Average Convergence Divergence  |
| **STOCH_D**   | Stochastic Oscillator %D               |
| **STOCH_K**   | Stochastic Oscillator %K               |
| **MA**        | Moving Average                         |
| **EMA**       | Exponential Moving Average             |
| **BB_UP**  | Bollinger Band Upper                   |
| **BB_LOW**  | Bollinger Band Lower                   |
| **BB_MID** | Bollinger Band Middle                  |
| **ADX**       | Average Directional Index              |
| **TSEN** | Ichimoku Tenkan-sen Line               |
| **KSEN**  | Ichimoku Kijun-sen Line                |
| **BBWP**      | Bollinger Band Width Percentile        |
| **BBWP_MA**   | Moving Average of Bollinger Band Width |

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

### Example 2: Filter → Sort → CHART
```dsl
FILTER(items=[stocks], conditions=[RSI(14) < 30]) & 
SORT(item=market_cap, dir=desc, limit=5) & 
CHART(from=2018-01-01, to=today, timeframe=1d)
```

### Example 3: Backtest with Conditions
```dsl
BACKTEST(items=[KRKA], 
         entry=[RSI(14) < 30 AND MA(36, SBITOP) > MA(58, SBITOP)], 
         exit=[(MA(36, SBITOP) < MA(58, SBITOP)) OR RSI(14) > 70], 
         size=1000, fee=0.05) & 
CHART
```

### Example 4: CHART Ratios
```dsl
CHART(items=[AAPL / MSFT, RSI(14, SPY)], from=2010-01-01, to=today, timeframe=1d)
```

---

## 6. Best Practices

- Always ensure parentheses match correctly in logical and arithmetic expressions.
- Start simple and test each command before chaining.
- Use clear and explicit arguments to avoid ambiguity.
- Follow the pipeline rules: `FILTER → SORT → CHART/BACKTEST`.

---

## 7. Quick Reference

| Command    | Purpose               |
|------------|----------------------|
| FILTER     | Select instruments    |
| SORT       | Order results         |
| CHART       | Visualize data        |
| BACKTEST   | Run strategy tests    |
| GROUP       | Group items         |
---
