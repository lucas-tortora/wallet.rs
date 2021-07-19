// // Copyright 2020 IOTA Stiftung
// // SPDX-License-Identifier: Apache-2.0

// use std::{
//     convert::TryFrom,
//     sync::{
//         mpsc::{channel, Receiver, Sender},
//         Arc, Mutex,
//     },
// };

// use iota_wallet::event::{
//     on_balance_change, on_broadcast, on_confirmation_state_change, on_error, on_migration_progress,
// on_new_transaction,     on_reattachment, on_transfer_progress, EventId,
// };
// use neon::prelude::*;

// pub enum EventType {
//     ErrorThrown,
//     BalanceChange,
//     NewTransaction,
//     ConfirmationStateChange,
//     Reattachment,
//     Broadcast,
//     TransferProgress,
//     MigrationProgress,
// }

// impl TryFrom<&str> for EventType {
//     type Error = String;

//     fn try_from(value: &str) -> Result<Self, Self::Error> {
//         let event_type = match value {
//             "ErrorThrown" => EventType::ErrorThrown,
//             "BalanceChange" => EventType::BalanceChange,
//             "NewTransaction" => EventType::NewTransaction,
//             "ConfirmationStateChange" => EventType::ConfirmationStateChange,
//             "Reattachment" => EventType::Reattachment,
//             "Broadcast" => EventType::Broadcast,
//             "TransferProgress" => EventType::TransferProgress,
//             "MigrationProgress" => EventType::MigrationProgress,
//             _ => return Err(format!("invalid event name {}", value)),
//         };
//         Ok(event_type)
//     }
// }

// async fn listen(event_type: EventType, sender: Sender<String>) -> EventId {
//     match event_type {
//         EventType::ErrorThrown => on_error(move |error| {
//             let _ = sender.send(serde_json::to_string(&error).unwrap());
//         }),
//         EventType::BalanceChange => {
//             on_balance_change(move |event| {
//                 let _ = sender.send(serde_json::to_string(&event).unwrap());
//             })
//             .await
//         }
//         EventType::NewTransaction => {
//             on_new_transaction(move |event| {
//                 let _ = sender.send(serde_json::to_string(&event).unwrap());
//             })
//             .await
//         }
//         EventType::ConfirmationStateChange => {
//             on_confirmation_state_change(move |event| {
//                 let _ = sender.send(serde_json::to_string(&event).unwrap());
//             })
//             .await
//         }
//         EventType::Reattachment => {
//             on_reattachment(move |event| {
//                 let _ = sender.send(serde_json::to_string(&event).unwrap());
//             })
//             .await
//         }
//         EventType::Broadcast => {
//             on_broadcast(move |event| {
//                 let _ = sender.send(serde_json::to_string(&event).unwrap());
//             })
//             .await
//         }
//         EventType::TransferProgress => {
//             on_transfer_progress(move |event| {
//                 let _ = sender.send(serde_json::to_string(&event).unwrap());
//             })
//             .await
//         }
//         EventType::MigrationProgress => {
//             on_migration_progress(move |event| {
//                 let _ = sender.send(serde_json::to_string(&event).unwrap());
//             })
//             .await
//         }
//     }
// }

// struct WaitForEventTask(Arc<Mutex<Receiver<String>>>);

// impl Task for WaitForEventTask {
//     type Output = String;
//     type Error = String;
//     type JsEvent = JsString;

//     fn perform(&self) -> Result<Self::Output, Self::Error> {
//         let rx = self
//             .0
//             .lock()
//             .map_err(|_| "Could not obtain lock on receiver".to_string())?;
//         rx.recv().map_err(|e| e.to_string())
//     }

//     fn complete(self, mut cx: TaskContext, result: Result<Self::Output, Self::Error>) -> JsResult<Self::JsEvent> {
//         match result {
//             Ok(s) => Ok(cx.string(s)),
//             Err(e) => cx.throw_error(format!("ReceiveTask error: {}", e)),
//         }
//     }
// }

// pub struct EventListener {
//     rx: Arc<Mutex<Receiver<String>>>,
// }

