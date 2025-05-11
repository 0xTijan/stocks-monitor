'use client';

import dynamic from 'next/dynamic';

const CandlestickChart = dynamic(() => import('@/components/chart/Chart'), {
  ssr: false,
});

export default CandlestickChart;
