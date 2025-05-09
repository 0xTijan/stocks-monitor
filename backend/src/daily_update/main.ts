import axios from 'axios';
import { getConnection } from "../config";
import { sendErrorEmail, sendSuccessEmail } from './send_email';
import { RowDataPacket } from 'mysql2';

const getFormattedDate = (n: number) => {
  const now = new Date();

  const yyyy = now.getFullYear();
  const dd = String(now.getDate() - n).padStart(2, '0');
  const mm = String(now.getMonth() + 1).padStart(2, '0');

  return `${yyyy}-${mm}-${dd}`;
}

export const updatePrices = async() => {
    try {
        const conn = await getConnection();

        const [stockRows] = await conn.query<RowDataPacket[]>('SELECT isin FROM stocks');
        const [indexRows] = await conn.query<RowDataPacket[]>('SELECT isin FROM indexes');    
            
        for (const stock of stockRows) {
            const DATE_TILL = getFormattedDate(0);
            const DATE_FROM = getFormattedDate(1);
            const isin = stock.isin;

            const url = isin.startsWith('HR')
                ? `https://rest.zse.hr/web/Bvt9fe2peQ7pwpyYqODM/security-history/XZAG/${isin}/${DATE_FROM}/${DATE_TILL}/json`
                : `https://rest.ljse.si/web/Bvt9fe2peQ7pwpyYqODM/security-history/XLJU/${isin}/${DATE_FROM}/${DATE_TILL}/json`;

            try {
                const response = await axios.get(url);
                const record = response.data.history[0];

                if (record) {
                    console.log(`Saving data for ISIN: ${isin}`);
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
                        isin, record.date, record.trading_model_id, record.open_price,
                        record.high_price, record.low_price, record.last_price,
                        record.vwap_price, record.change_prev_close_percentage,
                        record.num_trades, record.volume, record.turnover,
                        record.price_currency, record.turnover_currency
                    ]);
                }
            } catch (error) {
                let err = error as any;
                if (err.response && err.response.status === 400) {
                    console.warn(`400 Bad Request for ISIN ${isin}. Skipping.`);
                } else {
                    console.error(`Error for ISIN ${isin}:`, err);
                }
            }
        }

        for (const index of indexRows) {
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
                }
            } catch (error) {
                let err = error as any;
                if (err.response && err.response.status === 400) {
                    console.warn(`400 Bad Request for ISIN ${isin}. Skipping.`);
                } else {
                    console.error(`Error for ISIN ${isin}:`, err);
                }
            }
        }

        await conn.end();
        await sendSuccessEmail();
        console.log('Task completed successfully.');
    } catch (error) {
        console.error('Error occurred:', error);
        await sendErrorEmail(error);
        process.exit(1);
    }
}
