// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const addon = require('../../index.node');

let {  } = addon;

class SyncedAccount {
    constructor(id) {
      console.log("Synced account constructor called.");
      this.id = id;
    }

  };

module.exports.SyncedAccount = SyncedAccount;
