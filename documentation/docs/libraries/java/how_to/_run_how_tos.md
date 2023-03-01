
### Set Up Your .env file

The code examples use a `.env` file to store variables. You can download
the [example file](https://github.com/iotaledger/wallet.rs/blob/develop/wallet/.env.example) or create a new one with
the following variables:

```dotenv
NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC="endorse answer radar about source reunion marriage tag sausage weekend frost daring base attack because joke dream slender leisure group reason prepare broken river"
STRONGHOLD_PASSWORD="some_hopefully_secure_password"
NODE_URL="http://localhost:14265"
FAUCET_URL="http://localhost:8091/api/enqueue"
```

You should update the `NODE_URL` and `FAUCET_URL` values to match the [Hornet node](#hornet-node) you want to use.

## Run Code Examples

The IOTA Wallet Java library has numerous [examples](https://github.com/iotaledger/wallet.rs/tree/develop/wallet/bindings/java/examples/src)
you can run to get acquainted with the library. You can run any example with the following
command from the `examples` directory:

```bash
.././gradlew run -Pexample=CreateAccount
```

To run another example of your choice, replace `CreateAccount` with the name of any other example from the [Java examples directory](https://github.com/iotaledger/wallet.rs/tree/develop/wallet/bindings/java/examples/src).