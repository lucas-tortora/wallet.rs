## Prerequisites

Before you can run the examples, please refer to the [Rust Getting Started guide](./../getting_started/rust) to install
the library.

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

The wallet.rs library has numerous [examples](https://github.com/iotaledger/wallet.rs/tree/develop/wallet/examples)
you can run to get acquainted with the library. After you have followed the instructions to
[install the library](./../getting_started/rust#install-the-library), you can run any example with the following
command from the `examples` directory:

```bash
cargo run --example 0_generate_addresses --release
```

## Examples List

You can replace the `0_generate_addresses` by any other example from
the [Rust examples directory](https://github.com/iotaledger/wallet.rs/tree/develop/wallet/examples).

You can get a full list of examples by running the following command:

```bash
cargo run --example
```
