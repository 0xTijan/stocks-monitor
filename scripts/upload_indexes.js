const fs = require('fs/promises');
const path = require('path');
const mysql = require('mysql2/promise');
const { DB_OPTIONS } = require('./config');

async function main() {
    const connection = await mysql.createConnection(DB_OPTIONS);

    const indexesDir = path.join(__dirname, 'data_indexes');
    const files = await fs.readdir(indexesDir);

    for (const file of files) {
        if (!file.endsWith('.json')) continue;

        const filePath = path.join(indexesDir, file);
        const content = await fs.readFile(filePath, 'utf-8');
        const data = JSON.parse(content);

        const { isin, mic, symbol, name = null, history } = data;

        try {
            await connection.beginTransaction();

            // Insert into indexes table
            await connection.execute(`
                INSERT INTO indexes (isin, mic, symbol, name)
                VALUES (?, ?, ?, ?)
                ON DUPLICATE KEY UPDATE mic = VALUES(mic), symbol = VALUES(symbol), name = VALUES(name)
            `, [isin, mic, symbol, name]);

            // Insert each daily price
            for (const record of history) {
                await connection.execute(`
                    INSERT INTO index_values (
                        index_isin,
                        date,
                        open_value,
                        high_value,
                        low_value,
                        last_value,
                        change_prev_close_percentage,
                        turnover
                    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                    ON DUPLICATE KEY UPDATE
                        open_value = VALUES(open_value),
                        high_value = VALUES(high_value),
                        low_value = VALUES(low_value),
                        last_value = VALUES(last_value),
                        change_prev_close_percentage = VALUES(change_prev_close_percentage),
                        turnover = VALUES(turnover)
                `, [
                    isin,
                    record.date,
                    record.open_value,
                    record.high_value,
                    record.low_value,
                    record.last_value,
                    record.change_prev_close_percentage,
                    record.turnover,
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
