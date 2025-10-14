import { Request, Response } from 'express';
import { updatePrices } from "../services/dailyUpdatePrices";
import { updatePricesAt } from "../services/dailyUpdatePricesWien";

export const updatePricesController = async (req: Request, res: Response) => {
    await updatePrices();
    await updatePricesAt();
    res.status(200);
};