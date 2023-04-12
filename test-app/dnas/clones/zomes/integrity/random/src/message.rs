use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone)]
pub struct Message {
    pub message: String,
}
pub fn validate_create_message(
    _action: EntryCreationAction,
    _message: Message,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_message(
    _action: Update,
    _message: Message,
    _original_action: EntryCreationAction,
    _original_message: Message,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_message(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_message: Message,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_create_link_message_updates(
    _action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // Check the entry type for the given action hash
    let action_hash = match base_address.into_action_hash() {
        Some(a) => a,
        None => return Err(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Base address is not an action hash."))
            ),)
    };
    let record = must_get_valid_record(action_hash)?;
    let _message: crate::Message = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    // Check the entry type for the given action hash
    let action_hash = match target_address.into_action_hash() {
        Some(a) => a,
        None => return Err(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Target address is not an action hash."))
            ),)
    };
    let record = must_get_valid_record(action_hash)?;
    let _message: crate::Message = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_message_updates(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("MessageUpdates links cannot be deleted"),
        ),
    )
}
