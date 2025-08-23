use std::{ fs, io };
use colored::Colorize;
use std::path::Path;

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
    // println!("🪄 track entries -> {:?}  ", content);
    for entry in fs::read_dir(path_name)? {
        match entry {
            Ok(dir_entry) => {
                let p = dir_entry.path();
                if let Some(file_name) = dir_entry.file_name().to_str() {
                    if !file_name.is_empty() && !file_name.starts_with('.') {
                        content.push((file_name.to_owned(), p.is_dir()));
                    }
                }
            }
            Err(_e) => eprintln!("error in readinf '{}'", path_name),
        }
    }
    ls_printer(&mut content);
    Ok(())
}

pub fn ls(args: Vec<String>) {
    let mut new_args: Vec<String> = if args.is_empty() {
        vec!["./".to_string()]
    } else {
        args.clone()
    };
    new_args.sort();
    // println!("LS args BEFORE=> {:?}", new_args);

    for (i, path_str) in new_args.iter().enumerate() {
        let path_name = Path::new(path_str);
        // println!("🪄 metaaDATA -> {:?}", path_name.metadata());
        match path_name.metadata() {
            Ok(path_data) => {
                if path_data.is_file() {
                    println!("{}", path_str);
                } else if path_data.is_dir() {
                    // print!("{}  ", path_data);
                    if args.len() > 1 {
                        println!("{}:", path_str);
                    }
                    let _ = ls_helper(path_str);
                }
            }
            Err(_) => eprintln!("ls: cannot access '{}': No such file or directory", path_str),
        }

        if i != new_args.len() - 1 {
            println!();
        }
    }
}
