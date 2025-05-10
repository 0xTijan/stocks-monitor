import { Router } from 'express';
import dailyUpdateRoute from './daily-update';

const router = Router();

router.use('/daily-update', dailyUpdateRoute);

export default router;