// declare_types! {
//     pub class JsEventListener for EventListener {
//         init(mut cx) {
//             let event = EventType::try_from(cx.argument::<JsString>(0)?.value().as_str()).expect("invalid event
// type");             let (tx, rx) = channel();

//             crate::block_on(listen(event, tx));

//             Ok(EventListener {
//                 rx: Arc::new(Mutex::new(rx)),
//             })
//         }

//         method poll(mut cx) {
//             let cb = cx.argument::<JsFunction>(0)?;
//             let this = cx.this();

//             let rx = cx.borrow(&this, |listener| Arc::clone(&listener.rx));
//             let receive_task = WaitForEventTask(rx);

//             receive_task.schedule(cb);

//             Ok(JsUndefined::new().upcast())
//         }
//     }
// }

// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use neon::prelude::*;
use std::{
    convert::TryInto,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};

use std::{
    collections::{hash_map::Entry, HashMap},
    convert::TryFrom,
};

pub use iota_wallet::{
    account_manager::{AccountManager, DEFAULT_STORAGE_FOLDER},
    actor::{Message as WalletMessage, MessageType, Response, ResponseType, WalletMessageHandler},
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

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
pub enum EventType {
    ErrorThrown,
    BalanceChange,
    NewTransaction,
    ConfirmationStateChange,
    Reattachment,
    Broadcast,
    StrongholdStatusChange,
    TransferProgress,
    MigrationProgress,
}

impl TryFrom<&str> for EventType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let event_type = match value {
            "ErrorThrown" => EventType::ErrorThrown,
            "BalanceChange" => EventType::BalanceChange,
            "NewTransaction" => EventType::NewTransaction,
            "ConfirmationStateChange" => EventType::ConfirmationStateChange,
            "Reattachment" => EventType::Reattachment,
            "Broadcast" => EventType::Broadcast,
            "StrongholdStatusChange" => EventType::StrongholdStatusChange,
            "TransferProgress" => EventType::TransferProgress,
            "MigrationProgress" => EventType::MigrationProgress,
            _ => return Err(format!("invalid event name {}", value)),
        };
        Ok(event_type)
    }
}

type JsCallback = Root<JsFunction<JsObject>>;

pub(crate) struct EventListener {
    queue: EventQueue,
    callbacks: Arc<Mutex<HashMap<EventType, Vec<(JsCallback, EventId)>>>>,
}
impl Finalize for EventListener {}

