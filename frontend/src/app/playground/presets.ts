// presets.ts
export type Preset = {
  id: string;
  title: string;
  description: string;
  code: string;
};

export const presets: Preset[] = [
  {
    id: "growth-stocks",
    title: "High Growth Stocks",
    description: "Find stocks with revenue growth > 20% YoY",
    code: `FILTER(items=[stocks], conditions=[MA(36) > MA(58) AND market_cap <= 50000000])
    & SORT(item=RSI(14), dir=asc, limit=10)
    & PLOT(from=2019-01-01, to=today)
`,
  },
  {
    id: "undervalued-pe",
    title: "Low P/E Ratio",
    description: "Select stocks with P/E ratio under 15",
    code: `filter stocks where pe_ratio < 15`,
  },
  {
    id: "high-dividend",
    title: "High Dividend Yield",
    description: "Stocks with dividend yield over 5%",
    code: `filter stocks where dividend_yield > 0.05`,
  },
];
