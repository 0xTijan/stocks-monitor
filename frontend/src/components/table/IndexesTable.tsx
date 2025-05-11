"use client";

import Link from "next/link";
import { useState, useMemo } from "react";

interface Index {
  isin: string;
  symbol: string;
  country: string; // Assuming country is part of the stock data
}

interface Props {
  initialStocks: Index[];
}

type SortKey = "isin" | "symbol" | "country";
type SortDirection = "asc" | "desc";

export default function IndexesTable({ initialStocks }: Props) {
  const [search, setSearch] = useState("");
  const [sortKey, setSortKey] = useState<SortKey>("country");
  const [sortDirection, setSortDirection] = useState<SortDirection>("asc");

  // Function to determine the country flag based on ISIN
  const getCountryFlag = (isin: string) => {
    if (isin.startsWith("SI")) {
      return "🇸🇮"; // Slovenia
    }
    if (isin.startsWith("HR")) {
      return "🇭🇷"; // Croatia
    }
    return ""; // Default case (no flag)
  };

  // Function to get country name based on ISIN for sorting
  const getCountryName = (isin: string) => {
    if (isin.startsWith("SI")) {
      return "Slovenia";
    }
    if (isin.startsWith("HR")) {
      return "Croatia";
    }
    return "Other"; // Default case
  };

  const filteredAndSortedStocks = useMemo(() => {
    const filtered = initialStocks.filter((stock) =>
      [stock.country, stock.symbol, stock.isin]
        .join(" ")
        .toLowerCase()
        .includes(search.toLowerCase())
    );

    const sorted = filtered.sort((a, b) => {
      let valA: string | number = a[sortKey];
      let valB: string | number = b[sortKey];

      if (sortKey === "country") {
        valA = getCountryName(a.isin); // Get the country name for sorting
        valB = getCountryName(b.isin);
      }

      if (typeof valA === "string") {
        valA = valA.toLowerCase();
        valB = valB.toLowerCase();
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

  return (
    <div className="space-y-4 max-w-7xl mx-auto px-4">
        <input
            type="text"
            placeholder="Search indexes..."
            className="w-full p-2 rounded bg-gray-900 border border-gray-700 text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
        />

      <div className="overflow-x-auto">
        <table className="min-w-full table-auto border-collapse">
          <thead>
            <tr className="text-left text-gray-400 bg-gray-950">
              <th
                className="cursor-pointer px-4 py-2"
                onClick={() => toggleSort("symbol")}
              >
                Symbol {sortKey === "symbol" && (sortDirection === "asc" ? "↑" : "↓")}
              </th>
              <th
                className="cursor-pointer px-4 py-2"
                onClick={() => toggleSort("isin")}
              >
                ISIN {sortKey === "isin" && (sortDirection === "asc" ? "↑" : "↓")}
              </th>
              <th
                className="cursor-pointer px-4 py-2"
                onClick={() => toggleSort("country")}
              >
                Country {sortKey === "country" && (sortDirection === "asc" ? "↑" : "↓")}
              </th>
            </tr>
          </thead>

          <tbody>
            {filteredAndSortedStocks.map((stock) => (
              <tr
                key={stock.isin}
                className="border-b border-gray-800 hover:bg-gray-900"
              >
                <td className="px-4 py-2">
                  <Link href={`/indexes/${stock.isin}`} className="text-white hover:underline">
                    {stock.symbol}
                  </Link>
                </td>
                <td className="px-4 py-2 text-gray-400">{stock.isin}</td>
                <td className="px-4 py-2">{getCountryFlag(stock.isin)}</td>
              </tr>
            ))}
            {filteredAndSortedStocks.length === 0 && (
              <tr>
                <td colSpan={5} className="text-center px-4 py-6 text-gray-500">
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
