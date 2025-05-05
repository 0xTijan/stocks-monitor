const fs = require('fs/promises');
const path = require('path');

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
    const stockIds = [
        // 1. market - si
        "SI0031103805", "SI0031102120", "SI0031101346", "SI0021117344",
        "SI0031102153", "SI0021110513", "SI0031104290", "SI0021111651",
        // 2. market - si
        "SI0031108994", "SI0031117813",

        // 1. market - hr
        "HRADPLRA0006", "HRARNTRA0004", "HRATGRRA0003", "HRHT00RA0005",
        "HRPODRRA0004", "HRRIVPRA0000",
        // 2. market - hr
        "HRSPANRA0007", "HRIG00RA0009", "HRLKRIRA0007", "HRMRULRA0009",
        "HRHPB0RA0002", "HRKOEIRA0009", "HRDLKVRA0006", "HRMDKARA0000",
        "HRGRNLRA0006", "HRZB00RA0003", "HRIGH0RA0006", "HRLKPCRA0005",
        // 3. market - hr
        "HRVLENRB0001", "HRERNTRA0000", "HRJDGTRA0000", "HRKRASRA0008",
        "HRAUHRRA0009", "HRKODTRA0007", "HRULPLRA0002", "HRADRSPA0009",
        "HRZABARA0009", "HRDDJHRA0007", "HRINGRRA0001", "HRPLAGRA0003",
        "HRIKBARA0008", "HRJDPLRA0007", "HRKTJVRA0002", "HRMONPRA0007",

    ];

    const indexIds = [
        "SI0026109882", "SI0028409892", "HRZB00ICBEX6", "HRZB00ICBTR6",
        "HRZB00ICBE11", "HRZB00ICB103", "HRZB00ICBPR4", "HRZB00ICBEP2",
        "HRZB00IADPR4",
    ];

    const stockDir = path.join(__dirname, 'data_stocks');
    const indexDir = path.join(__dirname, 'data_indexes');

    await fs.mkdir(stockDir, { recursive: true });
    await fs.mkdir(indexDir, { recursive: true });

    const DATE_TILL = getFormattedDate();

    for (const id of stockIds) {
        const url = id.startsWith("HR")
            ? `https://rest.zse.hr/web/Bvt9fe2peQ7pwpyYqODM/security-history/XZAG/${id}/${DATE_FROM}/${DATE_TILL}/json`
            : `https://rest.ljse.si/web/Bvt9fe2peQ7pwpyYqODM/security-history/XLJU/${id}/${DATE_FROM}/${DATE_TILL}/json`;
        await fetchAndSave(url, id, stockDir);
    }

    for (const id of indexIds) {
        const url = id.startsWith("HR")
            ? `https://rest.zse.hr/web/Bvt9fe2peQ7pwpyYqODM/index-history/XZAG/${id}/${DATE_FROM}/${DATE_TILL}/json`
            : `https://rest.ljse.si/web/Bvt9fe2peQ7pwpyYqODM/index-history/XLJU/${id}/${DATE_FROM}/${DATE_TILL}/json`;
        await fetchAndSave(url, id, indexDir);
    }
}

main().catch(err => {
    console.error("Unexpected error:", err);
});
