import { Router } from 'express';
import stocksRouter from './stocks';
import indexesRouter from './indexes';
import generalRouter from './general';

const router = Router();

router.use("/stocks", stocksRouter);
router.use("/indexes", indexesRouter);
router.use("/general", generalRouter);

export default router;
