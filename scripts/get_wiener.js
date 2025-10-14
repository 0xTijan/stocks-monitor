const puppeteer = require("puppeteer");
const { AT_STOCK_IDS, AT_INDEX_IDS } = require("./config");
const path = require("path");
const fs = require("fs");
const fs1 = require("fs/promises");
const https = require("https");
const { parse } = require("csv-parse");

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function formatDate(input) {
  const [month, day, year] = input.split("/");
  return `${year}-${month.padStart(2, "0")}-${day.padStart(2, "0")}`;
}

const getFormattedDate = () => {
  const now = new Date();

  const yyyy = now.getFullYear();
  const dd = String(now.getDate()).padStart(2, "0");
  const mm = String(now.getMonth() + 1).padStart(2, "0"); // Months are 0-based

  const formattedDate = `${mm}%2F${dd}%2F${yyyy}`;
  return formattedDate;
};

async function downloadFile(url, destPath) {
  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(destPath);
    https
      .get(url, (response) => {
        if (response.statusCode !== 200) {
          reject(new Error(`Failed to get file: ${response.statusCode}`));
          return;
        }

        response.pipe(file);
        file.on("finish", () => file.close(resolve));
      })
      .on("error", (err) => {
        fs.unlink(destPath, () => reject(err));
      });
  });
}

async function parseCsv(filePath) {
  const fileContent = await fs1.readFile(filePath, "utf-8");
  return new Promise((resolve, reject) => {
    parse(
      fileContent,
      {
        delimiter: ";",
        trim: true,
        skip_empty_lines: true,
        relax_column_count: true, // ← allows extra fields
        columns: (header) =>
          header
            .map((h) => h.replace(/^\uFEFF/, "").trim())
            .filter((h) => h !== ""), // remove empty headers
      },
      (err, records) => {
        if (err) reject(err);
        else resolve(records);
      }
    );
  });
}

const getMarketCaps = async (params) => {
  const browser = await puppeteer.launch();
  const tablePage = await browser.newPage();
  await tablePage.goto(
    "https://www.wienerborse.at/en/market-data/statistics/daily-statistics/turnover-and-capitalisation/",
    { waitUntil: "networkidle2", timeout: 60000 }
  );

  // wait for table to render
  if (typeof tablePage.waitForTimeout === "function") {
    await tablePage.waitForTimeout(2000);
  } else if (typeof tablePage.waitFor === "function") {
    await tablePage.waitFor(2000);
  } else {
    await sleep(2000);
  }

  // returns object: { [isin]: mc }
  const tableData = await tablePage.$$eval("tr[data-key]", (rows) => {
    const result = {};
    rows.forEach((row) => {
      // ISIN: prefer <span class="isin">...</span>, fallback to data-isin on anchor
      const isinEl = row.querySelector(".isin");
      const anchor = row.querySelector("a[data-isin]");
      const isin = (
        isinEl?.textContent?.trim() ||
        anchor?.getAttribute("data-isin") ||
        ""
      ).trim();
      if (!isin) return; // skip rows without ISIN

      // Market cap cell in same row
      const mcCell = row.querySelector("td.daily-capitalization");
      let mc = null;
      if (mcCell) {
        const txt = mcCell.textContent.trim();
        // remove commas/spaces and any non-digit (preserve dot/minus)
        const numStr = txt
          .replace(/,/g, "")
          .replace(/\s+/g, "")
          .replace(/[^\d.-]/g, "");
        mc = numStr === "" ? null : Number(numStr);
        if (Number.isNaN(mc)) mc = null;
      }

      result[isin] = mc;
    });
    return result;
  });

  await browser.close();
  return tableData; // e.g. { "AT0000606306": 10065552403, ... }
};

