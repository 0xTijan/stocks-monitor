import { getConnection } from "../config/db";
import { fetchStockById, fetchStockPrices } from "./stockService";
import { fetchIndexById, fetchIndexPrices } from "./indexService";

export const fetchSymbolIsin = async (symbol: string) => {
    const connection = await getConnection();
    const [rows]: any = await connection.query('SELECT isin FROM stocks WHERE symbol = ? UNION SELECT isin FROM indexes WHERE symbol = ?', [symbol, symbol]);
    await connection.end();
    return rows[0];
};

export const fetchSymbolInfo = async (symbol: string) => {
    const connection = await getConnection();
    const [rows]: any = await connection.query(
        `SELECT isin, 'stock' AS type FROM stocks WHERE symbol = ?
         UNION
         SELECT isin, 'index' AS type FROM indexes WHERE symbol = ?`,
        [symbol, symbol]
    );

    if (rows.length === 0) {
        await connection.end();
        throw new Error(`Symbol not found: ${symbol}`);
    }

    const isin = rows[0].isin;
    const type = rows[0].type;

    await connection.end();

    if (type === 'stock') {
        let res = await fetchStockById(isin);
        return res;
    } else {
        let res = await fetchIndexById(isin);
        return res;
    }
}

export const fetchSymbolPrices = async (symbol: string, from?: string, until?: string) => {
    const connection = await getConnection();
    const [rows]: any = await connection.query(
        `SELECT isin, 'stock' AS type FROM stocks WHERE symbol = ?
         UNION
         SELECT isin, 'index' AS type FROM indexes WHERE symbol = ?`,
        [symbol, symbol]
    );

    if (rows.length === 0) {
        await connection.end();
        throw new Error(`Symbol not found: ${symbol}`);
    }

    const isin = rows[0].isin;
    const type = rows[0].type;

    await connection.end();

    if (type === 'stock') {
        let res = await fetchStockPrices(isin, from, until);
        return res;
    } else {
        let res = await fetchIndexPrices(isin, from, until);
        return res;
    }
};