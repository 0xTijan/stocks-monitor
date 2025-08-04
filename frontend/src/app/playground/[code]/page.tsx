"use client";

import ClientOnlyCandlestickChart from '@/components/chart/ClientChart';
import GenericChart from '@/components/chart/ClientGenericChart';
import { GenericSeries, VolumeSeriesInput } from '@/components/chart/GenericChart';
import MatchingItemsTable from '@/components/table/MatchingItemstable';
import QueryInput from "@/components/ui/QueryInput";
import { Item, Response } from '@/types/types.js';
import { CandlestickData, HistogramData, LineData } from 'lightweight-charts';
import Link from "next/link.js";
import { useParams } from "next/navigation";
import { useEffect, useState } from "react";


export default function PlaygroundQueryPage() {
  const { code } = useParams();
  const [decodedQuery, setDecodedQuery] = useState("");
  const [editableText, setEditableText] = useState("");
  const [response, setResponse] = useState<Response | null>(null);
  const [series, setSeries] = useState<GenericSeries[]>([]);

  useEffect(() => {
    if (typeof code === "string") {
      const decoded = decodeURIComponent(code);
      setDecodedQuery(decoded);
      setEditableText(decoded);
      console.log("Decoded query:", decoded);
    } else {
      console.warn("Query param not available or not a string:", code);
    }
  }, [code]);

  useEffect(() => {
    async function loadWasm() {
      const wasm = await import("../../../../public/wasm/evaluator_core.js");

      await wasm.default();

      const res1 = await wasm.evaluate_script_wasm(decodedQuery);
      console.log(res1);
      if (res1) {
        setResponse(res1);
      }
    }

    if (decodedQuery.length > 0) {
      loadWasm();
    }
  }, [decodedQuery]);

  useEffect(() => {
    if (response) {
      console.log("got response, crating charzs")
      // create chart series
      let chartSeries: GenericSeries[] = [];
      response.charts?.map((chart) => {
        console.log("Chart id ", chart.id)
        if (chart.panel_id == 0) {
          if (chart.chart_type == "Volume") {
            let volSeries: HistogramData[] = chart.data.map((x) => {return({time: x.date.split("T")[0], value: x.value[0]})});
            chartSeries.push({
              id: chart.id,
              type: "volume",
              data: volSeries
            });
          } else if (chart.chart_type == "Price") {
            let priceSeries: CandlestickData[] = chart.data.map((x) => {return({time: x.date.split("T")[0], close: x.value[0], open: x.value[1], high: x.value[2], low: x.value[3]})});
            chartSeries.push({
              id: chart.id,
              type: "candlestick",
              data: priceSeries
            });
          } else if (chart.chart_type == "Indicator") {
            let lineSeries: LineData[] = chart.data.map((x) => {return({time: x.date.split("T")[0], value: x.value[0]})});
            chartSeries.push({
              id: chart.id,
              type: "line",
              data: lineSeries,
              color: stringToColor(chart.id)
            });
          }
        }
      });
      setSeries(chartSeries);
    }
  }, [response]);

  useEffect(() => {
    console.log(series);
  }, [series]);

  function stringToColor(str: string): string {
    let hash = 0;

    // Create a hash from the string
    for (let i = 0; i < str.length; i++) {
      hash = str.charCodeAt(i) + ((hash << 5) - hash);
    }

    // Convert hash to hex color
    let color = "#";
    for (let i = 0; i < 3; i++) {
      const value = (hash >> (i * 8)) & 0xff;
      color += ("00" + value.toString(16)).slice(-2);
    }

    return color;
  }


  return (
    <>
      <div className="flex flex-col min-h-screen bg-black text-white overflow-hidden">
        {/* Top section: Chart + Settings */}
        <div className="flex flex-1 overflow-hidden">
          {/* Chart */}
          <div className="flex flex-col w-10/12 p-4">
            <div className="flex items-center justify-between mb-4">
              <div>
                <h2 className="text-2xl font-semibold">Query Results</h2>
                <p className="text-sm text-gray-400">
                  Your query response
                </p>
              </div>
            </div>

            <div className="flex-grow overflow-hidden mb-4">
              {/* Replace with your chart component */}
              <div className="w-full h-full bg-neutral-900 rounded-xl flex items-center justify-center">
                <GenericChart series={series} />
              </div>
            </div>
          </div>

          {/* Settings */}
          <div className="w-2/12 bg-black p-4 border-l border-neutral-800 overflow-y-auto">
            <h3 className="text-xl font-semibold mb-4">Chart Settings</h3>
            {/* Example toggle settings */}
            <div className="space-y-3 text-sm">
              <label className="flex items-center space-x-2">
                <input type="checkbox" className="accent-blue-500" />
                <span>Show Moving Average</span>
              </label>
              <label className="flex items-center space-x-2">
                <input type="checkbox" className="accent-blue-500" />
                <span>Show Volume</span>
              </label>
              <label className="flex items-center space-x-2">
                <input type="checkbox" className="accent-blue-500" />
                <span>Show Bollinger Bands</span>
              </label>
            </div>
          </div>
        </div>

        <div className="w-full px-4 py-8 border-t border-neutral-800 max-h-5/12 overflow-y-auto">
          {/* Replace with your table component if needed */}
          <div className="w-full rounded-xl">
            {response ? <MatchingItemsTable response={response} /> : null}
          </div>
        </div>

        {/* Bottom section: Textarea + Buttons */}
        <QueryInput
          editableText={editableText}
          decodedQuery={decodedQuery}
          setEditableText={setEditableText}
        />
      </div>
      <main className=" bg-black text-white px-6">
        <div className="flex justify-between font-bold py-3">
          <div className="flex">
            <p className="px-4">STOCKS &gt;&gt;</p>
            <Link className="px-4" href="/stocks/SI0031103805">
              slo
            </Link>
            <Link className="px-4" href="/stocks/HRADPLRA0006">
              hr
            </Link>
            <Link className="px-4" href="/stocks">
              all
            </Link>
          </div>
          <Link href="/" className="font-extrabold text-xl">
            MONITOR
          </Link>
          <div className="flex">
            <Link className="px-4" href="/indexes">
              all
            </Link>
            <Link className="px-4" href="/indexes/HRADPLRA0006">
              hr
            </Link>
            <Link className="px-4" href="/indexes/SI0026109882">
              slo
            </Link>
            <p className="px-4">&lt;&lt; INDEXES</p>
          </div>
        </div>
      </main>
    </>
  );
}
