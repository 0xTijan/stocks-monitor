import StockTable from "@/components/table/StocksTable";
import { Stock } from "@/types/types";
import Link from "next/link";

export default async function Page() {
    const res = await fetch("https://monitor-api.tijan.dev/api/stocks/", {
        cache: "no-store",
    });

    const _stocks = await res.json();
    const stocks = _stocks.sort((a: Stock, b: Stock) => {
        const aLogo = a.logo_url;
        const bLogo = b.logo_url;
      
        if (aLogo != null && bLogo == null) return -1;
        if (aLogo == null && bLogo != null) return 1;
        if (aLogo != null && bLogo != null) return aLogo.localeCompare(bLogo);
        return 0;
    });

    return (
        <main className="min-h-screen bg-black text-white p-6">
            <h1 className="text-3xl font-semibold">Stock Directory</h1>

            <p className="text-xl font-semibold mt-2">
                <Link href="/indexes/">&lt;&lt; Indexes Directory</Link>
            </p>
            <p className="text-xl font-semibold mb-8">
                <Link href="/playground/">&lt;&lt; Playground</Link>
            </p>

            <StockTable initialStocks={stocks} />

            <p className="text-xl font-semibold mt-6 text-center">
                <Link href="/indexse/">&lt;&lt; Indexes Directory</Link>
            </p>
            <p className="text-xl font-semibold mb-8 text-center">
                <Link href="/playground/">&lt;&lt; Playground</Link>
            </p>
        </main>
    );
}
