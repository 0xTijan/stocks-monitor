import { Data } from '@/components/chart/Chart';
import ClientOnlyCandlestickChart from '@/components/chart/ClientChart';
import { Index, IndexValue } from '@/types/types';
import Link from 'next/link';


export default async function IndexDetailedPage({ params }: { params: Promise<{ isin: string }> }) {
    const { isin } = await params;

    const [stockRes, priceRes, watchlistRes] = await Promise.all([
        fetch(`https://monitor-api.tijan.dev/api/indexes/${isin}`, { cache: 'no-store' }),
        fetch(`https://monitor-api.tijan.dev/api/indexes/${isin}/prices`, { cache: 'no-store' }),
        fetch(`https://monitor-api.tijan.dev/api/indexes/`, { cache: 'no-store' }),
    ]);

    const stock: Index = await stockRes.json();
    const prices: IndexValue[] = (await priceRes.json()).reverse();
    const watchlist: Index[] = (await watchlistRes.json()).filter((item: Index) => item.isin[0] == isin[0]);

    const chartPrices: Data[] = prices.map((price) => ({
        date: price.date,
        volume: 0,
        open: price.open_value || price.low_value || 0,
        high: price.high_value || 0,
        low: price.low_value || 0,
        last: price.last_value || 0,
    }));

    return (
        <div className="flex h-screen bg-black text-white overflow-hidden">
            {/* Watchlist */}
            <div className="w-2/12 bg-black p-4 overflow-y-auto">
                <h2 className="text-xl font-semibold mb-2"><Link href="/indexes">&lt;&lt; INDEXES</Link></h2>
                <div className='mb-6'><Link className='mb-6 font-lg font-semibold' href={`/indexes/${isin[0] == "S" ? "HRZB00ICBEX6" : "SI0026109882"}`}>Switch to: {isin[0] == "S" ? "Croatian" : "Slovenian"}</Link></div>
                {watchlist.map((item, idx) => (
                    <Link  key={idx} href={`/indexes/${item.isin}`}>
                        <div className="flex items-center justify-between space-x-2 hover:bg-neutral-800 p-2 rounded-xl cursor-pointer">
                            <div>
                                <div className="font-medium">{item.symbol}</div>
                                <div className="text-xs text-gray-400">{item.name}</div>
                            </div>
                        </div>
                    </Link>
                ))}
            </div>

            {/* Chart */}
            <div className="flex flex-col w-8/12 bg-black p-4">
                <div className="flex justify-between items-center mb-4">
                    <div className="flex items-center">
                        <div>
                            <h2 className="text-2xl font-semibold">{stock.symbol}</h2>
                            <p className="text-sm text-gray-400">{stock.name}</p>
                        </div>
                    </div>
                </div>

                <div className="flex-grow mb-4">
                    <ClientOnlyCandlestickChart data={chartPrices.reverse()} index />
                </div>

                <div className="flex justify-between text-sm text-gray-400">
                    {/**bottom */}
                </div>
            </div>

            {/* Stock Details (toggle visibility) */}
            <div id="stock-details" className="w-2/12 bg-black p-4 overflow-y-auto transition-all duration-300">
                <div className="flex flex-col items-center">
                    <h3 className="text-xl font-semibold">{stock.symbol}</h3>
                    <p className="text-sm text-gray-400">{stock.name}</p>
                </div>

                <div className="text-sm space-y-2 mt-4">
                    <p className='text-4xl font-extrabold text-center'>{prices.at(-1)?.last_value}€</p>
                    <p className='text-lg text-center mt-8'>{isin.startsWith("SI") ? "Slovenia" : "Croatia"}</p>
                </div>
            </div>
        </div>
    );
}
