use anyhow::{Context, Result};
use clap::Parser;
use std::{fs, path::PathBuf};

#[derive(Parser)]
#[command(version = "1.0", author = "Paul")]
struct Cli {
    /* this under can be used to describe when pattern when cr -- --help */
    /// The pattern to search for
    pattern: String,
    path: PathBuf,
}
fn main() -> Result<()> {
    let args = Cli::parse();

    let content = fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read file `{:?}`", args.path))?;

    println!("{}", content);

    Ok(())
    // dont use " Ok(()); "
}
