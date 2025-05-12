'use client';

import { useEffect, useRef } from 'react';
import {
  createChart,
  type IChartApi,
  type Time,
  type CandlestickData,
  CandlestickSeries,
  CrosshairMode,
  HistogramSeries,
} from 'lightweight-charts';
import useWindowDimensions from '@/hooks/useWindowDimensions';

export interface Data {
    low: number;
    high: number;
    open: number;
    last: number;
    volume: number;
    date: string;
}



export default function CandlestickChart(props: { data: Data[], index?: boolean}) {
    const { data, index=false } = props;

    const formattedData: CandlestickData[] = data.map((price) => ({
        time: price.date.split("T")[0] as Time,
        open: Number(price.open) || Number(price.low),
        high: Number(price.high) || 0,
        low: Number(price.low) || 0,
        close: Number(price.last) || 0,
    }));

    const volumeData = data.map((price) => ({
        time: price.date.split("T")[0] as Time,
        value: Number(price.volume) || 0,
        color: Number(price.open) > Number(price.last) ? 'red' : 'green',
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
            wickUpColor: 'green',
            upColor: 'green',
            wickDownColor: 'red',
            downColor: 'red',
            borderVisible: false,
        });

        const volumeSeries = chart.addSeries(HistogramSeries, {
            priceFormat: {
                type: 'volume',
            },
            priceScaleId: '', // set as an overlay by setting a blank priceScaleId
            // set the positioning of the volume series
        });
        volumeSeries.priceScale().applyOptions({
            scaleMargins: {
                top: 0.7, // highest point of the series will be 70% away from the top
                bottom: 0,
            },
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
                    width: 2, 
                    color: 'grey',
                    //style: LineStyle.Solid,
                    labelBackgroundColor: 'grey',
                    },

                // Horizontal crosshair line (showing Price in Label)
                horzLine: {
                    width:2,
                    color: 'grey',
                    labelBackgroundColor: 'grey',
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
        if (!index) {
            volumeSeries.setData(volumeData);
        }
        // Cleanup on unmount
        return () => chart.remove();
    }, []);

    return <div ref={chartContainerRef} style={{ width: '100%', height: '300px' }} />;
}
