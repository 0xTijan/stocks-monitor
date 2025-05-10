import { Router } from 'express';
import stocksRouter from './stocks';
import indexesRouter from './indexes';

const router = Router();

router.use("/stocks", stocksRouter);
router.use("/indexes", indexesRouter);

export default router;
