'use client';

import { useState } from 'react';

export default function CollapseToggle({ onToggle }: { onToggle: (collapsed: boolean) => void }) {
    const [collapsed, setCollapsed] = useState(false);

    const toggle = () => {
        const next = !collapsed;
        setCollapsed(next);
        onToggle(next);
    };

    return (
        <button
            onClick={toggle}
            className="bg-gray-700 hover:bg-gray-600 text-white text-sm px-3 py-1 rounded"
        >
            {collapsed ? 'Show Details' : 'Hide Details'}
        </button>
    );
}
