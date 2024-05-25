use clap::{Parser, Subcommand};

/// A fictional versioning CLI
#[derive(Debug, Parser)]
#[command(name = "dgit")]
#[command(about = "A fictional versioning CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = " Create an empty Git repository or reinitialize an existing one")]
    Init {
        #[arg(short, long)]
        directory: Option<String>,
    },

    #[command(about = "Provide contents or details of repository objects")]
    CatFile {
        /// Pretty-print the contents of <object> based on its type.
        #[arg(short, long)]
        pretty_print: String,
    },

    #[command(about = "Create a tree object from the current index")]
    WriteTree,

    #[command(about = "Create a commit object from the current index")]
    CommitTree {
        /// Tree SHA-1 hash to commit.
        tree_sha: String,
        /// The previous commit SHA-1 hash.
        #[arg(short, long)]
        parent: Vec<String>,
        /// The commit message.
        #[arg(short, long)]
        message: String,
    },

    #[command(about = "Compute object ID and optionally create an object from a file")]
    HashObject {
        /// Actually write the object into the object database.
        #[arg(short, long)]
        write: String,
    },

    #[command(about = "List the contents of a tree object")]
    LsTree {
        /// The object to list the contents of.
        path: String,
        /// List only filenames (instead of the "long" output), one per line. Cannot be combined with --object-only.
        #[arg(short, long)]
        name_only: bool,
    },
}
