const puppeteer = require("puppeteer");
const { STOCK_IDS } = require("./config");
const path = require('path');
const fs = require('fs/promises');


// SCRAPE FOR METADATA
async function main() {
    // start a puppeteer session
    const browser = await puppeteer.launch({
        headless: false,
        defaultViewport: { width: 1600, height: 1200 },
    });

    for (const id of STOCK_IDS) {
        try {
            console.log(`Scraping FINANCIAL data for ${id}...`);
            // open a new page
            const financialsPage = await browser.newPage();

            const url = `https://ljse.si/en/papir-311/310?isin=${id}&tab=stock_publisher`;

            // open this URL, and wait for the DOM to load
            await financialsPage.goto(url, { waitUntil: "networkidle2" });
            await financialsPage.waitForSelector(".th-inner.form-control");
            const financialsData = await financialsPage.evaluate(() => {
                const heading1 = document.querySelectorAll(".block-title");
                let heading = [];
                heading1.forEach(h => heading.push(h.innerText.trim()));

                return { heading: heading };
            });

            // save metadata to file
            const data = {
                ...financialsData,
                id,
            };

            console.log(data);
            /*const stockMetadataDir = path.join(__dirname, 'metadata_stocks');
            await fs.mkdir(stockMetadataDir, { recursive: true });
            const filePath = path.join(stockMetadataDir, `${financialsData.symbol}.json`);
            await fs.writeFile(filePath, JSON.stringify(data, null, 2));

            console.log(`Saved financials for ${financialsData.symbol}.`);*/
        } catch (err) {
            console.error(`Failed to scrape financial data: `, err.message);
        }
    }

    // Close the browser
    await browser.close();
}

main().catch(err => {
    console.error("Fatal error:", err);
});
