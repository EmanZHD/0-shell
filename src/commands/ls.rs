use std::fs;
use colored::Colorize;

pub fn ls_helper(list: &mut Vec<(String, bool)>) {
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

pub fn ls(args: Vec<&str>) {
    println!("LS args => {:?}", args);
    let mut content: Vec<(String, bool)> = vec![];
    match fs::read_dir(".") {
        Ok(entries) => {
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
            ls_helper(&mut content);
        }
        Err(e) => eprintln!("can't read dir: {}", e),
    }
}
