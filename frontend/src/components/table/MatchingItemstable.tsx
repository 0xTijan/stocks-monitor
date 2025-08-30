import { Item, Response } from "@/types/types";
import GenericTable, { Column } from "./Generictable";

type DisplayItem = {
  type: "Stock" | "Index" | "Derived";
  id: string;
  symbol?: string;
  name?: string;
  change?: number;
  [key: string]: string | number | undefined;
};

interface Props {
  response: Response;
}

export default function MatchingItemsTable({ response }: Props) {
  const items = response.matching_items ?? [];
  const displayItems: DisplayItem[] = normalizeItems(items);

  const extraKeys = Array.from(
    new Set(
      displayItems.flatMap((item) =>
        Object.keys(item).filter(
          (k) => !["type", "id", "symbol", "name", "change"].includes(k)
        )
      )
    )
  );

  // Define base columns
  const baseColumns: Column<DisplayItem>[] = [
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
          item.change > 0
            ? "text-green-400"
            : item.change < 0
            ? "text-red-400"
            : "text-gray-400";
        const arrow = item.change > 0 ? "▲" : item.change < 0 ? "▼" : "–";
        return (
          <span className={color}>
            {arrow} {Math.abs(item.change).toFixed(2)}%
          </span>
        );
      },
    },
  ];

  // Add extra_data columns dynamically
  const extraColumns: Column<DisplayItem>[] = extraKeys.map((key) => ({
    key: key,
    label: key.replace("_", " ").toUpperCase(),
    sortable: true,
    render: (item) => (item[key] !== undefined ? item[key]?.toString() : "–"),
  }));

  const columns: Column<DisplayItem>[] = [...baseColumns, ...extraColumns];

  function normalizeItems(items: Item[]): DisplayItem[] {
    return items.map((item) => {
      let base: DisplayItem;

      if ("Stock" in item.item) {
        const s = item.item.Stock;
        base = {
          type: "Stock",
          id: s.isin,
          symbol: s.symbol,
          name: s.name,
          change: s.change_prev_close_percentage,
        };
      } else if ("Index" in item.item) {
        const i = item.item.Index;
        base = {
          type: "Index",
          id: i.isin,
          symbol: i.symbol,
          name: i.name,
          change: i.change_prev_close_percentage,
        };
      } else if ("Derived" in item.item) {
        const d = item.item.Derived;
        base = {
          type: "Derived",
          id: d.id,
          symbol: d.id.split("/")[1] ?? d.id,
          name: "Derived",
          change: undefined,
        };
      } else {
        throw new Error("Unknown item type");
      }

      // Flatten extra_data Map into top-level keys
      if (item.extra_data instanceof Map) {
        for (const [key, value] of item.extra_data) {
          if ("Number" in value) (base as any)[key] = value.Number.toFixed(2);
          else if ("Text" in value) (base as any)[key] = value.Text.toString();
        }
      }

      return base;
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
