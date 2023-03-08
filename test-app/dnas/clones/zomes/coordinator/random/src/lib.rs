pub mod message;
use hdk::prelude::*;
use random_integrity::*;
#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
    Ok(InitCallbackResult::Pass)
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Signal {
    LinkCreated { action: SignedActionHashed, link_type: LinkTypes },
    LinkDeleted { action: SignedActionHashed, link_type: LinkTypes },
    EntryCreated { action: SignedActionHashed, app_entry: EntryTypes },
    EntryUpdated {
        action: SignedActionHashed,
        app_entry: EntryTypes,
        original_app_entry: EntryTypes,
    },
    EntryDeleted { action: SignedActionHashed, original_app_entry: EntryTypes },
}
#[hdk_extern(infallible)]
pub fn post_commit(committed_actions: Vec<SignedActionHashed>) {
    for action in committed_actions {
        if let Err(err) = signal_action(action) {
            error!("Error signaling new action: {:?}", err);
        }
    }
}
fn signal_action(action: SignedActionHashed) -> ExternResult<()> {
    match action.hashed.content.clone() {
        Action::CreateLink(create_link) => {
            let link_type = LinkTypes::from_type(
                    create_link.zome_index,
                    create_link.link_type,
                )?
                .ok_or(
                    wasm_error!(
                        WasmErrorInner::Guest("Link type should be exist".to_string())
                    ),
                )?;
            emit_signal(Signal::LinkCreated {
                action,
                link_type,
            })?;
            Ok(())
        }
        Action::DeleteLink(delete_link) => {
            let record = get(
                    delete_link.link_add_address.clone(),
                    GetOptions::default(),
                )?
                .ok_or(
                    wasm_error!(
                        WasmErrorInner::Guest("Create Link should exist".to_string())
                    ),
                )?;
            match record.action() {
                Action::CreateLink(create_link) => {
                    let link_type = LinkTypes::from_type(
                            create_link.zome_index,
                            create_link.link_type,
                        )?
                        .ok_or(
                            wasm_error!(
                                WasmErrorInner::Guest("Link type should be exist"
                                .to_string())
                            ),
                        )?;
                    emit_signal(Signal::LinkDeleted {
                        action,
                        link_type,
                    })?;
                    Ok(())
                }
                _ => {
                    return Err(
                        wasm_error!(
                            WasmErrorInner::Guest("Create Link should exist".to_string())
                        ),
                    );
                }
            }
        }
        Action::Create(_create) => {
            let app_entry = get_entry_for_action(&action.hashed.hash)?
                .ok_or(
                    wasm_error!(
                        WasmErrorInner::Guest("Create should carry an entry".to_string())
                    ),
                )?;
            emit_signal(Signal::EntryCreated {
                action,
                app_entry,
            })?;
            Ok(())
        }
        Action::Update(update) => {
            let app_entry = get_entry_for_action(&action.hashed.hash)?
                .ok_or(
                    wasm_error!(
                        WasmErrorInner::Guest("Update should carry an entry".to_string())
                    ),
                )?;
            let original_app_entry = get_entry_for_action(
                    &update.original_action_address,
                )?
                .ok_or(
                    wasm_error!(
                        WasmErrorInner::Guest("Updated action should carry an entry"
                        .to_string())
                    ),
                )?;
            emit_signal(Signal::EntryUpdated {
                action,
                app_entry,
                original_app_entry,
            })?;
            Ok(())
        }
        Action::Delete(delete) => {
            let original_app_entry = get_entry_for_action(&delete.deletes_address)?
                .ok_or(
                    wasm_error!(
                        WasmErrorInner::Guest("Deleted action should carry an entry"
                        .to_string())
                    ),
                )?;
            emit_signal(Signal::EntryDeleted {
                action,
                original_app_entry,
            })?;
            Ok(())
        }
        _ => Ok(()),
    }
}
fn get_entry_for_action(action_hash: &ActionHash) -> ExternResult<Option<EntryTypes>> {
    let record = match get_details(action_hash.clone(), GetOptions::default())? {
        Some(Details::Record(record_details)) => record_details.record,
        _ => {
            return Ok(None);
        }
    };
    let entry = match record.entry().as_option() {
        Some(entry) => entry,
        None => {
            return Ok(None);
        }
    };
    let (zome_index, entry_index) = match record.action().entry_type() {
        Some(EntryType::App(AppEntryDef { zome_index, entry_index, .. })) => {
            (zome_index, entry_index)
        }
        _ => {
            return Ok(None);
        }
    };
    Ok(
        EntryTypes::deserialize_from_type(
            zome_index.clone(),
            entry_index.clone(),
            entry,
        )?,
    )
}
