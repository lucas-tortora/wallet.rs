// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const addon = require('../index.node');
const mh = require("./messageHandler.js");
const el = require("./eventListener.js");
const am = require("./accountManager.js");

let { initLogger } = addon;
let { MessageHandler } = mh;
let { EventListener } = el;
let { AccountManager } = am;

initLogger(JSON.stringify({
  color_enabled: true,
  outputs: [{
    name: 'wallet.log',
    level_filter: 'debug'
  }]
}));

module.exports = {
  MessageHandler,
  EventListener,
  AccountManager,
  initLogger
};

