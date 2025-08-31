use anyhow::{Context, Result};
use clap::Parser;
use owo_colors::OwoColorize;
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

    find_matches(&content, &args.pattern);

    Ok(())
    // dont use " Ok(()); "
}

fn find_matches(content: &str, pattern: &str) {
    for line in content.lines() {
        if line.contains(pattern) {
            //color the output
            // Split the line by the pattern
            let parts: Vec<&str> = line.split(pattern).collect();
            
          
            let colored = parts.join(&pattern.red().bold().to_string());

            println!("{}", colored);
        }
    }
}
