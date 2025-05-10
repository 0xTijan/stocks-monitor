import { Router } from 'express';
import stocksRouter from './stocks';

const router = Router();

router.use("/stocks", stocksRouter);

export default router;
