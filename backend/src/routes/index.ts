import { Router } from 'express';
import dailyUpdateRoute from './daily-update';
import apiRouter from './api';

const router = Router();

router.use('/daily-update', dailyUpdateRoute);
router.use("/api", apiRouter)

export default router;
