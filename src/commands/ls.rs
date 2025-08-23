use std::{ fs, io };
use colored::Colorize;
use std::path::PathBuf;

pub fn ls_printer(list: &mut Vec<(String, bool)>) {
    list.sort();
    for c in list {
        if c.1 {
            print!("{} ", c.0.bold().blue());
        } else {
            print!("{} ", c.0);
        }
    }
    println!();
}

pub fn ls_helper(path_name: &str) -> Result<(), io::Error> {
    let mut content: Vec<(String, bool)> = vec![];

    match fs::read_dir(path_name) {
        Ok(entries) => {
            // println!("track entries -> {:?}  ", entries);
            for entry in entries {
                match entry {
                    Ok(dir_entry) => {
                        let p = dir_entry.path();
                        // println!("DIR ENTRY -> {:?}", p.is_file());
                        // if p.is_file() || p.is_dir() {
                        if let Some(filename) = dir_entry.file_name().to_str() {
                            if filename.len() >= 1 && filename.starts_with('.') {
                                continue;
                            } else {
                                content.push((filename.to_owned(), p.is_dir()));
                                // print!("{}  ", filename);
                            }
                        }
                        // }
                    }
                    Err(e) => eprintln!("err in reading this entry: {}", e),
                }
            }
            ls_printer(&mut content);
            Ok(())
        }
        // Err(_e) => { eprintln!("ls: cannot access '{}': No such file or directory", path_name) }
        Err(e) => {
            eprintln!("ls: cannot access '{}': {}", path_name, e);
            Err(e)
        }
    }
}

pub fn ls(args: Vec<String>) {
    let mut new_args: Vec<String> = args.clone();
    println!("LS args BEFORE=> {:?}", new_args);
    let mut path_name = "./";
    if args.len() > 1 {
        new_args.sort();
        println!("LS args AFTER=> {:?}", new_args);
        for (i, path_n) in new_args.iter().enumerate() {
            let path: PathBuf = PathBuf::from(path_n);
            // println!("testt--- {}", path.is_file());
            if path.is_file() {
                if let Err(e) = ls_helper(path_n) {
                    eprintln!("EROOR HERE'{}': {}", path_n, e);
                }
                println!("{}", path_n);
            } else {
                println!("{}:", path_n);
                if let Err(e) = ls_helper(path_n) {
                    eprintln!("EROOR HERE'{}': {}", path_n, e);
                }
            }
            if i != new_args.len() - 1 {
                println!();
            }
        }
    } else {
        if new_args.len() == 1 {
            path_name = &new_args[0];
            let path: PathBuf = PathBuf::from(path_name);
            // println!("testt--- {}", path.is_file());
            if path.is_file() {
                println!("{}", path_name);
                return;
            }
        }
        if let Err(e) = ls_helper(path_name) {
            eprintln!("EROOR HERE'{}': {}", path_name, e);
        }
        // println!("check type-> ", path_name.path().is_file());
    }
}
