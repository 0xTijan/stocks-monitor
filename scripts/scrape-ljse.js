/**
 * scrape-ljse.js (compatible version)
 * Node.js + Puppeteer scraper for LJSE instrument page.
 * Works with older and newer Puppeteer versions (uses sleep fallback).
 *
 * Usage:
 *   npm init -y
 *   npm install puppeteer
 *   node scrape-ljse.js
 */

const puppeteer = require("puppeteer");
const fs = require("fs");
const path = require("path");

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

(async () => {
  const url =
    "https://ljse.si/en/papir-311/310?isin=SI0031102120&tab=stock_publisher";
  const outputFile = path.resolve(process.cwd(), "ljse_financials.json");

  const browser = await puppeteer.launch({
    headless: false,
    defaultViewport: { width: 1600, height: 1200 },
    args: ["--no-sandbox", "--disable-setuid-sandbox"],
  });

  try {
    const page = await browser.newPage();
    await page.setViewport({ width: 1400, height: 1000 });

    await page.goto(url, { waitUntil: "networkidle2", timeout: 60000 });

    // use page.waitForTimeout if available, otherwise fallback to sleep
    if (typeof page.waitForTimeout === "function") {
      await page.waitForTimeout(10000);
    } else if (typeof page.waitFor === "function") {
      // older puppeteer versions had page.waitFor(ms)
      await page.waitFor(10000);
    } else {
      await sleep(10000);
    }

    const data = await page.evaluate(() => {
      // Helper: normalize text
      function textOf(el) {
        return (el && el.textContent && el.textContent.trim().replace(/\u00a0/g, " ")) || "";
      }

      // Find element by (partial, case-insensitive) text
      function findElementByText(root, text) {
        const re = new RegExp(text.replace(/\s+/g, "\\s*"), "i");
        const headings = Array.from(root.querySelectorAll("h1,h2,h3,h4,legend,caption,div,span,p"));
        for (const el of headings) {
          if (el.textContent && re.test(el.textContent)) return el;
        }
        // fallback search among all elements
        const all = Array.from(root.querySelectorAll("*"));
        return all.find(el => el.textContent && re.test(el.textContent));
      }

      function parseContainer(container) {
        const result = { rows: [] };
        if (!container) return result;

        // Prefer table parsing
        const tables = Array.from(container.querySelectorAll("table"));
        if (tables.length > 0) {
          const t = tables[0];
          const headerCells = Array.from(t.querySelectorAll("thead th")).map(h => (h.textContent || "").trim());
          if (headerCells.length > 0) result.headers = headerCells;

          // collect rows (tbody tr, or all tr excluding pure header rows)
          const rows = Array.from(t.querySelectorAll("tbody tr, tr")).filter(tr => {
            const children = Array.from(tr.children);
            return !children.every(c => /^th$/i.test(c.tagName));
          });

          rows.forEach(r => {
            const cells = Array.from(r.querySelectorAll("th,td")).map(c => (c.textContent || "").trim().replace(/\u00a0/g, " "));
            if (cells.length === 0) return;
            const metric = cells[0];
            const values = cells.slice(1);
            result.rows.push({ metric, values });
          });

          if (result.rows.length > 0) return result;
        }

        // fallback: parse div/li/p style rows
        const candidate = Array.from(container.querySelectorAll("div, li, p")).filter(el => {
          const kids = Array.from(el.children).filter(c => (c.textContent || "").trim().length > 0);
          return kids.length >= 2;
        });

        candidate.forEach(el => {
          const kids = Array.from(el.children).map(c => (c.textContent || "").trim().replace(/\u00a0/g, " "));
          if (kids.length >= 2) {
            result.rows.push({ metric: kids[0], values: kids.slice(1) });
          } else {
            const text = (el.textContent || "").trim();
            const sepMatch = text.match(/^(.+?)\s{2,}(.+)$/);
            if (sepMatch) {
              result.rows.push({ metric: sepMatch[1].trim(), values: [sepMatch[2].trim()] });
            }
          }
        });

        return result;
      }

      const targets = ["Financial Data", "Company Valuation Ratios", "Trading Data"];
      const out = {};

      targets.forEach(label => {
        const el = findElementByText(document, label);
        if (!el) {
          out[label] = { rows: [] };
          return;
        }

        // Try to pick a sensible container near the heading
        let container = null;
        const next = el.nextElementSibling;
        if (next && (next.querySelector("table") || /[0-9]{3,}/.test(next.textContent || ""))) {
          container = next;
        }
        if (!container) container = el.parentElement;
        if (!container) container = el.closest("section,article,div");

        out[label] = parseContainer(container);
      });

      return {
        scrapedAt: new Date().toISOString(),
        foundUrl: location.href,
        sections: {
          financialData: out["Financial Data"] || { rows: [] },
          valuationRatios: out["Company Valuation Ratios"] || { rows: [] },
          tradingData: out["Trading Data"] || { rows: [] },
        },
      };
    });

    fs.writeFileSync(outputFile, JSON.stringify(data, null, 2), "utf8");
    console.log("Saved output to:", outputFile);
    console.log("Summary:", {
      financialRows: data.sections.financialData.rows.length,
      valuationRows: data.sections.valuationRatios.rows.length,
      tradingRows: data.sections.tradingData.rows.length,
    });
  } catch (err) {
    console.error("Error scraping page:", err);
  } finally {
    await browser.close();
  }
})();
