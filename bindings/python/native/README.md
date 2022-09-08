# IOTA Wallet Library - Python binding

Python binding to the iota.rs wallet library.

## Requirements

Python 3

pip>=19.1
setuptools-rust>=0.10.2

`Rust` and `Cargo`, to compile the binding. Install them [here](https://doc.rust-lang.org/cargo/getting-started/installation.html).

## Installation

### Build the wheel file
- Go to `iota.rs/bindings/python/native`
- `python3 setup.py bdist_wheel`

### Create a virtual environment and use it (optional)
- `python3 -m venv .venv`
- `source .venv/bin/activate`

### Install the wheel file
`python3 -m pip install dist/[your built wheel file]`

Example:
- `python3 -m pip install dist/iota_wallet-0.2.0-cp310-cp310-linux_x86_64.whl`

### Run examples
`python3 example/[example file]`

Example: 
- `python3 examples/0-create-account.py`

### To deactivate the virtual environment (optional)
- `deactivate`

## Getting Started

After you installed the library, you can create a `IotaWallet` instance and interact with it.

```python
from iota_wallet import IotaWallet, StrongholdSecretManager

# This example creates a new database and account

wallet_options = {
    'nodes': ['https://api.testnet.shimmer.network'],
}

# Shimmer coin type
coin_type = 4219

secret_manager = StrongholdSecretManager("wallet.stronghold", "some_hopefully_secure_password")

wallet = IotaWallet('./alice-database', wallet_options, coin_type, secret_manager)

# Store the mnemonic in the Stronghold snapshot, this only needs to be done once
account = wallet.store_mnemonic("flame fever pig forward exact dash body idea link scrub tennis minute " +
          "surge unaware prosper over waste kitten ceiling human knife arch situate civil")

account = wallet.create_account('Alice')
print(account)

```

## Build docs
`pydoc-markdown -p iota_wallet > ../../../documentation/docs/references/python_api_reference.md`
