"use client";

import { useEffect, useRef } from "react";
import {
  createChart,
  CrosshairMode,
  LineStyle,
  HistogramSeries,
  CandlestickSeries,
  LineSeries,
} from "lightweight-charts";
import useWindowDimensions from "@/hooks/useWindowDimensions";

import {
  Time,
  CandlestickData,
  HistogramData,
  LineData,
} from "lightweight-charts";

export type ChartSeriesType = "candlestick" | "volume" | "line";

export interface BaseSeries<T> {
  id: string;
  type: ChartSeriesType;
  data: T[];
  color?: string;
  title?: string;
}

export type CandlestickSeriesInput = BaseSeries<CandlestickData> & {
  type: "candlestick";
};

export type VolumeSeriesInput = BaseSeries<HistogramData> & {
  type: "volume";
};

export type LineSeriesInput = BaseSeries<LineData> & {
  type: "line";
};

export type GenericSeries =
  | CandlestickSeriesInput
  | VolumeSeriesInput
  | LineSeriesInput;

interface GenericChartProps {
  series: GenericSeries[];
  heightRatio?: number; // e.g., 0.75
}

export default function GenericChart({
  series,
  heightRatio = 0.75,
}: GenericChartProps) {
  const chartContainerRef = useRef<HTMLDivElement>(null);
  const { height } = useWindowDimensions();

  useEffect(() => {
    if (!chartContainerRef.current || series.length === 0) return;

    const chart = createChart(chartContainerRef.current, {
      layout: {
        background: { color: "black" },
        textColor: "#C3BCDB",
      },
      grid: {
        vertLines: { color: "#444" },
        horzLines: { color: "#444" },
      },
      width: chartContainerRef.current.clientWidth,
      height: height * heightRatio,
    });

    const locale = window.navigator.languages[0];
    const priceFormatter = Intl.NumberFormat(locale, {
      style: "currency",
      currency: "EUR",
    }).format;

    chart.applyOptions({
      localization: {
        priceFormatter,
      },
      crosshair: {
        mode: CrosshairMode.Normal,
        vertLine: {
          width: 2,
          color: "grey",
          labelBackgroundColor: "grey",
        },
        horzLine: {
          width: 2,
          color: "grey",
          labelBackgroundColor: "grey",
        },
      },
    });

    chart.timeScale().applyOptions({
      borderColor: "#71649C",
      barSpacing: 10,
    });

    const priceScaleMarginBottom = series.some((s) => s.type === "volume")
      ? 0.2
      : 0;

    // Create each series
    for (const s of series) {
      switch (s.type) {
        case "candlestick": {
          const candlestickSeries = chart.addSeries(CandlestickSeries, {
            wickUpColor: "green",
            upColor: "green",
            wickDownColor: "red",
            downColor: "red",
            borderVisible: false,
            title: s.title
          });
          candlestickSeries.setData(s.data);
          candlestickSeries.priceScale().applyOptions({
            autoScale: true,
          });
          break;
        }

        case "volume": {
          const volumeSeries = chart.addSeries(HistogramSeries, {
            priceFormat: {
              type: "volume",
            },
            priceScaleId: "",
          });
          volumeSeries.setData(s.data);
          volumeSeries.priceScale().applyOptions({
            scaleMargins: {
              top: 0.8,
              bottom: priceScaleMarginBottom,
            },
          });
          break;
        }

        case "line": {
          const lineSeries = chart.addSeries(LineSeries, {
            color: s.color || "#5D5FEF",
            lineWidth: 2,
            lineStyle: LineStyle.Solid,
            title: s.title
          });
          lineSeries.setData(s.data);
          break;
        }
      }
    }

    chart.timeScale().scrollToRealTime();

    return () => chart.remove();
  }, [series, height, heightRatio]);

  return (
    <div ref={chartContainerRef} style={{ width: "100%", height: "100%" }} />
  );
}
