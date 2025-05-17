import { Data } from '@/components/chart/Chart';
import ClientOnlyCandlestickChart from '@/components/chart/ClientChart';
import { DailyPrice, Stock } from '@/types/types';
import Link from 'next/link';

function formatDate(date: Date) {
    const d = date.getDate();
    const m = date.getMonth() + 1; // Months are zero-indexed
    const y = date.getFullYear();
    return `${d}. ${m}. ${y}`;
}  

export default async function StockDetailPage({ params }: { params: Promise<{ isin: string }> }) {
    const { isin } = await params;

    const [stockRes, priceRes, watchlistRes] = await Promise.all([
        fetch(`https://monitor-api.tijan.dev/api/stocks/${isin}`, { cache: 'no-store' }),
        fetch(`https://monitor-api.tijan.dev/api/stocks/${isin}/prices`, { cache: 'no-store' }),
        fetch(`https://monitor-api.tijan.dev/api/stocks/`, { cache: 'no-store' }),
    ]);

    const stock: Stock = await stockRes.json();
    const prices: DailyPrice[] = (await priceRes.json()).reverse();
    const watchlist: Stock[] = (await watchlistRes.json()).filter((item: Stock) => item.isin[0] == isin[0]).sort((a: Stock, b: Stock) => {
        const aLogo = a.logo_url;
        const bLogo = b.logo_url;
      
        if (aLogo != null && bLogo == null) return -1;
        if (aLogo == null && bLogo != null) return 1;
        if (aLogo != null && bLogo != null) return aLogo.localeCompare(bLogo);
        return 0;
    });
    const chartPrices: Data[] = prices.map((price) => ({
        date: price.date,
        volume: price.volume || 0,
        open: price.open_price || price.low_price || 0,
        high: price.high_price || 0,
        low: price.low_price || 0,
        last: price.last_price || 0,
    }));

    function calculateChange(prices: DailyPrice[], daysAgo: number): number | null {
        const latest = prices[0]?.last_price;
        const pastDate = new Date(prices[0]?.date);
        pastDate.setDate(pastDate.getDate() - daysAgo);
        
        const past = prices.find(p => {
            const date = new Date(p.date);
            return date <= pastDate && p.last_price != null;
        });
        
        if (latest && past?.last_price) {
            return ((latest - past.last_price) / past.last_price) * 100;
        }
        
        return null;
    }
    
    const change1d = calculateChange(prices, 1);
    const change7d = calculateChange(prices, 7);
    const change30d = calculateChange(prices, 30);
    const change180d = calculateChange(prices, 180);
    const change1y = calculateChange(prices, 365);
    const changeYtd = (() => {
        const latest = prices[0]?.last_price;
        if (!latest) return null;
      
        const currentYear = new Date(prices[0].date).getFullYear();
      
        // Find the first available trading day in the current year
        const firstOfYear = [...prices]
            .reverse()
            .find(p => new Date(p.date).getFullYear() === currentYear && p.last_price != null);
      
        if (firstOfYear?.last_price) {
            return ((latest - firstOfYear.last_price) / firstOfYear.last_price) * 100;
        }
      
        return null;
    })();
    
    function getAllTimeHighLow(prices: DailyPrice[]) {
        if (!prices.length) return null;
      
        let allTimeHigh = { date: "", value: -Infinity };
        let allTimeLow = { date: "", value: Infinity };
      
        // Loop through all prices to find the highest and lowest values
        for (const price of prices) {
          if (price.last_price != null) {
            if (Number(price.last_price) > allTimeHigh.value) {
              allTimeHigh = { date: price.date, value: Number(price.last_price) };
            }
            if (Number(price.last_price) < allTimeLow.value) {
              allTimeLow = { date: price.date, value: Number(price.last_price) };
            }
          }
        }
      
        return {
          high: allTimeHigh,
          low: allTimeLow
        };
    }      

    const allTimeStats = getAllTimeHighLow(prices);

    return (
        <div className="flex h-screen bg-black text-white overflow-hidden">
            {/* Watchlist */}
            <div className="w-2/12 bg-black p-4 overflow-y-auto">
                <h2 className="text-xl font-semibold"><Link href="/stocks">&lt;&lt; STOCKS</Link></h2>
                <h2 className="text-xl font-semibold mb-2"><Link href="/playground">&lt;&lt; Playground</Link></h2>
                <div className='mb-6'><Link className='mb-6 font-lg font-semibold' href={`/stocks/${isin[0] == "S" ? "HRADPLRA0006" : "SI0031103805"}`}>Switch to: {isin[0] == "S" ? "Croatian" : "Slovenian"}</Link></div>
                {watchlist.map((item, idx) => (
                    <Link  key={idx} href={`/stocks/${item.isin}`}>
                        <div className="flex items-center justify-between space-x-2 hover:bg-neutral-800 p-2 rounded-xl cursor-pointer">
                            <div>
                                <div className="font-medium">{item.symbol}</div>
                                <div className="text-xs text-gray-400">{item.name}</div>
                                <div>
                                    <span className={(item?.change_prev_close_percentage || 0) >= 0 ? "text-green-600" : "text-red-600"}>
                                        {(item?.change_prev_close_percentage || 0) >= 0 ? "▲" : "▼"} {Math.abs(item?.change_prev_close_percentage || 0)}%
                                    </span>
                                </div>
                            </div>
                            {item.logo_url ? (<img src={item.logo_url} alt={item.name} className="h-6" />): null}
                        </div>
                    </Link>
                ))}
            </div>

            {/* Chart */}
            <div className="flex flex-col w-8/12 bg-black p-4">
                <div className="flex justify-between items-center mb-4">
                    <div className="flex items-center">
                        {stock.logo_url ? <img src={stock.logo_url} alt={stock.name} className="h-10 mr-6" /> : null}
                        <div>
                            <h2 className="text-2xl font-semibold">{stock.symbol}</h2>
                            <p className="text-sm text-gray-400">{stock.name}</p>
                        </div>
                    </div>
                </div>

                <div className="flex-grow mb-4">
                    <ClientOnlyCandlestickChart data={chartPrices.reverse()} />
                </div>

                <div className="flex justify-between text-sm text-gray-400">
                    {/**bottom */}
                </div>
            </div>

            {/* Stock Details (toggle visibility) */}
            <div id="stock-details" className="w-2/12 bg-black p-4 overflow-y-auto transition-all duration-300">
                <div className="flex flex-col items-center">
                    {stock.logo_url ? <img src={stock.logo_url} alt={stock.name} className="h-20 mb-4 mt-6" /> : null}
                    <h3 className="text-xl font-semibold">{stock.symbol}</h3>
                    <p className="text-sm text-gray-400">{stock.name}</p>
                </div>

                <div className="text-sm space-y-2 mt-4">
                    <p className='text-4xl font-extrabold text-center'>{prices[0]?.last_price}€</p>
                    <p className='text-2xl font-extrabold text-center'>
                        <span className={(prices[0]?.change_prev_close_percentage || 0) >= 0 ? "text-green-600" : "text-red-600"}>
                            {(prices[0]?.change_prev_close_percentage || 0) >= 0 ? "▲" : "▼"} {Math.abs(prices[0]?.change_prev_close_percentage || 0)}%
                        </span>
                    </p>

                    <p className='text-lg text-center mt-8'>{isin.startsWith("SI") ? "Slovenia" : "Croatia"}</p>
                    <p className='text-lg mt-2 text-center mb-10'><strong>{stock.sector_name}</strong></p>
                    <div className="flex justify-between items-center text-md">
                        <div className="text-left text-gray-300">Market Cap:</div>
                        <div className="text-right text-base">{((stock.quantity||0)*(stock.last_price||0)).toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}€</div>
                    </div>
                    <div className="flex justify-between items-center text-md">
                        <div className="text-left text-gray-300">First Day:</div>
                        <div className="text-right text-base">{formatDate(new Date(stock.first_trading_date || "")) || 'N/A'}</div>
                    </div>
                    {allTimeStats && prices[0]?.last_price && (
                        <div className="mt-6 space-y-4 text-sm text-gray-300">
                            {/* All-Time High */}
                            <div className="flex justify-between items-center">
                            <div className="text-left font-medium">All-Time High</div>
                            <div className="text-right">
                                <div className="text-white font-semibold text-base">
                                €{allTimeStats.high.value.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}{' '}
                                <span className="text-sm text-red-500 font-medium">
                                    (-{((allTimeStats.high.value - prices[0].last_price) / allTimeStats.high.value * 100).toFixed(1)}%)
                                </span>
                                </div>
                                <div className="text-xs text-gray-500">
                                {new Date(allTimeStats.high.date).toLocaleDateString(undefined, {
                                    year: "numeric",
                                    month: "short",
                                    day: "numeric",
                                })}
                                </div>
                            </div>
                            </div>

                            {/* All-Time Low */}
                            <div className="flex justify-between items-center">
                            <div className="text-left font-medium">All-Time Low</div>
                            <div className="text-right">
                                <div className="text-white font-semibold text-base">
                                €{allTimeStats.low.value.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}{' '}
                                <span className="text-sm text-green-500 font-medium">
                                    (+{((prices[0].last_price - allTimeStats.low.value) / allTimeStats.low.value * 100).toFixed(1)}%)
                                </span>
                                </div>
                                <div className="text-xs text-gray-500">
                                {new Date(allTimeStats.low.date).toLocaleDateString(undefined, {
                                    year: "numeric",
                                    month: "short",
                                    day: "numeric",
                                })}
                                </div>
                            </div>
                            </div>
                        </div>
                    )}
                </div>

                <div className="mt-6">
                    <table className="w-full text-md text-center border border-gray-800">
                        <thead className="bg-gray-900 text-gray-400">
                        <tr>
                            <th>1D</th>
                            <th>7D</th>
                            <th>30D</th>
                            <th>180D</th>
                            <th>1Y</th>
                            <th>YTD</th>
                        </tr>
                        </thead>
                        <tbody>
                        <tr className="bg-black">
                            {[change1d, change7d, change30d, change180d, change1y, changeYtd].map((change, idx) => (
                            <td key={idx} className={`${change != null ? (change >= 0 ? 'text-green-500' : 'text-red-500') : 'text-gray-500'}`}>
                                {change != null ? `${change >= 0 ? '▲' : '▼'} ${Math.abs(change).toFixed(2)}%` : '—'}
                            </td>
                            ))}
                        </tr>
                        </tbody>
                    </table>
                </div>

                <div className="text-sm space-y-2 mt-8">
                    <p className='mt-4'>{stock.description || 'N/A'}</p>
                    {stock.website_url && (
                        <a href={stock.website_url} target="_blank" rel="noopener noreferrer" className="text-blue-500 hover:underline">
                            Visit Website
                        </a>
                    )}
                </div>
            </div>
        </div>
    );
}
