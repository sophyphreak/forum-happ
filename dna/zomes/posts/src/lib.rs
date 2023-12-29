use hdk::prelude::*;

/**
 * Add your edits to the bottom of this file
 */
pub use posts_zome;

#[hdk_entry_helper]
struct Post {
    title: String,
    content: String,
}

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
enum EntryTypes {
    Post(Post),
}

#[derive(Serialize, Deserialize, Debug)]
struct CreatePostInput {
    post: Post,
    channel: String,
}

#[hdk_link_types]
enum LinkTypes {
    PathToChannel,
    ChannelToPost,
}

#[hdk_extern]
fn create_post(create_post_input: CreatePostInput) -> ExternResult<ActionHash> {
    let post_action_hash = create_entry(EntryTypes::Post(create_post_input.post))?;

    let path = Path::from(format!("all_posts.{}", create_post_input.channel));

    let typed_path = path.typed(LinkTypes::PathToChannel)?;

    let _result = typed_path.ensure();

    create_link(
        typed_path.path_entry_hash()?,
        post_action_hash.clone(),
        LinkTypes::ChannelToPost,
        (),
    )?;

    Ok(post_action_hash)
}

#[hdk_extern]
fn get_channel_posts(channel: String) -> ExternResult<Vec<ActionHash>> {
    let path = Path::from(channel);

    let links: Vec<Link> = get_links(
        Path::path_entry_hash(&path)?,
        LinkTypes::ChannelToPost,
        None,
    )?;
    let action_hashes: Vec<ActionHash> = links
        .into_iter()
        .map(|link| ActionHash::from(link.target))
        .collect();
    Ok(action_hashes)
}

#[hdk_extern]
fn get_all_channels(_: ()) -> ExternResult<Vec<String>> {
    let all_posts_path = Path::from(format!("all_posts")).typed(LinkTypes::PathToChannel)?;

    let channel_paths: Vec<TypedPath> = all_posts_path.children_paths()?;

    let mut all_names: Vec<String> = vec![];
    for path in channel_paths {
        let last_component = path
            .leaf()
            .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
                "The path is empty"
            ))))?
            .clone();

        let channel = String::try_from(&last_component).map_err(|err| wasm_error!(err))?;

        all_names.push(channel);
    }
    Ok(all_names)
}
