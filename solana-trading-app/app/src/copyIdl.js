const fs = require('fs');
const idl = require('./target/idl/solana_trading_app.json');

fs.writeFileSync('./app/src/idl.json', JSON.stringify(idl));