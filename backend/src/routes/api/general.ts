import { Router } from 'express';
import {
    getSymbolInfo,
    getSymbolPrices,
    getSymbolIsin,
} from '../../controllers/generalController';

const router = Router();

router.get('/:symbol/info', getSymbolInfo);
router.get('/:symbol/prices', getSymbolPrices);          // ?from=2024-01-01&until=2024-06-01
router.get('/:symbol/isin', getSymbolIsin);

export default router;
