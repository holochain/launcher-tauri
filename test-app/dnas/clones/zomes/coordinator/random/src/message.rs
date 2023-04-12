use hdk::prelude::*;
use random_integrity::*;
#[hdk_extern]
pub fn create_message(message: Message) -> ExternResult<Record> {
    let message_hash = create_entry(&EntryTypes::Message(message.clone()))?;
    let record = get(message_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created Message"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_message(original_message_hash: ActionHash) -> ExternResult<Option<Record>> {
    let links = get_links(
        original_message_hash.clone(),
        LinkTypes::MessageUpdates,
        None,
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_b.timestamp.cmp(&link_a.timestamp));
    let latest_message_hash = match latest_link {
        Some(link) => match link.target.clone().into_action_hash() {
            Some(a) => a,
            None => return Err(
                wasm_error!(
                    WasmErrorInner::Guest(String::from("Link target is not an action hash."))
                ),)
        },
        None => original_message_hash.clone(),
    };
    get(latest_message_hash, GetOptions::default())
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateMessageInput {
    pub original_message_hash: ActionHash,
    pub previous_message_hash: ActionHash,
    pub updated_message: Message,
}
#[hdk_extern]
pub fn update_message(input: UpdateMessageInput) -> ExternResult<Record> {
    let updated_message_hash = update_entry(
        input.previous_message_hash.clone(),
        &input.updated_message,
    )?;
    create_link(
        input.original_message_hash.clone(),
        updated_message_hash.clone(),
        LinkTypes::MessageUpdates,
        (),
    )?;
    let record = get(updated_message_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated Message"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn delete_message(original_message_hash: ActionHash) -> ExternResult<ActionHash> {
    delete_entry(original_message_hash)
}
