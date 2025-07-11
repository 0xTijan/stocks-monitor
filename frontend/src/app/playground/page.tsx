'use client'

import Link from "next/link";
import { useState } from "react";
import { useRouter } from "next/navigation";
import { presets } from "@/app/playground/presets";

export default function PlaygroundHome() {
  const [query, setQuery] = useState('');
  const router = useRouter();
  const [expanded, setExpanded] = useState<string | null>(null);

  const handleExecute = (code: string) => {
    if (!code.trim()) return;
    const encoded = encodeURIComponent(code);
    router.push(`/playground/${encoded}`);
  };

  return (
    <main className="min-h-screen bg-black text-white p-6 flex flex-col items-center">
      <h1 className="text-3xl font-semibold mb-6">Stocks Screener</h1>
      {/* Directory Links - Row Style */}
      <div className="flex gap-8 text-center mb-10">
        <Link
          href="/stocks/"
          className="text-xl font-semibold text-blue-400 hover:text-blue-300 transition"
        >
          📈 Stocks Directory
        </Link>
        <Link
          href="/indexes/"
          className="text-xl font-semibold text-blue-400 hover:text-blue-300 transition"
        >
          📊 Indexes Directory
        </Link>
      </div>

      {/* Query Input Textarea */}
      <div className="w-full max-w-7xl mb-6">
        <label htmlFor="query" className="block text-lg font-medium mb-2">
          Write your script:
        </label>
        <textarea
          id="query"
          placeholder="Enter query code here..."
          value={query}
          wrap="off"
          onChange={(e) => setQuery(e.target.value)}
          className="w-full h-50 p-4 bg-black text-white rounded-xl border border-zinc-700 focus:outline-none focus:ring-2 focus:ring-white resize-none shadow-md font-mono text-md leading-snug"
        />
        <button
          onClick={() => handleExecute(query)}
          className="mt-2 px-6 py-2 bg-blue-600 hover:bg-blue-500 text-white rounded-lg transition font-medium shadow cursor-pointer"
        >
          Execute
        </button>
      </div>

      {/* Preset Cards */}
      <div className="w-full max-w-7xl grid md:grid-cols-2 gap-6 mt-6">
        {presets.map((preset) => {
          const isOpen = expanded === preset.id;
          return (
            <div
              key={preset.id}
              className="bg-zinc-900 p-5 rounded-xl border border-zinc-700 shadow hover:shadow-lg transition"
            >
              <div className="flex justify-between items-center mb-2">
                <div>
                  <h3 className="text-xl font-semibold">{preset.title}</h3>
                  <p className="text-sm text-zinc-400 whitespace-nowrap overflow-x-auto">{preset.description}</p>
                </div>
                <button
                  onClick={() => handleExecute(preset.code)}
                  title="Execute"
                  className="text-blue-400 hover:text-blue-300 text-2xl"
                >
                  ➤
                </button>
              </div>

              <pre className="bg-zinc-800 p-3 rounded text-sm font-mono text-white whitespace-pre-wrap max-h-48 overflow-auto">
                {isOpen ? preset.code : `${preset.code.slice(0, 80)}${preset.code.length > 80 ? '...' : ''}`}
              </pre>

              <div className="mt-2 flex gap-4">
                {preset.code.length > 80 && (
                  <button
                    onClick={() => setExpanded(isOpen ? null : preset.id)}
                    className="text-blue-400 hover:underline text-sm cursor-pointer"
                  >
                    {isOpen ? 'Show less' : 'Show more'}
                  </button>
                )}

                <button
                  onClick={() => setQuery(preset.code)}
                  className="text-blue-400 hover:underline text-sm cursor-pointer"
                >
                  ✎ Edit
                </button>
              </div>
            </div>
          );
        })}
      </div>
    </main>
  );
}
