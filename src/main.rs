use clap::Parser;
use owo_colors::OwoColorize;
use std::{
    fs::{self},
    // fs::{self, File},
    path::{Path, PathBuf},
};
use strum::Display;

#[derive(Debug, Parser)]
#[command(/*-V*/version = "1.0", author = "Paul", /*-h and --help*/about="testing",long_about = "A simple CLI tool")]
struct Cli {
    path: Option<PathBuf>,
}

#[derive(Debug, Display)]
enum EntryType {
    File,
    Dir,
}
#[derive(Debug)]
struct FileEntry {
    name: String,
    e_type: EntryType,
    len_bytes: u64,
    modified: String,
}
fn main() {
    let cli = Cli::parse();
    let path = cli.path.unwrap_or(PathBuf::from("."));

    if let Ok(does_exist) = fs::exists(&path) {
        if does_exist {
            for file in get_files(&path) {
                println!("{:?}", file);
            }
        } else {
            println!("{}", "path doesnt exist".red());
        }
    } else {
        println!("{}", "error checking path".red());
        // return;
    }

    print!("working?? path info -- {:?}\n", path);
}

fn get_files(path: &Path) -> Vec<FileEntry> {
    let mut data = Vec::default();
    if let Ok(read_dir) = fs::read_dir(path) {
        for entry in read_dir {
            if let Ok(file) = entry {
                if let Ok(meta) = fs::metadata(&file.path()) {
                    data.push(FileEntry {
                        name: file
                            .file_name()
                            .into_string()
                            .unwrap_or("unknown name".into()),
                        e_type: if meta.is_dir() {
                            EntryType::Dir
                        } else {
                            EntryType::File
                        },
                        len_bytes: meta.len(),
                        modified: "".to_string(), // placeholder for now
                    });
                }
            }
        }
    }
    data
}