// SCRAPE FOR METADATA
async function main() {
  // start a puppeteer session
  const browser = await puppeteer.launch();
  const tillDate = getFormattedDate();
  const mcs = await getMarketCaps();

  for (const idGroup of AT_STOCK_IDS) {
    let type = idGroup.type;
    for (const id of idGroup.ids) {
      // open a new page
      const dataPage = await browser.newPage();
      const urlPage = await browser.newPage();

      const url = `https://www.wienerborse.at/en/${type}/${id}/`;

      // open this URL, and wait for the DOM to load
      await dataPage.goto(`${url}`, {
        waitUntil: "networkidle2",
        timeout: 60000,
      });
      await urlPage.goto(`${url}company-profile/`, {
        waitUntil: "networkidle2",
        timeout: 60000,
      });
/*
      // use page.waitForTimeout if available, otherwise fallback to sleep
      if (typeof dataPage.waitForTimeout === "function") {
        await dataPage.waitForTimeout(10000);
      } else if (typeof dataPage.waitFor === "function") {
        // older puppeteer versions had page.waitFor(ms)
        await dataPage.waitFor(10000);
      } else {
        await sleep(10000);
      }

      // use page.waitForTimeout if available, otherwise fallback to sleep
      if (typeof urlPage.waitForTimeout === "function") {
        await urlPage.waitForTimeout(10000);
      } else if (typeof urlPage.waitFor === "function") {
        // older puppeteer versions had page.waitFor(ms)
        await urlPage.waitFor(10000);
      } else {
        await sleep(10000);
      }*/

      // get stock data (first tab)
      const stockData = await dataPage.evaluate(() => {
        const heading = document.querySelector(".header_module");
        const h1 = heading?.querySelector("h1");
        const name = [...h1.childNodes] // get all child nodes (text nodes and elements)
          .filter((node) => node.nodeType === Node.TEXT_NODE) // keep only text nodes
          .map((node) => node.textContent.trim()) // trim whitespace
          .join(" "); // join if multiple text nodes
        const symbol =
          heading
            ?.querySelector(".header-summary")
            ?.querySelectorAll("span")?.[5]?.innerText || null;

        const logo_box = document.querySelector(".stock-logo");
        const logo = logo_box?.querySelector("img")?.src || null;

        let sectorName = null;
        let quantity = null;
        const container = document.querySelector("#c1171-module-container");
        const table = container?.querySelector("table")?.querySelector("tbody");
        const trs = table?.querySelectorAll("tr");
        if (trs?.length > 6) {
          sectorName = trs[1]?.querySelector("td")?.innerText || null;
          quantity =
            trs[6]?.querySelector("td")?.innerText.replace(/,/g, "") || null;
        }

        const container2 = document.querySelector("#c1159-module-container");
        const table2 = container2
          ?.querySelector("table")
          ?.querySelector("tbody");
        const trs2 = table2?.querySelectorAll("tr");

        let firstDay = null;

        trs2?.forEach((row) => {
          const label = row.querySelector("th")?.innerText?.trim();
          if (label === "First Trading Day") {
            const value = row.querySelector("td")?.innerText?.trim();
            if (value) {
              const [month, day, year] = value.split("/");
              firstDay = `${year}-${month.padStart(2, "0")}-${day.padStart(
                2,
                "0"
              )}`;
            }
          }
        });

        const description =
          document.querySelector(".company-description")?.innerText || null;

        const nace = "";
        const sectorId = "";

        return {
          name,
          symbol,
          logo,
          description,
          firstDay,
          nace,
          sectorId,
          sectorName,
        };
      });

      const urlData = await urlPage.evaluate(() => {
        const parent = document.querySelector(
          ".vertical-value.col-sm-8.col-md-9"
        );
        const link = parent?.querySelector("a")?.href || null;
        return { url: link };
      });

      const isin = id.split("-")[id.split("-").length - 1];

      // save prices to file
      const pricesUrl = `${url}historical-data/?c48840%5BDOWNLOAD%5D=csv&c48840%5BDATETIME_TZ_END_RANGE%5D=${tillDate}&c48840%5BDATETIME_TZ_START_RANGE%5D=01%2F01%2F2015`;
      const csvPath = path.join(__dirname, "temp_data", "downloaded.csv");
      const jsonPath = path.join(
        __dirname,
        "data_stocks",
        `${stockData.symbol}.json`
      );
      await downloadFile(pricesUrl, csvPath);

      const csvData = await parseCsv(csvPath);

      const jsonDataArr = csvData.map((row) => ({
        date: formatDate(row["Date"]),
        trading_model_id: "CT",
        open_price: row["Open"],
        last_price: row["Last Close"],
        high_price: row["High"],
        low_price: row["Low"],
        change_prev_close_percentage: row["Chg.%"],
        volume: row["Total Volume1"].replace(/,/g, ""),
        vwap_price: null,
        num_trades: null,
        turnover: row["Total Value1"].replace(/,/g, ""),
        price_currency: "EUR",
        turnover_currency: "EUR",
      })).reverse(); // reverse to have oldest first

      const jsonData = {
        timestamp: "",
        mic: "XWBO",
        symbol: stockData.symbol,
        isin: isin,
        history: jsonDataArr,
      };

      const lastPrice = jsonData.history[0].last_price;

      await fs1.writeFile(jsonPath, JSON.stringify(jsonData, null, 2));

      fs.unlinkSync(csvPath);

      // save metadata to file
      const data = {
        ...stockData,
        ...urlData,
        webId: id,
        quantity: Math.round(mcs[isin] / lastPrice) || null,
        id: id.split("-")[id.split("-").length - 1],
      };
      const stockMetadataDir = path.join(__dirname, "metadata_stocks");
      await fs1.mkdir(stockMetadataDir, { recursive: true });
      const filePath = path.join(stockMetadataDir, `${stockData.symbol}.json`);
      await fs1.writeFile(filePath, JSON.stringify(data, null, 2));

      console.log(`Saved metadata for ${stockData.symbol}.`);
    }
  }

  // Close the browser
  await browser.close();
}

main().catch((err) => {
  console.error("Fatal error:", err);
});
