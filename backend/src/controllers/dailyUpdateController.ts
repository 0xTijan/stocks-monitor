import { Request, Response } from 'express';
import { updatePrices } from "../services/dailyUpdatePrices";

export const updatePricesController = async (req: Request, res: Response) => {
    await updatePrices();
    res.status(200);
};