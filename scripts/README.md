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


SQL schema after which the scripts are formulated is in `/sql`.
