'use client';

import dynamic from 'next/dynamic';

const GenericChart = dynamic(() => import('@/components/chart/GenericChart'), {
  ssr: false,
});

export default GenericChart;
