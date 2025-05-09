import express, { Request, Response } from 'express';
import { updatePrices } from './daily_update/main';

const app = express();
const port = process.env.PORT || 3000;

app.get('/', (req: Request, res: Response) => {
    res.send('Hello, TypeScript Express!');
});

app.get("/daily-update", async (req: Request, res: Response) => {
    console.log("updating daily prices...");
    await updatePrices();
    console.log("Daily prices updated successfully.");
    res.status(200).send("Daily prices updated successfully.");
});

app.listen(port, () => {
    console.log(`Server running ${port}`);
});