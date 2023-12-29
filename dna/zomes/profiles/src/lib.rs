use hdk::prelude::*;

/**
 * Add your edits to the bottom of this file
 */
pub use profiles_zome;

#[hdk_entry_helper]
struct Profile {
    nickname: String,
}

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
enum EntryTypes {
    Profile(Profile),
}

#[hdk_link_types]
enum LinkTypes {
    AgentToProfile,
}

#[hdk_extern]
fn create_profile(profile: Profile) -> ExternResult<ActionHash> {
    let profile_action_hash = create_entry(EntryTypes::Profile(profile))?;

    let my_pub_key: AgentPubKey = agent_info()?.agent_initial_pubkey;

    let create_link_action_hash: ActionHash = create_link(
        my_pub_key,
        profile_action_hash,
        LinkTypes::AgentToProfile,
        (),
    )?;

    Ok(create_link_action_hash)
}

#[hdk_extern]
fn get_agent_profile(pub_key: AgentPubKey) -> ExternResult<Option<Profile>> {
    let links: Vec<Link> = get_links(pub_key, LinkTypes::AgentToProfile, None)?;
    match links.first() {
        None => Ok(None),
        Some(link) => {
            if let Some(record) = get(ActionHash::from(link.target.clone()), GetOptions::default())?
            {
                let profile: Profile = record
                    .entry()
                    .to_app_option::<Profile>()
                    .map_err(|err: SerializedBytesError| wasm_error!(err))?
                    .ok_or(wasm_error!(WasmErrorInner::Guest(
                        "Could not deserialize element to Profile".into(),
                    )))?;
                Ok(Some(profile))
            } else {
                Ok(None)
            }
        }
    }
}

#[hdk_extern]
fn get_my_profile(_: ()) -> ExternResult<Option<Profile>> {
    let my_pub_key: AgentPubKey = agent_info()?.agent_initial_pubkey;
    get_agent_profile(my_pub_key)
}
