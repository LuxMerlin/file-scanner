use core::fmt;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "PATH")]
    path: Option<PathBuf>,
    #[arg(short)]
    show_output: bool,
    #[arg(short, long)]
    verbose: bool,
}

struct Item {
    path: PathBuf,
    name: OsString,
    file_type: FileType,
    sub_items: Vec<Item>,
}

enum FileType {
    File,
    Dir,
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileType::File => write!(f, "File"),
            FileType::Dir => write!(f, "Directory"),
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let user_path = cli.path.expect("Path was not provided");
    let all_items: Vec<Item> = scan(&user_path);

    println!(
        "File Count: {}",
        all_items
            .iter()
            .filter(|item| matches!(item.file_type, FileType::File))
            .count()
    );

    println!(
        "Folder Count: {}",
        all_items
            .iter()
            .filter(|item| matches!(item.file_type, FileType::Dir))
            .count()
    );

    if cli.show_output {
        println!("Root: {}", user_path.display());
        print_items(&all_items, 1, cli.verbose);
    }
}

fn print_items(items: &Vec<Item>, level: i32, verbose: bool) {
    for i in items.iter().rev() {
        println!(
            "{0}{1} : {2} : {3}",
            print_items_prefix(level),
            i.file_type,
            i.name.to_str().unwrap(),
            if verbose {
                i.path.to_str().unwrap()
            } else {
                ""
            }
        );

        if i.sub_items.len() > 0 {
            print_items(&i.sub_items, level + 1, verbose);
        }
    }
}

fn print_items_prefix(level: i32) -> String {
    let mut prefix = String::new();

    for _i in 0..level {
        prefix += " ";
    }
    prefix += "↳ ";
    return prefix;
}

fn scan(current_path: &PathBuf) -> Vec<Item> {
    let paths = fs::read_dir(&current_path).unwrap();
    let mut items = Vec::new();

    for path in paths {
        if let Ok(path) = path {
            if let Ok(file_type) = path.file_type() {
                if file_type.is_dir() {
                    let subitems = scan(&path.path());

                    items.push(Item {
                        path: path.path(),
                        name: path.file_name(),
                        file_type: FileType::Dir,
                        sub_items: subitems,
                    });
                } else {
                    items.push(Item {
                        path: path.path(),
                        name: path.file_name(),
                        file_type: FileType::File,
                        sub_items: Vec::new(),
                    });
                }
            } else {
                println!("Couldn't get file type for {:?}", path.path());
            }
        }
    }

    return items;
}
