const { execSync } = require('child_process');

function runTask(command) {
    try {
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
    runTask("upload_stocks.js");
    runTask("upload_indexes.js");
    runTask("get_stocks_metadata.js");
    runTask("upload_index_members.js");
}

main();
