const fs = require('fs/promises');
const path = require('path');
const mysql = require('mysql2/promise');
const { DB_OPTIONS } = require('./config.js');

async function main() {
    const connection = await mysql.createConnection(DB_OPTIONS);
    console.log(connection.config.database);
}

main().catch(err => {
    console.error("Fatal error:", err);
});
