# Initial scripts
Collection of scripts that have to be run 1 time to setup and fill up the database initially.


**!! run this setup initial scripts run `node sync.js` !!**


Or do it manually:
1. Manually select ids of stocks and indexes to trace and database connection details in `config.js`.
2. Run script `get.js` to get daily prices history for each stock/index - **set the dates as needed**.
3. - Run script `upload_stocks.js` to upload data to MySQL database.
   - Run script `upload_indexes.js` to upload data to MySQL database.
4. Run script `get_stocks_metadata.js` to get metadata for each stock.
5. Run script `upload_index_members.js` to upload final data to database.


**At the end run this in sql to fix croatian currency change!!!**
```
UPDATE daily_prices
SET
    open_price = open_price / 7.5,
    high_price = high_price / 7.5,
    low_price = low_price / 7.5,
    last_price = last_price / 7.5
WHERE
    stock_isin LIKE 'HR%' AND
    date < '2023-01-01';

```

SQL schema after which the scripts are formulated is in `/sql`.
