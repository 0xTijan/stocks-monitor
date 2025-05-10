import { Router } from 'express';
import {
    getAllIndexes,
    getAllIndexIds,
    getAllPriceHistories,
    getIndexPriceHistory
} from '../../controllers/indexesController';

const router = Router();

router.get('/', getAllIndexes);
router.get('/ids', getAllIndexIds);
router.get('/prices', getAllPriceHistories);              // ?from=2024-01-01&until=2024-06-01
router.get('/:indexId/prices', getIndexPriceHistory);     // ?from=...&until=...

export default router;
