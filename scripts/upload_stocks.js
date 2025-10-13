const fs = require('fs/promises');
const path = require('path');
const mysql = require('mysql2/promise');
const { DB_OPTIONS } = require('./config');

async function main() {
    const connection = await mysql.createConnection(DB_OPTIONS);

    const stocksDir = path.join(__dirname, 'data_stocks');
    const metadataDir = path.join(__dirname, 'metadata_stocks');
    const files = await fs.readdir(stocksDir);

    for (const file of files) {
        if (!file.endsWith('.json')) continue;
        
        const filePathHistory = path.join(stocksDir, file);
        const contentHistory = await fs.readFile(filePathHistory, 'utf-8');
        const { isin, mic, symbol, history } = JSON.parse(contentHistory);

        const filePathMetadata = path.join(metadataDir, file);
        const contentMetadata = await fs.readFile(filePathMetadata, 'utf-8');
        const { name, logo, nace, sectorId, sectorName, firstDay, quantity, description, url, webId } = JSON.parse(contentMetadata);
        console.log(`Processing: `, name, logo, nace, sectorId, sectorName, firstDay, quantity, url, webId);
        try {
            await connection.beginTransaction();

            // Insert into stocks table
            await connection.execute(`
                INSERT INTO stocks (
                    isin,
                    mic,
                    symbol,
                    name,
                    nace,
                    sector_id,
                    sector_name,
                    first_trading_date,
                    quantity,
                    description,
                    logo_url,
                    website_url,
                    last_price,
                    change_prev_close_percentage,
                    web_id
                )
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                ON DUPLICATE KEY UPDATE mic = VALUES(mic), symbol = VALUES(symbol), name = VALUES(name)
            `, [
                isin,
                mic,
                symbol,
                name,
                nace,
                sectorId,
                sectorName,
                firstDay,
                quantity,
                description,
                logo,
                url,
                history[0].last_price,
                history[0].change_prev_close_percentage,
                webId
            ]);
            console.log("pricesing daily prices for", symbol);
            // Insert each daily price
            for (const record of history) {
                await connection.execute(`
                    INSERT INTO daily_prices (
                        stock_isin,
                        date,
                        trading_model_id,
                        open_price,
                        high_price,
                        low_price,
                        last_price,
                        vwap_price,
                        change_prev_close_percentage,
                        num_trades,
                        volume,
                        turnover,
                        price_currency,
                        turnover_currency
                    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                    ON DUPLICATE KEY UPDATE
                        open_price = VALUES(open_price),
                        high_price = VALUES(high_price),
                        low_price = VALUES(low_price),
                        last_price = VALUES(last_price),
                        vwap_price = VALUES(vwap_price),
                        change_prev_close_percentage = VALUES(change_prev_close_percentage),
                        num_trades = VALUES(num_trades),
                        volume = VALUES(volume),
                        turnover = VALUES(turnover),
                        price_currency = VALUES(price_currency),
                        turnover_currency = VALUES(turnover_currency)
                `, [
                    isin,
                    record.date,
                    record.trading_model_id,
                    record.open_price,
                    record.high_price,
                    record.low_price,
                    record.last_price,
                    record.vwap_price,
                    record.change_prev_close_percentage,
                    record.num_trades,
                    record.volume,
                    record.turnover,
                    record.price_currency,
                    record.turnover_currency
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
