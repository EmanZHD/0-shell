use std::{ fs, io, iter::FlatMap, os::unix::fs::FileTypeExt };
use colored::{ ColoredString, Colorize };
use std::path::Path;
use is_executable::is_executable;

#[derive(Debug)]
pub enum Files {
    Symlink,
    Dir,
    Socket,
    Exec,
    Fifo,
    Reg,
}

impl Files {
    pub fn new_file(p_: &Path) -> Self {
        // let p_ = Path::new(p);
        // println!("INSIE NEW --> {:?}", p_.is_symlink());
        if p_.is_dir() {
            return Files::Dir;
        }
        if p_.is_symlink() {
            return Files::Symlink;
        }
        if let Ok(path) = fs::metadata(p_) {
            if path.file_type().is_socket() {
                return Files::Socket;
            }
            if path.file_type().is_fifo() {
                return Files::Fifo;
            }
        }
        if p_.is_file() && is_executable(p_) {
            return Files::Exec;
        }
        Files::Reg
    }

    pub fn file_color(&self, path_str: &str) -> ColoredString {
        match self {
            Files::Dir => path_str.bold().blue(),
            Files::Exec => path_str.bold().green(),
            Files::Socket => path_str.bold().magenta(),
            Files::Fifo => path_str.yellow(),
            Files::Symlink => path_str.bold().cyan(),
            _ => path_str.white(),
        }
    }

    pub fn file_symbol(&self, path_str: &ColoredString) -> String {
        // println!("INSIDE SYmbole---> {:?}", self);
        let mut s = path_str.to_string();
        match self {
            Files::Dir => s.push('/'),
            Files::Exec => s.push('*'),
            Files::Socket => s.push('='),
            Files::Fifo => s.push('|'),
            Files::Symlink => s.push('@'),
            _ => {}
        }
        s
    }
}

pub fn ls_printer(list: &mut Vec<(String, Files)>, flag: bool) {
    // list.sort_by(|f1, f2| f1.0.cmp(&f2.0));
    list.sort_by(|f1, f2| f1.0.to_lowercase().cmp(&f2.0.to_lowercase()));

    for c in list {
        if !flag {
            print!("{} ", c.1.file_color(&c.0));
        } else {
            print!("{} ", c.1.file_symbol(&c.1.file_color(&c.0)));
        }
    }
    println!();
}

pub fn ls_helper(path_name: &str, flag: bool) -> Result<(), io::Error> {
    let mut content: Vec<(String, Files)> = vec![];
    // println!("🪄 track entries -> {:?}  ", content);
    for entry in fs::read_dir(path_name)? {
        match entry {
            Ok(dir_entry) => {
                let p = dir_entry.path();
                if let Some(file_name) = dir_entry.file_name().to_str() {
                    if !file_name.is_empty() && !file_name.starts_with('.') {
                        // println!("-------> {:?} WITH TYPE => {:?}", p, Files::new_file(&p));
                        content.push((file_name.to_owned(), Files::new_file(&p)));
                    }
                }
            }
            Err(_e) => eprintln!("error in readinf '{}'", path_name),
        }
    }
    // content.sort();
    // detect_file_type(&p);
    ls_printer(&mut content, flag);
    Ok(())
}

pub fn ls(args: Vec<String>) {
    let mut detect_flag = false;
    let mut arg = args.clone();

    let mut new_args: Vec<String> = if args.is_empty() {
        vec!["./".to_string()]
    } else if args.len() == 1 && args[0] == "-F" {
        detect_flag = true;
        arg.pop();
        vec!["./".to_string()]
    } else {
        if let Some(last) = arg.last() {
            if last == "-F" {
                detect_flag = true;
                arg.pop();
            }
        }
        arg
    };

    println!("🏳️ CHECK FLAG {:?}", detect_flag);
    new_args.sort();
    // println!("LS args BEFORE=> {:?}", new_args);
    for (i, path_str) in new_args.iter().enumerate() {
        let path_name = Path::new(path_str);
        // println!("{} {:?}", "🪄 detect file type -->".yellow().bold(), path_name.metadata());
        match path_name.metadata() {
            Ok(path_data) => {
                if path_data.is_file() {
                    println!("{}", path_str);
                } else if path_data.is_dir() {
                    // print!("{}  ", path_data);
                    if args.len() > 1 {
                        println!("{}:", path_str);
                    }
                    let _ = ls_helper(path_str, detect_flag);
                }
            }
            Err(_) => eprintln!("ls: cannot access '{}': No such file or directory", path_str),
        }

        if i != new_args.len() - 1 {
            println!();
        }
    }
}
