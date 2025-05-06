const puppeteer = require("puppeteer");
const { INDEX_IDS } = require("./config");
const path = require('path');
const fs = require('fs/promises');


// SCRAPE FOR METADATA
async function main() {
    // start a puppeteer session
    const browser = await puppeteer.launch();

    for (const id of INDEX_IDS) {
        try {
            // open a new page
            const page = await browser.newPage();

            const url = id.startsWith("HR")
                ? `https://zse.hr/en/indeks-366/365?isin=${id}&tab=stock_info`
                : `https://ljse.si/en/indeks-366/365?isin=${id}&tab=stock_info`;

            // open this URL, and wait for the DOM to load
            await page.goto(url, { waitUntil: "domcontentloaded" });

            // get instrument page data { fullName, logo, quantity, sectorId, sectorName, firstDay, nace }
            const compositionData = await page.evaluate(() => {
                // heading
                const heading = document.querySelector(".stock-page-left");
                const symbol = heading.querySelector(".stock-short").innerText;

                // read table data (find tbody each row is a stock)
                // get free float factor and weight and isin
            });

            // save metadata to file
            const data = {
                ...compositionData,
                id,
            };
            const indexesMetadataDir = path.join(__dirname, 'metadata_indexes');
            await fs.mkdir(indexesMetadataDir, { recursive: true });
            const filePath = path.join(indexesMetadataDir, `${compositionData.symbol}.json`);
            await fs.writeFile(filePath, JSON.stringify(data, null, 2));

            console.log(`Saved metadata for ${compositionData.symbol}.`);
        } catch (err) {
            console.error(`Failed to scrape data: `, err.message);
        }
    }

    // Close the browser
    await browser.close();
}

main().catch(err => {
    console.error("Fatal error:", err);
});
