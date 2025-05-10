import { Router } from 'express';
import { updatePricesController } from '../../controllers/dailyUpdateController';

const router = Router();

router.get('/', updatePricesController);

export default router;
