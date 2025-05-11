import Link from "next/link";

export default function Home() {
    return (
        <main className="min-h-screen bg-black text-white p-6">
            <h1 className="text-3xl font-semibold mb-6">Directories:</h1>
            <p className="text-xl font-semibold mt-6 text-center">
                <Link href="/stocks/">Stocks Directory</Link>
            </p>
            <p className="text-xl font-semibold mt-6 text-center">
                <Link href="/indexes/">Indexes Directory</Link>
            </p>
        </main>
    );
}
