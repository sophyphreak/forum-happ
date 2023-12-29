use hdk::prelude::*;

/**
 * Add your edits to the bottom of this file
 */
pub use comments_zome;

#[hdk_entry_helper]
struct Comment {
    comment: String,
}

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
enum EntryTypes {
    Comment(Comment),
}

#[derive(Serialize, Deserialize, Debug)]
struct CreateCommentInput {
    comment_on: ActionHash,
    comment: String,
}

#[hdk_link_types]
enum LinkTypes {
    CommentedOnToComment,
}

#[hdk_extern]
fn create_comment(create_comment_input: CreateCommentInput) -> ExternResult<ActionHash> {
    let comment_action_hash = create_entry(EntryTypes::Comment(Comment {
        comment: create_comment_input.comment,
    }))?;

    create_link(
        create_comment_input.comment_on,
        comment_action_hash.clone(),
        LinkTypes::CommentedOnToComment,
        (),
    )?;

    Ok(comment_action_hash)
}

#[hdk_extern]
fn get_comments_on(post_action_hash: ActionHash) -> ExternResult<Vec<Record>> {
    let links: Vec<Link> = get_links(post_action_hash, LinkTypes::CommentedOnToComment, None)?;
    let mut comments: Vec<Record> = vec![];
    for link in links {
        if let Some(record) = get(ActionHash::from(link.target), GetOptions::default())? {
            comments.push(record);
        }
    }
    Ok(comments)
}

#[hdk_extern]
fn delete_comment(comment_action_hash: ActionHash) -> ExternResult<()> {
    let _action_hash = delete_entry(comment_action_hash);
    Ok(())
}
