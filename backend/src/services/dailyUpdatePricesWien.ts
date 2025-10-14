import axios from 'axios';
import { RowDataPacket } from 'mysql2';
import { getConnection } from '../config/db';
import { sendErrorEmail, sendSuccessEmail } from '../helpers/email';

const getFormattedDate = (n: number) => {
  const now = new Date();

  const yyyy = now.getFullYear();
  const dd = String(now.getDate() - n).padStart(2, '0');
  const mm = String(now.getMonth() + 1).padStart(2, '0');

  return `${mm}%2F${dd}%2F${yyyy}`;
}

const getFormattedDateToSave = (n: number) => {
  const now = new Date();

  const yyyy = now.getFullYear();
  const dd = String(now.getDate() - n).padStart(2, '0');
  const mm = String(now.getMonth() + 1).padStart(2, '0');

  return `${yyyy}-${mm}-${dd}`;
}

type RecordData = Record<string, string>;

function csvToRecord(csv: string): RecordData {
  // Split into non-empty lines
  const lines = csv.trim().split(/\r?\n/).filter(Boolean);
  if (lines.length < 2) return {};

  // Parse header and first data row
  const headers = lines[0]
    .split(';')
    .map(h => h.replace(/[";]/g, '').trim())
    .filter(Boolean);

  const values = lines[1]
    .split(';')
    .map(v => v.replace(/[";]/g, '').trim())
    .filter(Boolean);

  // Convert header names -> camelCase keys
  const normalizeKey = (key: string): string =>
    key
      .toLowerCase()
      .replace(/\s*%/g, 'Pct')          // Replace '%' with 'Pct'
      .replace(/\s+/g, ' ')             // Normalize spaces
      .replace(/\s(.)/g, (_, c) => c.toUpperCase()) // CamelCase
      .replace(/\s/g, '')               // Remove spaces
      .replace('.', '');                // Remove dots

  const record: RecordData = {};

  headers.forEach((header, i) => {
    const key = normalizeKey(header);
    const value = values[i];
    if (value !== undefined && value !== '') {
      record[key] = value;
    }
  });

  return record;
}


export const updatePricesAt = async() => {
    console.log('Starting daily update task - AT...');
    try {
        const conn = await getConnection();

        const [stockRows] = await conn.query<RowDataPacket[]>('SELECT isin, web_id FROM stocks');
        // const [indexRows] = await conn.query<RowDataPacket[]>('SELECT isin FROM indexes');    
            
        for (const stock of stockRows) {
            const isin = stock.isin;
            const web_id = stock.web_id;
            if (isin.startsWith("AT")) {
                const DATE_TILL = getFormattedDate(0);
                const DATE_FROM = getFormattedDate(0);
                const DATE_TO_SAVE = getFormattedDateToSave(0);
                console.log(web_id)
                const url = `https://www.wienerborse.at/en/stock-prime-market/${web_id}/historical-data/?c48840%5BDOWNLOAD%5D=csv&c48840%5BDATETIME_TZ_END_RANGE%5D=${DATE_TILL}&c48840%5BDATETIME_TZ_START_RANGE%5D=${DATE_FROM}`;
                console.log("URL:", url);
                try {
                    const response = await axios.get(url);
                    const record = csvToRecord(response.data);
                    if (record) {
                        console.log(`Saving data for ISIN: ${isin}`);
                        // insert or update daily prices
                        await conn.execute(`
                            INSERT INTO daily_prices (
                                stock_isin, date, trading_model_id, open_price, high_price,
                                low_price, last_price, vwap_price, change_prev_close_percentage,
                                num_trades, volume, turnover, price_currency, turnover_currency
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
                            isin, DATE_TO_SAVE, "CT", Number(record.open) ?? null,
                            Number(record.high) ?? null, Number(record.low) ?? null, Number(record.lastClose) ?? null,
                            null, Number(record.chgPct.replace('%', '')) ?? null,
                            null, Number(record.totalVolume1.replace(/,/g, "")) ?? null, Number(record.totalValue1.replace(/,/g, "")) ?? null,
                            "EUR", "EUR"
                        ]);

                        // update price in stocks table               
                        await conn.execute(`
                            UPDATE stocks
                            SET 
                                last_price = ?, 
                                change_prev_close_percentage = ?
                            WHERE isin = ?
                        `, [
                            Number(record.lastClose) ?? null, Number(record.chgPct.replace('%', '')) ?? null, isin
                        ]);
                    }
                } catch (error) {
                    let err = error as any;
                    if (err.response && err.response.status === 400) {
                        console.warn(`400 Bad Request for ISIN ${isin}. Skipping.`);
                    } else {
                        console.error(`Error for ISIN ${isin}:`, err);
                    }
                    // update change_prev_close_percentage to null - since we don't have new data              
                    await conn.execute(`
                        UPDATE stocks
                        SET 
                            change_prev_close_percentage = ?
                        WHERE isin = ?
                    `, [
                        0, isin
                    ]);
                }
            }
        }

        /*for (const index of indexRows) {
            const DATE_TILL = getFormattedDate(0);
            const DATE_FROM = getFormattedDate(1);
            const isin = index.isin;

            const url = isin.startsWith('HR')
                ? `https://rest.zse.hr/web/Bvt9fe2peQ7pwpyYqODM/index-history/XZAG/${isin}/${DATE_FROM}/${DATE_TILL}/json`
                : `https://rest.ljse.si/web/Bvt9fe2peQ7pwpyYqODM/index-history/XLJU/${isin}/${DATE_FROM}/${DATE_TILL}/json`;

            try {
                const response = await axios.get(url);
                const record = response.data.history[0];

                if (record) {
                    console.log(`Saving data for ISIN: ${isin}`);
                    // insert or update daily prices
                    await conn.execute(`
                        INSERT INTO index_values (
                            index_isin, date, open_value, high_value, low_value,
                            last_value, change_prev_close_percentage, turnover
                        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                        ON DUPLICATE KEY UPDATE
                            open_value = VALUES(open_value),
                            high_value = VALUES(high_value),
                            low_value = VALUES(low_value),
                            last_value = VALUES(last_value),
                            change_prev_close_percentage = VALUES(change_prev_close_percentage),
                            turnover = VALUES(turnover)
                    `, [
                        isin, record.date, record.open_value, record.high_value,
                        record.low_value, record.last_value,
                        record.change_prev_close_percentage, record.turnover
                    ]);

                    // update price in indexes table
                    await conn.execute(`
                        UPDATE indexes
                        SET 
                            last_value = ?, 
                            change_prev_close_percentage = ?
                        WHERE isin = ?
                    `, [
                        record.last_value, record.change_prev_close_percentage, isin
                    ]);
                }
            } catch (error) {
                let err = error as any;
                if (err.response && err.response.status === 400) {
                    console.warn(`400 Bad Request for ISIN ${isin}. Skipping.`);
                } else {
                    console.error(`Error for ISIN ${isin}:`, err);
                }
                // update change_prev_close_percentage to null - since we don't have new data
                await conn.execute(`
                    UPDATE indexes
                    SET 
                        change_prev_close_percentage = ?
                    WHERE isin = ?
                `, [
                    0, isin
                ]);
            }
        }*/

        await conn.end();
        await sendSuccessEmail();
        console.log('Task completed successfully.');
    } catch (error) {
        console.error('Error occurred:', error);
        await sendErrorEmail(error);
        process.exit(1);
    }
}
