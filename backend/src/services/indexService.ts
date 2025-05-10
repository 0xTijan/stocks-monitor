import { getConnection } from "../config/db";

export const fetchAllIndexes = async () => {
    const connection = await getConnection();
    const [rows] = await connection.query('SELECT * FROM indexes');
    await connection.end();
    return rows;
};

export const fetchAllIndexIds = async () => {
    const connection = await getConnection();
    const [rows] = await connection.query('SELECT isin FROM indexes');
    await connection.end();
    return rows;
};

export const fetchIndexPrices = async (indexId: string, from?: string, until?: string) => {
    const connection = await getConnection();
    const params: any[] = [indexId];
    let query = `SELECT * FROM index_values WHERE index_isin = ?`;

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
    let query = `SELECT * FROM index_values WHERE 1=1`;

    if (from) {
        query += ' AND date >= ?';
        params.push(from);
    }
    if (until) {
        query += ' AND date <= ?';
        params.push(until);
    }

    query += ' ORDER BY index_isin, date ASC';
    const [rows] = await connection.query(query, params);
    await connection.end();

    // Group rows by index_isin
    const grouped: Record<string, any[]> = {};
    for (const row of rows as any[]) {
        if (!grouped[row.index_isin]) {
            grouped[row.index_isin] = [];
        }
        grouped[row.index_isin].push(row);
    }

    return grouped;
};
