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

    #[arg(short = 'i', long = "ignore-case", default_value = "false")]
    ignore_case: bool,
}
fn main() -> Result<()> {
    let args = Cli::parse();

    let content = fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read file `{:?}`", args.path))?;

    find_matches(&content, &args.pattern, args.ignore_case);

    Ok(())
    // dont use " Ok(()); "
}

fn find_matches(content: &str, pattern: &str, ignore_case: bool) {
    if ignore_case {
        let lower_pattern = pattern.to_lowercase();
        for line in content.lines() {
            // The .find() method returns an Option<usize> containing the byte index where the pattern begins.
            if let Some(start_index) = line.to_lowercase().find(&lower_pattern) {
                let end_index = start_index + pattern.len();
                let before = &line[..start_index];
                let matched_part = &line[start_index..end_index];
                let after = &line[end_index..];
                println!("{}{}{}", before, matched_part.red().bold(), after);
            }
        }
    } else {
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
}
