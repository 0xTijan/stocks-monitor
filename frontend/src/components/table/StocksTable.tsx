"use client";

import Link from "next/link";
import { useState, useMemo } from "react";

interface Stock {
  isin: string;
  symbol: string;
  name: string;
  country: string;
  logo_url?: string;
  last_price: number;
  change_prev_close_percentage: number; // e.g., +1.23 or -0.45
  quantity: number; // in EUR
  market_cap: number; // in EUR
}

interface Props {
  initialStocks: Stock[];
}

type SortKey = "name" | "symbol" | "country" | "last_price" | "change_prev_close_percentage" | "market_cap";
type SortDirection = "asc" | "desc";

export default function StockTable({ initialStocks }: Props) {
  const [search, setSearch] = useState("");
  const [sortKey, setSortKey] = useState<SortKey>("country");
  const [sortDirection, setSortDirection] = useState<SortDirection>("desc");

  const getCountryFlag = (isin: string) => {
    if (isin.startsWith("SI")) return "🇸🇮";
    if (isin.startsWith("HR")) return "🇭🇷";
    return "";
  };

  const getCountryName = (isin: string) => {
    if (isin.startsWith("SI")) return "Slovenia";
    if (isin.startsWith("HR")) return "Croatia";
    return "Other";
  };

  const filteredAndSortedStocks = useMemo(() => {
    const filtered = initialStocks.filter((stock) =>
      [stock.name, stock.symbol, stock.isin]
        .join(" ")
        .toLowerCase()
        .includes(search.toLowerCase())
    );

    const sorted = filtered.sort((a, b) => {
      let valA = a[sortKey];
      let valB = b[sortKey];
    
      if (sortKey === "country") {
        valA = getCountryName(a.isin);
        valB = getCountryName(b.isin);
      } else if (sortKey === "market_cap") {
        valA = a.last_price * a.quantity;
        valB = b.last_price * b.quantity;
      }
    
      if (typeof valA === "string") {
        valA = valA.toLowerCase();
        valB = valB.toString().toLowerCase();
      }
    
      if (valA < valB) return sortDirection === "asc" ? -1 : 1;
      if (valA > valB) return sortDirection === "asc" ? 1 : -1;
      return 0;
    });
    

    return sorted;
  }, [initialStocks, search, sortKey, sortDirection]);

  const toggleSort = (key: SortKey) => {
    if (sortKey === key) {
      setSortDirection(sortDirection === "asc" ? "desc" : "asc");
    } else {
      setSortKey(key);
      setSortDirection("asc");
    }
  };

  const formatCurrency = (value: number) =>
    `€${value.toLocaleString("en-US", { minimumFractionDigits: 2 })}`;

  return (
    <div className="space-y-4 max-w-7xl mx-auto px-4">
      <input
        type="text"
        placeholder="Search stocks..."
        className="w-full p-2 rounded bg-gray-900 border border-gray-700 text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
        value={search}
        onChange={(e) => setSearch(e.target.value)}
      />

      <div className="overflow-x-auto">
        <table className="min-w-full table-auto border-collapse">
          <thead>
            <tr className="text-left text-gray-400 bg-gray-950">
              <th className="px-4 py-2">Logo</th>
              <th className="cursor-pointer px-4 py-2" onClick={() => toggleSort("name")}>
                Name {sortKey === "name" && (sortDirection === "asc" ? "↑" : "↓")}
              </th>
              <th className="cursor-pointer px-4 py-2" onClick={() => toggleSort("symbol")}>
                Symbol {sortKey === "symbol" && (sortDirection === "asc" ? "↑" : "↓")}
              </th>
              <th className="cursor-pointer px-4 py-2" onClick={() => toggleSort("last_price")}>
                Price {sortKey === "last_price" && (sortDirection === "asc" ? "↑" : "↓")}
              </th>
              <th className="cursor-pointer px-4 py-2" onClick={() => toggleSort("change_prev_close_percentage")}>
                Change {sortKey === "change_prev_close_percentage" && (sortDirection === "asc" ? "↑" : "↓")}
              </th>
              <th className="cursor-pointer px-4 py-2" onClick={() => toggleSort("market_cap")}>
                Market Cap {sortKey === "market_cap" && (sortDirection === "asc" ? "↑" : "↓")}
              </th>
              <th className="cursor-pointer px-4 py-2" onClick={() => toggleSort("country")}>
                Country {sortKey === "country" && (sortDirection === "asc" ? "↑" : "↓")}
              </th>
              <th className="px-4 py-2">ISIN</th>
            </tr>
          </thead>

          <tbody>
            {filteredAndSortedStocks.map((stock) => (
              <tr key={stock.isin} className="border-b border-gray-800 hover:bg-gray-900">
                <td className="px-4 py-2">
                  {stock.logo_url && (
                    <img src={stock.logo_url} alt={`${stock.name} logo`} className="h-6" />
                  )}
                </td>
                <td className="px-4 py-2">
                  <Link href={`/stocks/${stock.isin}`} className="text-white hover:underline">
                    {stock.name}
                  </Link>
                </td>
                <td className="px-4 py-2">{stock.symbol}</td>
                <td className="px-4 py-2">{formatCurrency(stock.last_price)}</td>
                <td
                  className={`px-4 py-2 ${
                    stock.change_prev_close_percentage > 0
                      ? "text-green-400"
                      : stock.change_prev_close_percentage < 0
                      ? "text-red-400"
                      : "text-gray-400"
                  }`}
                >
                  {stock.change_prev_close_percentage > 0 ? "▲" : stock.change_prev_close_percentage < 0 ? "▼" : "–"}{" "}
                  {Math.abs(stock.change_prev_close_percentage)}
                </td>
                <td className="px-4 py-2 text-white">
                  {formatCurrency(stock.last_price * stock.quantity)}
                </td>
                <td className="px-4 py-2">{getCountryFlag(stock.isin)}</td>
                <td className="px-4 py-2 text-gray-400">{stock.isin}</td>
              </tr>
            ))}
            {filteredAndSortedStocks.length === 0 && (
              <tr>
                <td colSpan={8} className="text-center px-4 py-6 text-gray-500">
                  No matching stocks found.
                </td>
              </tr>
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
}
