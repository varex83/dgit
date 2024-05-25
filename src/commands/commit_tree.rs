use crate::objects::commit::{CommitAuthor, CommitContent, CommitObject};
use crate::traits::{Hash, ObjectSave};

pub fn commit_tree(tree_hash: String, parent: Vec<String>, message: String) -> anyhow::Result<()> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs() as i64;

    let commit_object = CommitObject::new(CommitContent {
        tree_sha: tree_hash,
        parent_sha: parent,
        author: CommitAuthor {
            name: "Varex".to_string(),
            email: "example@exampe.com".to_string(),
            timestamp,
            timezone: "+0000".to_string(),
        },
        committer: CommitAuthor {
            name: "Varex".to_string(),
            email: "example@exampe.com".to_string(),
            timestamp,
            timezone: "+0000".to_string(),
        },
        message,
    });

    commit_object.save_object()?;

    println!("{}", commit_object.hash());

    Ok(())
}
