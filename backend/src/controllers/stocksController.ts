import { Request, Response } from 'express';
import { fetchAllPriceHistories, fetchAllStockIds, fetchAllStocks, fetchStockPrices } from '../services/stockService';

export const getAllStocks = async (req: Request, res: Response) => {
    const stocks = await fetchAllStocks();
    res.json(stocks);
};

export const getAllStockIds = async (req: Request, res: Response) => {
    const ids = await fetchAllStockIds();
    res.json(ids);
};

export const getStockPriceHistory = async (req: Request, res: Response) => {
    const { stockId } = req.params;
    const { from, until } = req.query;
    const prices = await fetchStockPrices(stockId, from as string, until as string);
    res.json(prices);
};

export const getAllPriceHistories = async (req: Request, res: Response) => {
    const { from, until } = req.query;
    const data = await fetchAllPriceHistories(from as string, until as string);
    res.json(data);
};
