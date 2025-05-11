import { Request, Response } from 'express';
import { fetchAllIndexes, fetchAllIndexIds, fetchAllPriceHistories, fetchIndexById, fetchIndexPrices } from '../services/indexService';

export const getAllIndexes = async (req: Request, res: Response) => {
    const indexes = await fetchAllIndexes();
    res.json(indexes);
};

export const getAllIndexIds = async (req: Request, res: Response) => {
    const ids = await fetchAllIndexIds();
    res.json(ids);
};

export const getIndexById = async (req: Request, res: Response) => {
    const { indexId } = req.params;
    const index = await fetchIndexById(indexId);
    res.json(index);
};

export const getIndexPriceHistory = async (req: Request, res: Response) => {
    const { indexId } = req.params;
    const { from, until } = req.query;
    const prices = await fetchIndexPrices(indexId, from as string, until as string);
    res.json(prices);
};

export const getAllPriceHistories = async (req: Request, res: Response) => {
    const { from, until } = req.query;
    const data = await fetchAllPriceHistories(from as string, until as string);
    res.json(data);
};
