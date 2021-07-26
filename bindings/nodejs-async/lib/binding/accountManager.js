// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const addon = require('../../index.node');
const utils = require('../utils.js');
const acc = require('./account.js');

let {
    accountManagerNew,
    getAccount,
    getAccounts,
    createAccount,
    setStrongholdPassword,
    storeMnemonic,
    backup,
    importAccounts,
    setStoragePassword,
    changeStrongholdPassword,
    generateMnemonic,
    id
} = addon;
let { Account } = acc;
class AccountManager {
    constructor(options) {
        console.log("AccountManager constructor called.");
        console.log(options)
        console.log(JSON.stringify(options));
        this.accountManager = accountManagerNew(JSON.stringify(options));
        console.log(this.accountManager);
    }
    getAccount(accountId) {
        let inner_account = getAccount.apply(this.accountManager, [accountId]);
        return new Account(inner_account);
    }

    getAccounts() {
        let inner_accounts = getAccounts.apply(this.accountManager);
        return inner_accounts.map(a => new Account(a));
    }

    createAccount(account) {
        let acc = createAccount.apply(this.accountManager, [JSON.stringify(account)]);
        // console.log(acc);
        return new Account(acc);
    }

    setStrongholdPassword(password) {
        return setStrongholdPassword.apply(this.accountManager, [password]);
    }

    storeMnemonic(signerType, mnemonic) {
        console.log(signerType, mnemonic);

        return storeMnemonic.apply(this.accountManager, [signerType, mnemonic].filter(e => e != undefined));
    }

    backup(destination, password) {
        return backup.apply(this.accountManager, [destination, password]);
    }

    importAccounts(backupPath, password) {
        return importAccounts.apply(this.accountManager, [backupPath, password]);
    }
    setStoragePassword(password) {
        return setStoragePassword.apply(this.accountManager, [password]);
    }

    changeStrongholdPassword(currentPassword, oldPassword) {
        return changeStrongholdPassword.apply(this.accountManager, [currentPassword, oldPassword]);
    }

    generateMnemonic() {
        return generateMnemonic.apply(this.accountManager);
    }
};

module.exports.AccountManager = AccountManager;