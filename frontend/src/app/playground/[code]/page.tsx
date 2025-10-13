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
  const [ogSeries, setOgSeries] = useState<GenericSeries[]>([]);
  const [hasChart, setHasChart] = useState(false); 

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
      if (response.charts !== undefined && response.charts.length > 0) {
        setHasChart(true);
      }
      console.log("got response, crating charzs")
      // create chart series
      let chartSeries: GenericSeries[] = [];

      response.charts?.map((chart) => {
        console.log("Chart id ", chart.id)
        if (chart.chart_type == "Volume") {
          let volSeries: HistogramData[] = chart.data.map((x) => {return({time: x.date.split("T")[0], value: x.value[0], color: x.value[1] == 1 ? "green" : "red"})});
          chartSeries.push({
            id: chart.id,
            type: "volume",
            data: volSeries,
            title: chart.id,
            panel: chart.panel_id
          });
        } else if (chart.chart_type == "Price") {
          let priceSeries: CandlestickData[] = chart.data.map((x) => {return({time: x.date.split("T")[0], close: x.value[0], open: x.value[1], high: x.value[2], low: x.value[3]})});
          chartSeries.push({
            id: chart.id,
            type: "candlestick",
            data: priceSeries,
            title: chart.id,
            panel: chart.panel_id
          });
        } else if (chart.chart_type == "Indicator") {
          let lineSeries: LineData[] = chart.data.map((x) => {return({time: x.date.split("T")[0], value: x.value[0]})});
          chartSeries.push({
            id: chart.id,
            type: "line",
            data: lineSeries,
            color: stringToColor(chart.id),
            title: chart.id,
            panel: chart.panel_id
          });
        } else if (chart.chart_type == "Rebase") {
          let lineSeries: LineData[] = chart.data.map((x) => {return({time: x.date.split("T")[0], value: x.value[0]})});
          chartSeries.push({
            id: chart.id,
            type: "line",
            data: lineSeries,
            color: stringToColor(chart.id),
            title: chart.id,
            panel: chart.panel_id
          });
        }
      });

      let sortedSeries = sortSeries(chartSeries);
      setSeries(sortedSeries);
      setOgSeries(sortedSeries);
    }
  }, [response]);

  const sortSeries = (items: GenericSeries[]) => {
    const NON_SYMBOL_SUFFIXES = new Set(['volume']); // add more suffixes here if needed

    const getSymbol = (id: string) => {
      const parts = id.split('_');
      if (parts.length === 1) return id; // plain symbol like "POSR"
      const last = parts[parts.length - 1].toLowerCase();
      if (NON_SYMBOL_SUFFIXES.has(last)) {
        return parts[parts.length - 2]; // e.g. "ZVTG_volume" -> "ZVTG"
      }
      return parts[parts.length - 1]; // usual case: symbol is last segment
    };

    return items.slice().sort((a, b) => {
      const symA = getSymbol(a.id);
      const symB = getSymbol(b.id);

      // group by symbol first
      if (symA !== symB) return symA.localeCompare(symB);

      // same symbol: order by rank
      const isPlainA = a.id === symA;
      const isPlainB = b.id === symB;
      if (isPlainA !== isPlainB) return isPlainA ? -1 : 1;

      const isVolA = a.id.toLowerCase().endsWith(`_volume`);
      const isVolB = b.id.toLowerCase().endsWith(`_volume`);
      if (isVolA !== isVolB) return isVolA ? -1 : 1;

      // fallback: alphabetical among remaining indicators
      return a.id.localeCompare(b.id);
    });
  };

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

  const toggleSeries = (id: string, panelId: number) => {
    const exists = series.some(item => item.id === id);
    if (exists) {
      if (id.includes("_")) {
        const updatedArray = series.filter(item => item.id !== id);
        setSeries(updatedArray);
      } else {
        const updatedArray = series.filter(item => !item.id.includes(id));
        setSeries(updatedArray);
      }
    } else {
      let items = [];
      if (id.includes("_")) {
        items = ogSeries.filter(x => x.id == id);
      } else {
        items = ogSeries.filter(x => x.id.includes(id));
      }
      if (items.length > 0) {
        let s = [...series, ...items];
        setSeries(s); 
      }
    }
  }

  const toggleIndicators = () => {
    let exists = series.some(item => item.id.includes("_"));
    if (exists) {
      const updatedArray = series.filter(item => !item.id.includes("_"));
      setSeries(updatedArray);
    } else {
      const toAdd = ogSeries.filter(x => x.id.includes("_"));
      if (toAdd.length > 0) {
        let s = [...series, ...toAdd];
        setSeries(s);
      }
    }
  }

  return (
    <>
      <main className=" bg-black text-white border-b border-neutral-800">
        <div className="flex justify-between font-bold py-3">
          <div className="flex">
            <p className="px-4">STOCKS &gt;&gt;</p>
            <Link className="px-4" href="/stocks/SI0031103805">
              slo
            </Link>
            <Link className="px-4" href="/stocks/HRADPLRA0006">
              hr
            </Link>
            <Link className="px-4" href="/stocks/AT000ADDIKO0">
              at
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
      <div className="flex flex-col min-h-screen bg-black text-white overflow-hidden">
        {/* Top section: Chart + Settings */}
        {hasChart ? (
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
                <div className="w-full h-full bg-neutral-900 rounded-xl flex flex-col items-center justify-center">
                  <GenericChart series={series} />
                </div>
              </div>
            </div>

            {/* Settings */}
            <div className="w-2/12 bg-black p-4 border-l border-neutral-800 overflow-y-auto">
              <h3 className="text-xl font-semibold mb-4">Chart Settings</h3>
              <div className="space-y-3 text-sm ">
                <label className="flex items-center space-x-2" onClick={() => toggleIndicators()}>
                  <input type="checkbox" className="accent-blue-500" defaultChecked/>
                  <span>Show all indicators</span>
                </label>
                <hr />
                {ogSeries.map((x) => {
                  return(
                    <label className={`flex items-center space-x-2 ${x.title?.includes('_') ? "pl-8 text-xs -mt-2" : "pt-2"}`} key={x.id} onClick={() => toggleSeries(x.id, 0)}>
                      <input type="checkbox" className="accent-blue-500" checked={series.includes(x)} />
                      <span>{x.title}</span>
                    </label>
                  )
                })}
              </div>
            </div>
          </div>
        ):null}

        <div className="w-full px-4 py-8 border-t border-neutral-800 max-h-4/12 overflow-y-auto">
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
    </>
  );
}
