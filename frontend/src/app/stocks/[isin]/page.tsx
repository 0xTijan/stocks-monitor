import ClientOnlyCandlestickChart from '@/components/chart/ClientChart';
import { DailyPrice, Stock } from '@/types/types';
import Link from 'next/link';

export default async function StockDetailPage({ params }: { params: { isin: string } }) {
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

    return (
        <div className="flex h-screen bg-black text-white overflow-hidden">
            {/* Watchlist */}
            <div className="w-2/12 bg-black p-4 overflow-y-auto">
                <h2 className="text-xl font-semibold mb-2"><Link href="/stocks">&lt;&lt; STOCKS</Link></h2>
                <div className='mb-6'><Link className='mb-6 font-lg font-semibold' href={`/stocks/${isin[0] == "S" ? "HRADPLRA0006" : "SI0031103805"}`}>Switch to: {isin[0] == "S" ? "Croatian" : "Slovenian"}</Link></div>
                {watchlist.map((item, idx) => (
                    <Link  key={idx} href={`/stocks/${item.isin}`}>
                        <div className="flex items-center justify-between space-x-2 hover:bg-neutral-800 p-2 rounded-xl cursor-pointer">
                            <div>
                                <div className="font-medium">{item.symbol}</div>
                                <div className="text-xs text-gray-400">{item.name}</div>
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
                        <img src={stock.logo_url} alt={stock.name} className="h-10 mr-6" />
                        <div>
                            <h2 className="text-2xl font-semibold">{stock.name}</h2>
                            <p className="text-sm text-gray-400">{stock.symbol}</p>
                        </div>
                    </div>
                </div>

                <div className="flex-grow mb-4">
                    <ClientOnlyCandlestickChart data={prices.reverse()} />
                </div>

                <div className="flex justify-between text-sm text-gray-400">
                    {/**bottom */}
                </div>
            </div>

            {/* Stock Details (toggle visibility) */}
            <div id="stock-details" className="w-2/12 bg-black p-4 overflow-y-auto transition-all duration-300">
                <div className="flex flex-col items-center">
                    <img src={stock.logo_url} alt={stock.name} className="h-20 mb-4 mt-6" />
                    <h3 className="text-xl font-semibold">{stock.name}</h3>
                    <p className="text-sm text-gray-400">{stock.symbol}</p>
                </div>

                <div className="text-sm space-y-2 mt-4">
                    <p className='text-4xl font-extrabold text-center'>{prices.at(-1)?.last_price}€</p>
                    <p className='text-xl text-center mt-8'>{isin.startsWith("SI") ? "Slovenia" : "Croatia"}</p>
                    <p>{stock.sector_name}</p>
                    <p><strong>Since:</strong> {stock.first_trading_date || 'N/A'}</p>
                    <p>{stock.description || 'N/A'}</p>
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
