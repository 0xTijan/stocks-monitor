import { Router } from 'express';
import { updatePrices } from './main';

const router = Router();

router.get('/', updatePrices);

export default router;
