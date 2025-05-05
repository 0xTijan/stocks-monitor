const fs = require('fs/promises');
const path = require('path');
const { STOCK_IDS, INDEX_IDS } = require('./config');

const DATE_FROM = '2015-01-01';

const getFormattedDate = () => {
    const now = new Date();

    const yyyy = now.getFullYear();
    const dd = String(now.getDate()).padStart(2, '0');
    const mm = String(now.getMonth() + 1).padStart(2, '0'); // Months are 0-based

    const formattedDate = `${yyyy}-${dd}-${mm}`;
    return formattedDate;
}

async function fetchAndSave(url, id, outputDir) {
    console.log(`Fetching data for ISIN: ${id}`);
    try {
        const response = await fetch(url);
        if (!response.ok) throw new Error(`HTTP ${response.status}`);
        const data = await response.json();

        const symbol = data.symbol || 'unknown_symbol';
        const filePath = path.join(outputDir, `${symbol}.json`);
        await fs.writeFile(filePath, JSON.stringify(data, null, 2));

        console.log(`Saved to: ${path.join(path.basename(outputDir), `${symbol}.json`)}`);
    } catch (err) {
        console.error(`Error fetching ${id}: ${err.message}`);
    }
}

async function main() {
    const stockDir = path.join(__dirname, 'data_stocks');
    const indexDir = path.join(__dirname, 'data_indexes');

    await fs.mkdir(stockDir, { recursive: true });
    await fs.mkdir(indexDir, { recursive: true });

    const DATE_TILL = getFormattedDate();

    for (const id of STOCK_IDS) {
        const url = id.startsWith("HR")
            ? `https://rest.zse.hr/web/Bvt9fe2peQ7pwpyYqODM/security-history/XZAG/${id}/${DATE_FROM}/${DATE_TILL}/json`
            : `https://rest.ljse.si/web/Bvt9fe2peQ7pwpyYqODM/security-history/XLJU/${id}/${DATE_FROM}/${DATE_TILL}/json`;
        await fetchAndSave(url, id, stockDir);
    }

    for (const id of INDEX_IDS) {
        const url = id.startsWith("HR")
            ? `https://rest.zse.hr/web/Bvt9fe2peQ7pwpyYqODM/index-history/XZAG/${id}/${DATE_FROM}/${DATE_TILL}/json`
            : `https://rest.ljse.si/web/Bvt9fe2peQ7pwpyYqODM/index-history/XLJU/${id}/${DATE_FROM}/${DATE_TILL}/json`;
        await fetchAndSave(url, id, indexDir);
    }
}

main().catch(err => {
    console.error("Unexpected error:", err);
});
