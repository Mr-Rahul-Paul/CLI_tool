use clap::Parser;
use std::path::PathBuf;
#[derive(Parser)]
#[command(version = "1.0" , author = "Paul")]
struct Cli {
    pattern : String,  /// The pattern to search for
    path : PathBuf,
}
fn main(){

}
