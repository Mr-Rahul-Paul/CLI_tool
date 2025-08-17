use clap::Parser;
use owo_colors::OwoColorize;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Parser)]
#[command(/*-V*/version = "1.0", author = "Paul", /*-h and --help*/about="testing",long_about = "A simple CLI tool")]
struct Cli {
    path: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    let path = cli.path.unwrap_or(PathBuf::from("."));

    if let Ok(does_exist) = fs::exists(&path) {
        if does_exist {
            for file in get_files(& path){
                println!("{}",file);
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

fn get_files(path: &Path) -> Vec<String> {
    let mut data = Vec::default();
    if let Ok(read_dir) = fs::read_dir(path) {
        for entry in read_dir {
            if let Ok(file) = entry {
                data.push(
                    file.file_name()
                        .into_string()
                        .unwrap_or("unknown name".into()),
                );
            }
        }
    }
    return data;
}
