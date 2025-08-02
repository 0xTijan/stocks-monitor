'use client'

import { useRouter, useParams } from 'next/navigation'
import { useEffect, useState } from 'react'

export default function PlaygroundQueryPage() {
  const { code } = useParams()
  const router = useRouter()
  const [decodedQuery, setDecodedQuery] = useState('')
  const [isEditing, setIsEditing] = useState(false)
  const [editableText, setEditableText] = useState('')
  
  useEffect(() => {
    if (typeof code === 'string') {
      const decoded = decodeURIComponent(code)
      setDecodedQuery(decoded)
      setEditableText(decoded)
      console.log('Decoded query:', decoded)
    } else {
      console.warn('Query param not available or not a string:', code)
    }
  }, [code])

  const handleExecute = () => {
    if (!editableText.trim()) return
    const encoded = encodeURIComponent(editableText)
    router.push(`/playground/${encoded}`)
  }

  useEffect(() => {
    async function loadWasm() {
      const wasm = await import("../../../../public/wasm/evaluator_core.js");

      await wasm.default();

      const res1 = await wasm.evaluate_script(decodedQuery);
      console.log(res1);
    }

    if (decodedQuery.length > 0) {
      loadWasm();
    }
  }, [decodedQuery]);

  return (
    <main className="min-h-screen bg-black text-white p-6 flex flex-col items-center">
      <h1 className="text-3xl font-semibold mb-6 text-center">Query Preview</h1>

      <div className="w-full max-w-4xl">
        {isEditing ? (
          <textarea
            value={editableText}
            onChange={(e) => setEditableText(e.target.value)}
            className="w-full h-60 p-4 bg-zinc-900 text-white rounded-xl border border-zinc-700 focus:outline-none focus:ring-2 focus:ring-blue-500 resize-none shadow-md font-mono text-sm"
          />
        ) : (
          <pre className="w-full h-60 overflow-auto p-4 bg-zinc-900 text-white rounded-xl border border-zinc-700 shadow-md whitespace-pre-wrap font-mono text-sm">
            {decodedQuery}
          </pre>
        )}

        <div className="flex gap-4 mt-6">
          <button
            onClick={() => router.push('/playground')}
            className="px-5 py-2 bg-gray-700 hover:bg-gray-600 text-white rounded-lg transition"
          >
            ← Back to Editor
          </button>

          {isEditing ? (
            <button
              onClick={handleExecute}
              className="px-5 py-2 bg-blue-600 hover:bg-blue-500 text-white rounded-lg transition"
            >
              Execute
            </button>
          ) : (
            <button
              onClick={() => setIsEditing(true)}
              className="px-5 py-2 bg-green-600 hover:bg-green-500 text-white rounded-lg transition"
            >
              Edit
            </button>
          )}
        </div>
      </div>
    </main>
  )
}
