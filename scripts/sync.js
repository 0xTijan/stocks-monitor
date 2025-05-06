const { execSync } = require('child_process');

function runTask(command) {
    try {
        console.log(`\n`);
        console.log(`➡️ Running: ${command}`);
        execSync(command, { stdio: 'inherit' });
        console.log(`✅ Success: ${command}`);
    } catch (err) {
        console.error(`❌ Failed: ${command}`);
        throw err;
    }
}

function main() {
    runTask("node get.js");
    runTask("node get_stocks_metadata.js");
    runTask("get_indexes_metadata.js");
    runTask("node upload_stocks.js");
    runTask("node upload_indexes.js");
    runTask("node upload_index_members.js");
}

main();
