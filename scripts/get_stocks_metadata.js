const puppeteer = require("puppeteer");
const { STOCK_IDS } = require("./config");
const path = require('path');
const fs = require('fs/promises');


// SCRAPE FOR METADATA
async function main() {
    // start a puppeteer session
    const browser = await puppeteer.launch();

    for (const id of STOCK_IDS) {
        try {
            console.log(`Scraping data for ${id}...`);
            // open a new page
            const instrumentPage = await browser.newPage();
            const dataPage = await browser.newPage();

            const url = id.startsWith("HR")
                ? `https://zse.hr/en/papir-311/310?isin=${id}`
                : `https://ljse.si/en/papir-311/310?isin=${id}`;

            // open this URL, and wait for the DOM to load
            await instrumentPage.goto(`${url}&tab=stock_info`, { waitUntil: "domcontentloaded" });
            await dataPage.goto(`${url}&tab=stock_publisher`, { waitUntil: "domcontentloaded" });

            // get instrument page data { fullName, logo, quantity, sectorId, sectorName, firstDay, nace }
            const instrumentData = await instrumentPage.evaluate(() => {
                // heading
                const heading = document.querySelector(".stock-page-left");
                const name = heading.querySelector("h1").innerText;
                const logo = heading.querySelector("img")?.src || null;
                const symbol = heading.querySelector(".stock-short").innerText;

                // industry
                const industry = document.querySelector(".industry");
                const sector = industry.querySelector("a").innerText.split("·");
                const sectorId = sector[0].trim();
                const sectorName = sector[1].trim();
                const allLisIndustry = industry.querySelectorAll('li');
                const _nace = allLisIndustry[allLisIndustry.length - 1].textContent.trim();
                const nace = _nace.split("·")[1].trim();
                
                // quantity & first day
                const div = document.querySelector("#stock_info");
                const blockWrappers = div.querySelectorAll('div.block-wrapper');
                const secondBlockWrapper = blockWrappers[1];
                const blockInner = secondBlockWrapper.querySelector('div.block-inner');
                const uls = blockInner.querySelectorAll('ul');
                const _firstDay = uls[4].querySelectorAll('li')[1].textContent.trim();                  
                const firstDay = new Date(_firstDay).toISOString().split('T')[0];
                const quantity = uls[5].querySelectorAll('li')[1].textContent.trim().replaceAll(",", "");

                return { name, logo, symbol, nace, sectorId, sectorName, firstDay, quantity };
            });

            // get data page data { description, url }
            const pageData = await dataPage.evaluate(() => {
                const description = document.querySelector(".issuer-description")?.innerText || null; 
                const url = document.querySelector("#stock_publisher .block-wrapper a")?.href || null;

                return { description, url };
            });

            // save metadata to file
            const data = {
                ...instrumentData,
                ...pageData,
                id,
            };
            const stockMetadataDir = path.join(__dirname, 'metadata_stocks');
            await fs.mkdir(stockMetadataDir, { recursive: true });
            const filePath = path.join(stockMetadataDir, `${instrumentData.symbol}.json`);
            await fs.writeFile(filePath, JSON.stringify(data, null, 2));

            console.log(`Saved metadata for ${instrumentData.symbol}.`);
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
