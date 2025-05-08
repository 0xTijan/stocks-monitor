import cron from 'node-cron';
import { updatePrices } from './task.js';

// update prices every day at 16:30
cron.schedule('*/5 * * * *', async () => {
    console.log(`[${new Date().toISOString()}] ⏰ Running daily task...`);
    try {
        await updatePrices();
        console.log(`[${new Date().toISOString()}] ✅ Task completed`);
    } catch (err) {
        console.error(`[${new Date().toISOString()}] ❌ Error occurred:`, err);
    }
}, {
    timezone: "Europe/Ljubljana"
});