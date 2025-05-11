import { Router } from 'express';
import {
    getAllStocks,
    getAllStockIds,
    getStockPriceHistory,
    getAllPriceHistories,
    getStockById
} from '../../controllers/stocksController';

const router = Router();

router.get('/', getAllStocks);
router.get('/ids', getAllStockIds);
router.get('/prices', getAllPriceHistories);            // ?from=2024-01-01&until=2024-06-01
router.get('/:stockId/prices', getStockPriceHistory);   // ?from=...&until=...
router.get('/:stockId', getStockById);

export default router;
