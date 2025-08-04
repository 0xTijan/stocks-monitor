import { Item, Response } from "@/types/types";
import GenericTable, { Column } from "./Generictable";


type DisplayItem = {
  type: "Stock" | "Index" | "Derived";
  id: string;
  symbol?: string;
  name?: string;
  change?: number;
};

interface Props {
  response: Response;
}

export default function MatchingItemsTable({ response }: Props) {
  const items = response.matching_items ?? [];
  const displayItems: DisplayItem[] = normalizeItems(items);

  const columns: Column<DisplayItem>[] = [
    { key: "type", label: "Type", sortable: true },
    { key: "id", label: "ID", sortable: true },
    { key: "symbol", label: "Symbol", sortable: true },
    { key: "name", label: "Name", sortable: true },
    {
      key: "change",
      label: "Change %",
      sortable: true,
      render: (item) => {
        if (item.change === undefined) return "–";
        const color =
          item.change > 0 ? "text-green-400" : item.change < 0 ? "text-red-400" : "text-gray-400";
        const arrow = item.change > 0 ? "▲" : item.change < 0 ? "▼" : "–";
        return <span className={color}>{arrow} {Math.abs(item.change).toFixed(2)}%</span>;
      },
    },
  ];

  function normalizeItems(items: Item[]): DisplayItem[] {
    return items.map((item) => {
        if ("Stock" in item) {
        const s = item.Stock;
        return {
            type: "Stock",
            id: s.isin,
            symbol: s.symbol,
            name: s.name,
            change: s.change_prev_close_percentage,
        };
        } else if ("Index" in item) {
        const i = item.Index;
        return {
            type: "Index",
            id: i.isin,
            symbol: i.symbol,
            name: i.name,
            change: i.change_prev_close_percentage,
        };
        } else if ("Derived" in item) {
        const d = item.Derived;
        return {
            type: "Derived",
            id: d.id,
            symbol: d.id.split("/")[1] ?? d.id,
            name: "Derived",
            change: undefined,
        };
        } else {
        throw new Error("Unknown item type");
        }
    });
    }


  return (
    <GenericTable
      data={displayItems}
      columns={columns}
      initialSortKey="type"
      initialSortDirection="asc"
      searchableKeys={["id", "symbol", "name"]}
    />
  );
}
