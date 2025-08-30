// presets.ts
export type Preset = {
  id: string;
  title: string;
  description: string;
  code: string;
};

export const presets: Preset[] = [
  {
    id: "basic-chart",
    title: "Basic Chart",
    description: "Basic chart with some default indicators.",
    code: `CHART(items=[SBITOP, RSI(14), RSIMA(14, 14), MA(36), MA(58)])`,
  },
  {
    id: "most-volatile-stocks",
    title: "Most Volatile Stocks",
    description: "All stocks sorted by current volatility levels.",
    code: `FILTER(items=[stocks], conditions=[country=si]) & SORT(property=BBWP(13, 252), dir=asc)`,
  },
  {
    id: "YTD-performance-stocks",
    title: "YTD Performance Stocks",
    description: "Year-to-date performance of all tracked stocks.",
    code: `CHART(items=[stocks], from=2025-01-01, to=today, rebase=100)`,
  },
  {
    id: "YTD-performance-stocks-si",
    title: "YTD Performance Stocks SI",
    description: "Year-to-date performance of all SI stocks.",
    code: `FILTER(items=[stocks], conditions=[country=si]) & CHART(from=2025-01-01, to=today, rebase=100)`,
  },
    {
    id: "YTD-performance-stocks-hr",
    title: "YTD Performance Stocks HR",
    description: "Year-to-date performance of all HR stocks.",
    code: `FILTER(items=[stocks], conditions=[country=hr]) & CHART(from=2025-01-01, to=today, rebase=100)`,
  },
  {
    id: "YTD-performance-indexes",
    title: "YTD Performance Indexes",
    description: "Year-to-date performance of all tracked indxes.",
    code: `CHART(items=[indexes], from=2025-01-01, to=today, rebase=100)`,
  },
  {
    id: "3Y-performance",
    title: "3Y Performance",
    description: "3 years performance of all tracked stocks.",
    code: `CHART(items=[stocks], from=2022-01-01, to=today, rebase=100)`,
  },
  {
    id: "5Y-performance",
    title: "5Y Performance",
    description: "5 years performance of all tracked stocks.",
    code: `CHART(items=[stocks], from=2020-01-01, to=today, rebase=100)`,
  },
  {
    id: "slo-hr-ratio",
    title: "SI/HR Ratio",
    description: "Ratio between top SI and HR indexes.",
    code: `CHART(items=[(SBITOP / CBX)], from=2019-01-01, to=today)`,
  },
];