impl EventListener {
    fn new(queue: EventQueue) -> Arc<Self> {
        Arc::new(Self {
            queue,
            callbacks: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    fn call(&self, message: String, event_type: EventType) {
        let callbacks = self.callbacks.clone();
        self.queue.send(move |mut cx| {
            if let Some(cbs) = callbacks.lock().unwrap().get(&event_type) {
                for (cb, _) in cbs {
                    let cb = cb.to_inner(&mut cx);
                    let this = cx.undefined();
                    let args = vec![
                        cx.undefined().upcast::<JsValue>(),
                        cx.string(message.clone()).upcast::<JsValue>(),
                    ];

                    cb.call(&mut cx, this, args)?;
                }
            };

            Ok(())
        });
    }

    async fn remove_event_listeners(event_type: EventType, event_id: &[u8; 32]) {
        match event_type {
            EventType::ErrorThrown => remove_error_listener(&event_id),
            EventType::BalanceChange => remove_balance_change_listener(&event_id).await,
            EventType::NewTransaction => remove_new_transaction_listener(&event_id).await,
            EventType::ConfirmationStateChange => remove_confirmation_state_change_listener(&event_id).await,
            EventType::Reattachment => remove_reattachment_listener(&event_id).await,
            EventType::Broadcast => remove_broadcast_listener(&event_id).await,
            EventType::StrongholdStatusChange => remove_stronghold_status_change_listener(&event_id).await,
            EventType::TransferProgress => remove_transfer_progress_listener(&event_id).await,
            EventType::MigrationProgress => remove_migration_progress_listener(&event_id).await,
        };
    }

    pub async fn add_event_listener<F: Fn(String, EventType) + Send + 'static>(
        event_type: EventType,
        callback: F,
    ) -> [u8; 32] {
        match event_type {
            EventType::ErrorThrown => on_error(move |error| {
                let _ = callback(serde_json::to_string(&error).unwrap(), event_type);
            }),
            EventType::BalanceChange => {
                on_balance_change(move |event| {
                    let _ = callback(serde_json::to_string(&event).unwrap(), event_type);
                })
                .await
            }
            EventType::NewTransaction => {
                on_new_transaction(move |event| {
                    let _ = callback(serde_json::to_string(&event).unwrap(), event_type);
                })
                .await
            }
            EventType::ConfirmationStateChange => {
                on_confirmation_state_change(move |event| {
                    let _ = callback(serde_json::to_string(&event).unwrap(), event_type);
                })
                .await
            }
            EventType::Reattachment => {
                on_reattachment(move |event| {
                    let _ = callback(serde_json::to_string(&event).unwrap(), event_type);
                })
                .await
            }
            EventType::Broadcast => {
                on_broadcast(move |event| {
                    let _ = callback(serde_json::to_string(&event).unwrap(), event_type);
                })
                .await
            }
            EventType::StrongholdStatusChange => {
                on_stronghold_status_change(move |event| {
                    let _ = callback(serde_json::to_string(&event).unwrap(), event_type);
                })
                .await
            }
            EventType::TransferProgress => {
                on_transfer_progress(move |event| {
                    let _ = callback(serde_json::to_string(&event).unwrap(), event_type);
                })
                .await
            }
            EventType::MigrationProgress => {
                on_migration_progress(move |event| {
                    let _ = callback(serde_json::to_string(&event).unwrap(), event_type);
                })
                .await
            }
        }
    }
}

pub(crate) fn remove_event_listeners(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let event_name = cx.argument::<JsString>(0)?.value(&mut cx);
    let event_type: EventType = event_name.as_str().try_into().expect("unknown event name");
    let event_handler = Arc::clone(&&cx.argument::<JsBox<Arc<EventListener>>>(1)?);
    let callback = cx.argument::<JsFunction>(2)?.root(&mut cx);

    crate::RUNTIME.spawn(async move {
        let cbs;
        {
            cbs = event_handler.callbacks.lock().unwrap().remove(&event_type);
        }

        if let Some(ref cbs) = cbs {
            for (_, event_id) in cbs {
                EventListener::remove_event_listeners(event_type, event_id).await;
            }
        }

        event_handler.queue.send(move |mut cx| {
            if let Some(cbs) = cbs {
                for (cb, _) in cbs {
                    cb.drop(&mut cx);
                }
            }

            let cb = callback.into_inner(&mut cx);
            let this = cx.undefined();
            let args = vec![cx.undefined().upcast::<JsValue>()];

            cb.call(&mut cx, this, args)?;

            Ok(())
        });
    });

    Ok(cx.undefined())
}

pub(crate) fn listen(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let event_name = cx.argument::<JsString>(0)?.value(&mut cx);
    let event_type: EventType = event_name.as_str().try_into().expect("unknown event name");
    let event_handler = Arc::clone(&&cx.argument::<JsBox<Arc<EventListener>>>(1)?);
    let callback = cx.argument::<JsFunction>(2)?.root(&mut cx);

    crate::RUNTIME.spawn(async move {
        let cloned_eh = event_handler.clone();
        let event_id = EventListener::add_event_listener(event_type, move |message: String, event_type: EventType| {
            log::debug!("--------------------------------------------------------------------------------------------");
            log::debug!(
                "--------------------------------------------{:?}------------------------------------------------",
                event_type
            );
            cloned_eh.call(message, event_type);
        })
        .await;

        match event_handler.callbacks.lock().unwrap().entry(event_type) {
            Entry::Vacant(e) => {
                e.insert(vec![(callback, event_id)]);
            }
            Entry::Occupied(mut e) => {
                e.get_mut().push((callback, event_id));
            }
        }
    });

    Ok(cx.undefined())
}

pub(crate) fn event_listener_new(mut cx: FunctionContext) -> JsResult<JsBox<Arc<EventListener>>> {
    let queue = cx.queue();
    let event_handler = EventListener::new(queue);

    Ok(cx.boxed(event_handler))
}
