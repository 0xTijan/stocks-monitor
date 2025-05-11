'use client';

import { useEffect, useRef } from 'react';
import {
  createChart,
  type IChartApi,
  type Time,
  type CandlestickData,
  CandlestickSeries,
  CrosshairMode,
  AreaSeries,
  LineData,
  LineStyle,
} from 'lightweight-charts';
import { DailyPrice } from '@/types/types';
import useWindowDimensions from '@/hooks/useWindowDimensions';


export default function CandlestickChart(props: { data: DailyPrice[]}) {
    const { data } = props;

    const formattedData: CandlestickData[] = data.map((price) => ({
        time: price.date.split("T")[0] as Time,
        open: Number(price.open_price) || Number(price.low_price),
        high: Number(price.high_price) || 0,
        low: Number(price.low_price) || 0,
        close: Number(price.last_price) || 0,
    }));

    const lineData: LineData[] = data.map((price) => ({
        time: price.date.split("T")[0] as Time,
        value: Number(price.last_price) || 0,
    }));

    const chartContainerRef = useRef<HTMLDivElement>(null);

    const { height } = useWindowDimensions();

    useEffect(() => {
        if (!chartContainerRef.current) return;

        const chartOptions = {
            layout: {
                background: { color: "black" },
                textColor: "#C3BCDB",
            },
            grid: {
                vertLines: { color: "#444" },
                horzLines: { color: "#444" },
            },
            width: chartContainerRef.current.clientWidth,
            height: height * 0.75,
        };

        // Get the current users primary locale
        const currentLocale = window.navigator.languages[0];
        // Create a number format using Intl.NumberFormat
        const myPriceFormatter = Intl.NumberFormat(currentLocale, {
            style: 'currency',
            currency: 'EUR', // Currency for data points
        }).format;


        const chart: IChartApi = createChart(chartContainerRef.current, chartOptions);
        const series = chart.addSeries(CandlestickSeries, {
            wickUpColor: 'rgb(54, 116, 217)',
            upColor: 'rgb(54, 116, 217)',
            wickDownColor: 'rgb(225, 50, 85)',
            downColor: 'rgb(225, 50, 85)',
            borderVisible: false,
        });

        /*const areaSeries = chart.addSeries(AreaSeries, {
            lastValueVisible: false, // hide the last value marker for this series
            crosshairMarkerVisible: false, // hide the crosshair marker for this series
            lineColor: 'transparent', // hide the line
            topColor: 'rgba(56, 33, 110,0.6)',
            bottomColor: 'rgba(56, 33, 110, 0.1)',
        });
        // Set the data for the Area Series
        areaSeries.setData(lineData);*/

        series.setData(formattedData);
        chart.applyOptions({
            localization: {
                priceFormatter: myPriceFormatter,
            },
            crosshair: {
                // Change mode from default 'magnet' to 'normal'.
                // Allows the crosshair to move freely without snapping to datapoints
                mode: CrosshairMode.Normal,
        
                vertLine: {
                    width: 8, 
                    color: '#C3BCDB44',
                    style: LineStyle.Solid,
                    labelBackgroundColor: '#9B7DFF',
                },

                // Horizontal crosshair line (showing Price in Label)
                horzLine: {
                    color: '#9B7DFF',
                    labelBackgroundColor: '#9B7DFF',
                },
            },
        });
        chart.timeScale().applyOptions({
            borderColor: '#71649C',
            barSpacing: 10,
        });
        series.priceScale().applyOptions({
            autoScale: false,
        });
        chart.timeScale().scrollToRealTime();

        // Cleanup on unmount
        return () => chart.remove();
    }, []);

    return <div ref={chartContainerRef} style={{ width: '100%', height: '300px' }} />;
}
