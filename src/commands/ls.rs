use std::{ fs, thread::panicking };
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

pub fn ls_helper(path_name: &str) {
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
        }
        Err(e) => {
            eprintln!("can't read dir: {}", e)
            // let path: PathBuf = PathBuf::from(path_name);
            // if path.is_file() {
            //     match path.to_str() {
            //         Some(s) => { println!("{}", s) }
            //         None => println!("Path is not valid UTF-8"),
            //     }
            // } else {
            //     match path.to_str() {
            //         Some(s) => { eprintln!("can't read {} dir: {}", s, e) }
            //         None => println!("Path is not valid UTF-8"),
            //     }
            // }
        }
    }
}

pub fn ls(args: &mut Vec<&str>) {
    println!("LS args => {:?}", args);
    let mut path_name = "./".to_owned();
    if args.len() > 1 {
        args.sort();
        for (i, path_n) in args.iter().enumerate() {
            let path: PathBuf = PathBuf::from(path_n);
            // println!("testt--- {}", path.is_file());
            if path.is_file() {
                match path.to_str() {
                    Some(s) => {
                        path_n = s;
                    }
                    None => println!("Path not vqlid"),
                }
            } else {
                println!("{}:", path_n);
            }
            ls_helper(path_name);
            if i != args.len() - 1 {
                println!();
            }
        }
    } else {
        if args.len() == 1 {
            path_name = args[0];
            let path: PathBuf = PathBuf::from(path_name);
            // println!("testt--- {}", path.is_file());
            if path.is_file() {
                match path.to_str() {
                    Some(s) => {
                        println!("{}", s);
                        return;
                    }
                    None => println!("Path not vqlid"),
                }
            }
        }
        ls_helper(path_name);
        // println!("check type-> ", path_name.path().is_file());
    }
}
