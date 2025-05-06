const fs = require('fs/promises');
const path = require('path');
const mysql = require('mysql2/promise');
const { DB_OPTIONS } = require('./config.js');

async function main() {
   const connection = await mysql.createConnection(DB_OPTIONS);

    const indexesDir = path.join(__dirname, 'data_indexes');
    const metadataDir = path.join(__dirname, 'metadata_indexes');
    const files = await fs.readdir(indexesDir);

    for (const file of files) {
        if (!file.endsWith('.json')) continue;

        const filePathHistory = path.join(indexesDir, file);
        const contentHistory = await fs.readFile(filePathHistory, 'utf-8');
        const { isin, symbol } = JSON.parse(contentHistory);

        const filePathMetadata = path.join(metadataDir, file);
        const contentMetadata = await fs.readFile(filePathMetadata, 'utf-8');
        const { composition } = JSON.parse(contentMetadata);

        try {
            await connection.beginTransaction();

            // Insert each daily price
            for (const record of composition) {
                await connection.execute(`
                    INSERT INTO index_members (
                        index_isin,
                        stock_isin,
                        weight
                    )
                    VALUES (?, ?, ?)
                    ON DUPLICATE KEY UPDATE index_isin = VALUES(index_isin), stock_isin = VALUES(stock_isin), weight = VALUES(weight)
                `, [
                    isin,
                    record.isin,
                    record.weight,
                ]);
            }

            await connection.commit();
            console.log(`Imported: ${symbol} (${isin})`);
        } catch (err) {
            await connection.rollback();
            console.error(`Failed to import ${file}:`, err.message);
        }
    }

    await connection.end();
}

main().catch(err => {
    console.error("Fatal error:", err);
});
