use clap::Parser;
use dgit::cli::{Cli, Commands};
use dgit::commands;
#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;

fn main() {
    let Cli { command } = Cli::parse();

    match command {
        Commands::Init { .. } => commands::init(),
        Commands::CatFile { pretty_print } => {
            commands::cat_file(pretty_print.as_str()).expect("Failed to cat file")
        }
        Commands::HashObject { write } => {
            commands::hash_object(write.as_str()).expect("Failed to hash object")
        }
        Commands::LsTree { path, name_only } => {
            commands::ls_tree(path.as_str(), name_only).expect("Failed to ls tree")
        }
        Commands::WriteTree => commands::write_tree(".").expect("Failed to write tree"),
        Commands::CommitTree {
            tree_sha,
            parent,
            message,
        } => commands::commit_tree(tree_sha, parent, message).expect("Failed to commit tree"),
        #[allow(unreachable_patterns)]
        _ => println!("Not implemented yet"),
    }
}
