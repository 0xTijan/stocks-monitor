import { getConnection } from "../config/db";

export const fetchAllStocks = async () => {
    const connection = await getConnection();
    const [rows] = await connection.query('SELECT * FROM stocks');
    await connection.end();
    return rows;
};

export const fetchAllStockIds = async () => {
    const connection = await getConnection();
    const [rows] = await connection.query('SELECT isin FROM stocks');
    await connection.end();
    return rows;
};

export const fetchStockById = async (stockId: string) => {
    const connection = await getConnection();
    const [rows]: any = await connection.query('SELECT * FROM stocks WHERE isin = ?', [stockId]);
    await connection.end();
    return rows[0];
};

export const fetchStockPrices = async (stockId: string, from?: string, until?: string) => {
    const connection = await getConnection();
    const params: any[] = [stockId];
    let query = `SELECT * FROM daily_prices WHERE stock_isin = ?`;

    if (from) {
        query += ' AND date >= ?';
        params.push(from);
    }
    if (until) {
        query += ' AND date <= ?';
        params.push(until);
    }

    query += ' ORDER BY date ASC';
    const [rows] = await connection.query(query, params);
    await connection.end();
    return rows;
};

export const fetchAllPriceHistories = async (from?: string, until?: string) => {
    const connection = await getConnection();
    const params: any[] = [];
    let query = `SELECT * FROM daily_prices WHERE 1=1`;

    if (from) {
        query += ' AND date >= ?';
        params.push(from);
    }
    if (until) {
        query += ' AND date <= ?';
        params.push(until);
    }

    query += ' ORDER BY stock_isin, date ASC';
    const [rows] = await connection.query(query, params);
    await connection.end();

    // Group rows by stock_isin
    const grouped: Record<string, any[]> = {};
    for (const row of rows as any[]) {
        if (!grouped[row.stock_isin]) {
            grouped[row.stock_isin] = [];
        }
        grouped[row.stock_isin].push(row);
    }

    return grouped;
};
