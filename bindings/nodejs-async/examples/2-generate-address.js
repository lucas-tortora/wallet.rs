/**
 * This example genrates a new address.
 */


async function run() {
    const { MessageHandler } = require('../lib/index.js');
    const manager = new MessageHandler({
        storagePath: './alice-database',
    });
    try {
        await manager.setStrongholdPassword("A12345678*");

        const account = await manager.getAccount('Alice')
        console.log('Account:', account)

        // Always sync before doing anything with the account
        const synced = await account.sync()
        console.log('Syncing... - ' + synced)

        const address = await account.generateAddress()
        console.log('New address:', address)

        // You can also get the latest unused address:
        const addressObject = await account.latestAddress()
        console.log("Address:", addressObject)

        // Use the Chrysalis Faucet to send testnet tokens to your address:
        // console.log("Fill your address with the Faucet: https://faucet.testnet.chrysalis2.com/")
    } catch (error) {
        console.log("Error: " + error)
    }
}

run()
