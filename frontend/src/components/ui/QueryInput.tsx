"use client";

import { useRouter } from "next/navigation";
import { useState } from "react";

export default function QueryInput(props: {
  decodedQuery: string;
  editableText: string;
  setEditableText: (s: string) => void;
}) {
  const { decodedQuery, editableText, setEditableText } = props;
  const router = useRouter();
  const [isEditing, setIsEditing] = useState(false);

  const handleExecute = () => {
    if (!editableText.trim()) return;
    const encoded = encodeURIComponent(editableText);
    router.push(`/playground/${encoded}`);
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.shiftKey && e.key === "Enter") {
      e.preventDefault();
      handleExecute();
    }
  };

  return (
    <div className="w-full gap-4 p-4 border-t border-b border-neutral-800">
      {isEditing ? (
        <textarea
          value={editableText}
          onChange={(e) => setEditableText(e.target.value)}
          onKeyDown={handleKeyDown}
          className="w-full p-4 bg-zinc-900 text-white rounded-xl border border-zinc-700 focus:outline-none focus:ring-2 focus:ring-blue-500 resize-none shadow-md font-mono text-sm"
        />
      ) : (
        <pre className="w-full overflow-auto p-4 bg-zinc-900 text-white rounded-xl border border-zinc-700 shadow-md whitespace-pre-wrap font-mono text-sm">
          {decodedQuery}
        </pre>
      )}
      <div className="flex flex-row gap-4 mt-4 justify-end">
        <button
          onClick={() => router.push("/playground")}
          className="px-5 py-2 bg-gray-700 hover:bg-gray-600 text-white rounded-lg transition h-fit font-bold cursor-pointer"
        >
          Playground
        </button>

        {isEditing ? (
          <>
          <button
            onClick={() => {
              setIsEditing(false)
              setEditableText(decodedQuery)
            }}
            className="px-5 py-2 bg-gray-700 hover:bg-gray-600 text-white rounded-lg transition h-fit font-bold cursor-pointer"
          >
            Cancel
          </button>
          <button
            onClick={handleExecute}
            className="px-5 py-2 bg-blue-600 hover:bg-blue-500 text-white rounded-lg transition h-fit font-bold cursor-pointer"
            >
            Execute
          </button>
          </>
        ) : (
          <button
            onClick={() => setIsEditing(true)}
            className="px-5 py-2 bg-green-600 hover:bg-green-500 text-white rounded-lg transition h-fit font-bold cursor-pointer"
          >
            Edit
          </button>
        )}
      </div>
      {isEditing ? <p className="text-sm text-zinc-400 whitespace-nowrap overflow-x-auto mt-1.5 text-right mr-1">Shift + Enter</p> : null}
    </div>
  );
}
