initSidebarItems({"enum":[["MigrationProgressType","Migration event type."],["TransactionEventType",""],["TransferProgressType","Transfer event type."]],"fn":[["add_transaction_listener","Adds a transaction-related event listener."],["address_consolidation_needed_listeners","Gets the address consolodation needed listeners array."],["balance_listeners","Gets the balance change listeners array."],["emit_address_consolidation_needed","Emits a balance change event."],["emit_balance_change","Emits a balance change event."],["emit_confirmation_state_change","Emits a transaction confirmation state change event."],["emit_error",""],["emit_ledger_address_generation","Emits a balance change event."],["emit_migration_progress","Emit a migration event."],["emit_reattachment_event","Emits a transaction reattachment change event."],["emit_stronghold_status_change",""],["emit_transaction_event","Emits a transaction-related event."],["emit_transfer_progress","Emit a transfer event."],["error_listeners","Gets the balance change listeners array."],["generate_event_id",""],["generate_indexation_id",""],["ledger_address_generation_listeners","Gets address that will be generated with a prompt on the ledger."],["migration_progress_listeners",""],["on_address_consolidation_needed","Listen to `address consolidation needed` events."],["on_balance_change","Listen to balance changes."],["on_broadcast","Listen to transaction broadcast."],["on_confirmation_state_change","Listen to transaction confirmation state change."],["on_error","Listen to errors."],["on_ledger_address_generation","Listen to `ledger address generation` events."],["on_migration_progress","Listen to a migration event."],["on_new_transaction","Listen to new messages."],["on_reattachment","Listen to transaction reattachment change."],["on_stronghold_status_change","Listen to stronghold status change events."],["on_transfer_progress","Listen to a transfer event."],["remove_address_consolidation_needed_listener","Removes the balance change listener associated with the given identifier."],["remove_balance_change_listener","Removes the balance change listener associated with the given identifier."],["remove_broadcast_listener","Removes the broadcast listener associated with the given identifier."],["remove_confirmation_state_change_listener","Removes the new confirmation state change listener associated with the given identifier."],["remove_error_listener","Removes the error listener associated with the given identifier."],["remove_event_listener",""],["remove_ledger_address_generation_listener","Removes the balance change listener associated with the given identifier."],["remove_migration_progress_listener","Remove a migration event listener."],["remove_new_transaction_listener","Removes the new transaction listener associated with the given identifier."],["remove_reattachment_listener","Removes the reattachment listener associated with the given identifier."],["remove_stronghold_status_change_listener","Removes the stronghold status change listener associated with the given identifier."],["remove_transfer_progress_listener","Remove a transfer event listener."],["stronghold_status_change_listeners","Gets the stronghold status change listeners array."],["transaction_confirmation_change_listeners","Gets the transaction confirmation change listeners array."],["transaction_listeners","Gets the transaction listeners array."],["transaction_reattachment_listeners","Gets the transaction reattachment listeners array."],["transfer_progress_listeners",""]],"struct":[["AddressConsolidationNeeded","The `address consolidation needed` data."],["AddressConsolidationNeededHandler",""],["AddressData","Address data for TransferProgressType::GeneratingRemainderDepositAddress and LedgerAddressGeneration."],["BalanceChange","The balance change event payload."],["BalanceEvent","The balance change event data."],["BalanceEventHandler",""],["ErrorHandler",""],["LedgerAddressGeneration","Ledger generate address event data."],["LedgerAddressGenerationHandler",""],["MigrationProgress","Migration event data."],["MigrationProgressHandler",""],["PreparedTransactionData","Prepared transaction event data."],["StrongholdStatusChangeEventHandler",""],["TransactionConfirmationChangeEvent","A transaction confirmation state change event data."],["TransactionConfirmationChangeEventHandler",""],["TransactionEvent","A transaction-related event data."],["TransactionEventHandler",""],["TransactionIO","Input or output data for PreparedTransactionData"],["TransactionReattachmentEvent","Transaction reattachment event data."],["TransactionReattachmentEventHandler",""],["TransferProgress","Transfer event data."],["TransferProgressHandler",""]],"trait":[["EventHandler",""]],"type":[["AddressConsolidationNeededListeners",""],["BalanceListeners",""],["ErrorListeners",""],["EventId","The event identifier type."],["LedgerAddressGenerationListeners",""],["MigrationProgressListeners",""],["StrongholdStatusChangeListeners",""],["TransactionConfirmationChangeListeners",""],["TransactionListeners",""],["TransactionReattachmentListeners",""],["TransferProgressListeners",""]]});