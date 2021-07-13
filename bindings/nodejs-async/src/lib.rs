#![allow(clippy::unnecessary_wraps)]

mod classes;
use classes::*;
pub mod types;

use neon::prelude::*;
use std::{
    any::Any,
    collections::HashMap,
    panic::AssertUnwindSafe,
    sync::{Arc, Mutex},
};
use once_cell::sync::Lazy;
use tokio::{runtime::Runtime, sync::RwLock};
use serde::Deserialize;

use std::{path::PathBuf, time::Duration};

pub use iota_wallet::{
    account_manager::{AccountManager, DEFAULT_STORAGE_FOLDER},
    account::{AccountHandle, SyncedAccount},
    actor::{Message as WalletMessage, MessageType, Response, ResponseType, WalletMessageHandler, AccountIdentifier},
    address::parse as parse_address,
    event::{
        on_balance_change, on_broadcast, on_confirmation_state_change, on_error, on_migration_progress,
        on_new_transaction, on_reattachment, on_stronghold_status_change, on_transfer_progress,
        remove_balance_change_listener, remove_broadcast_listener, remove_confirmation_state_change_listener,
        remove_error_listener, remove_migration_progress_listener, remove_new_transaction_listener,
        remove_reattachment_listener, remove_stronghold_status_change_listener, remove_transfer_progress_listener,
        EventId,
    },
    message::{IndexationPayload, MessageId, RemainderValueStrategy, Transfer, TransferOutput},
    Error,
};








// use futures::{Future, FutureExt};
// use iota_client::common::logger::{logger_init, LoggerConfigBuilder};
// use neon::prelude::*;
// use once_cell::sync::{Lazy, OnceCell};
use rand::{distributions::Alphanumeric, thread_rng, Rng};

// mod classes;
// use classes::*;
// pub(crate) mod types;

type AccountInstanceMap = Arc<RwLock<HashMap<String, AccountHandle>>>;
type SyncedAccountHandle = Arc<RwLock<SyncedAccount>>;
type SyncedAccountInstanceMap = Arc<RwLock<HashMap<String, SyncedAccountHandle>>>;

/// Gets the account instances map.
fn account_instances() -> &'static AccountInstanceMap {
    static INSTANCES: Lazy<AccountInstanceMap> = Lazy::new(Default::default);
    &INSTANCES
}

pub(crate) async fn get_account(id: &str) -> AccountHandle {
    account_instances()
        .read()
        .await
        .get(id)
        .expect("account dropped or not initialised")
        .clone()
}

pub(crate) async fn store_account(account_handle: AccountHandle) -> String {
    let handle = account_handle.clone();
    let id = handle.id().await;

    account_instances().write().await.insert(id.clone(), account_handle);

    id
}

/// Gets the synced account instances map.
fn synced_account_instances() -> &'static SyncedAccountInstanceMap {
    static INSTANCES: Lazy<SyncedAccountInstanceMap> = Lazy::new(Default::default);
    &INSTANCES
}

#[allow(dead_code)]
pub(crate) async fn get_synced_account(id: &str) -> SyncedAccountHandle {
    synced_account_instances()
        .read()
        .await
        .get(id)
        .expect("synced account dropped or not initialised")
        .clone()
}

pub(crate) async fn store_synced_account(synced_account: SyncedAccount) -> String {
    let mut map = synced_account_instances().write().await;
    let id: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .map(char::from)
        .take(10)
        .collect();
    map.insert(id.clone(), Arc::new(RwLock::new(synced_account)));
    id
}

pub(crate) async fn remove_synced_account(id: &str) {
    synced_account_instances().write().await.remove(id);
}

fn panic_to_response_message(panic: Box<dyn Any>) -> String {
    let msg = if let Some(message) = panic.downcast_ref::<String>() {
        format!("Internal error: {}", message)
    } else if let Some(message) = panic.downcast_ref::<&str>() {
        format!("Internal error: {}", message)
    } else {
        "Internal error".to_string()
    };
    let current_backtrace = backtrace::Backtrace::new();
    format!("{}\n\n{:?}", msg, current_backtrace)
}

// pub async fn convert_async_panics<T, F: Future<Output = Result<T, Error>>>(f: impl FnOnce() -> F) -> Result<T, Error> {
//     match AssertUnwindSafe(f()).catch_unwind().await {
//         Ok(result) => result,
//         Err(panic) => Err(Error::Panic(panic_to_response_message(panic))),
//     }
// }



pub static RUNTIME: Lazy<Runtime> = Lazy::new(|| Runtime::new().unwrap());

use iota_client::common::logger::{logger_init, LoggerConfigBuilder};

pub fn init_logger(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let config = cx.argument::<JsString>(0)?.value(&mut cx);
    let config: LoggerConfigBuilder = serde_json::from_str(&config).expect("invalid logger config");
    logger_init(config.finish()).expect("failed to init logger");
    Ok(cx.undefined())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {

    // Account methods.
    cx.export_function("accountNew", classes::account::account_new)?;
    cx.export_function("id", classes::account::id)?;
    cx.export_function("index", classes::account::index)?;
    cx.export_function("alias", classes::account::alias)?;
    cx.export_function("message_count", classes::account::message_count)?;
    cx.export_function("sync", classes::account::sync)?;

    // Account manager methods.
    cx.export_function("accountManagerNew", classes::account_manager::account_manager_new)?;
    cx.export_function("getAccount", classes::account_manager::get_account)?;
    cx.export_function("getAccounts", classes::account_manager::get_accounts)?;
    cx.export_function("createAccount", classes::account_manager::create_account)?;
    cx.export_function("setStrongholdPassword", classes::account_manager::set_stronghold_password)?;
    cx.export_function("storeMnemonic", classes::account_manager::store_mnemonic)?;
    cx.export_function("backup", classes::account_manager::backup)?;
    cx.export_function("importAccounts", classes::account_manager::import_accounts)?;
    cx.export_function("setStoragePassword", classes::account_manager::set_storage_password)?;
    cx.export_function("changeStrongholdPassword", classes::account_manager::change_stronghold_password)?;
    cx.export_function("generateMnemonic", classes::account_manager::generate_mnemonic)?;


    // Message handler methods.
    cx.export_function("sendMessage", classes::message_handler::send_message)?;
    cx.export_function("messageHandlerNew", classes::message_handler::message_handler_new)?;

    cx.export_function("eventListenerNew", classes::event_listener::event_listener_new)?;
    cx.export_function("initLogger", init_logger)?;
    cx.export_function("listen", classes::event_listener::listen)?;
    cx.export_function("removeEventListeners", classes::event_listener::remove_event_listeners)?;
    Ok(())
}
