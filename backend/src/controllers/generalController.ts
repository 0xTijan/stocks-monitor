import { Request, Response } from 'express';
import { fetchSymbolIsin, fetchSymbolInfo, fetchSymbolPrices} from '../services/generalService';

export const getSymbolInfo = async (req: Request, res: Response) => {
    const { symbol } = req.params;
    const index = await fetchSymbolInfo(symbol);
    res.json(index);
};

export const getSymbolIsin = async (req: Request, res: Response) => {
    const { symbol } = req.params;
    const index = await fetchSymbolIsin(symbol);
    res.json(index);
};

export const getSymbolPrices = async (req: Request, res: Response) => {
    const { symbol } = req.params;
    const { from, until } = req.query;
    const prices = await fetchSymbolPrices(symbol, from as string, until as string);
    res.json(prices);
};