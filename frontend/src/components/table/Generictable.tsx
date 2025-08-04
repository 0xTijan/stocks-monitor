"use client";

import { useMemo, useState } from "react";

export interface Column<T> {
  key: keyof T | string;
  label: string;
  sortable?: boolean;
  render?: (row: T) => React.ReactNode;
}

type SortDirection = "asc" | "desc";

interface Props<T> {
  data: T[];
  columns: Column<T>[];
  initialSortKey?: string;
  initialSortDirection?: SortDirection;
  searchableKeys?: (keyof T)[];
}

export default function GenericTable<T>({
  data,
  columns,
  initialSortKey,
  initialSortDirection = "asc",
  searchableKeys = [],
}: Props<T>) {
  const [search, setSearch] = useState("");
  const [sortKey, setSortKey] = useState<string | undefined>(initialSortKey);
  const [sortDirection, setSortDirection] = useState<SortDirection>(initialSortDirection);

  const filteredAndSortedData = useMemo(() => {
    let filtered = data;

    if (search) {
      filtered = data.filter((row) =>
        searchableKeys.some((key) =>
          String(row[key]).toLowerCase().includes(search.toLowerCase())
        )
      );
    }

    const sorted = [...filtered];

    if (sortKey) {
      sorted.sort((a, b) => {
        const valA = (a as any)[sortKey];
        const valB = (b as any)[sortKey];

        const aStr = typeof valA === "string" ? valA.toLowerCase() : valA;
        const bStr = typeof valB === "string" ? valB.toLowerCase() : valB;

        if (aStr < bStr) return sortDirection === "asc" ? -1 : 1;
        if (aStr > bStr) return sortDirection === "asc" ? 1 : -1;
        return 0;
      });
    }

    return sorted;
  }, [data, search, sortKey, sortDirection, searchableKeys]);

  const toggleSort = (key: string) => {
    if (sortKey === key) {
      setSortDirection(sortDirection === "asc" ? "desc" : "asc");
    } else {
      setSortKey(key);
      setSortDirection("asc");
    }
  };

  return (
    <div className="space-y-4 mx-auto px-4">
      <input
        type="text"
        placeholder="Search..."
        className="w-full p-2 rounded bg-gray-900 border border-gray-700 text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
        value={search}
        onChange={(e) => setSearch(e.target.value)}
      />

      <div className="overflow-x-auto">
        <table className="min-w-full table-auto border-collapse">
          <thead>
            <tr className="text-left text-gray-400 bg-gray-950">
              {columns.map((col) => (
                <th
                  key={String(col.key)}
                  className={`px-4 py-2 ${col.sortable ? "cursor-pointer" : ""}`}
                  onClick={() => col.sortable && toggleSort(String(col.key))}
                >
                  {col.label}
                  {col.sortable && sortKey === col.key && (sortDirection === "asc" ? " ↑" : " ↓")}
                </th>
              ))}
            </tr>
          </thead>

          <tbody>
            {filteredAndSortedData.map((row, idx) => (
              <tr key={idx} className="border-b border-gray-800 hover:bg-gray-900">
                {columns.map((col) => (
                  <td key={String(col.key)} className="px-4 py-2 text-white">
                    {col.render ? col.render(row) : String((row as any)[col.key])}
                  </td>
                ))}
              </tr>
            ))}
            {filteredAndSortedData.length === 0 && (
              <tr>
                <td colSpan={columns.length} className="text-center px-4 py-6 text-gray-500">
                  No matching items found.
                </td>
              </tr>
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
}
