use crate::commands::ls::ls_tools::{ parse_args };
use crate::commands::ls::ls_models::{ Files, Flags };
use std::path::Path;
use std::{ fs, io };

// ls_printer fn
fn ls_printer(list: &mut Vec<(String, Files)>, flag: &Flags, path_name: &str) {
    list.sort_by(|f1, f2| {
        let f1_key = f1.0.strip_prefix('.').unwrap_or(&f1.0);
        let f2_key = f2.0.strip_prefix('.').unwrap_or(&f2.0);
        f1_key.to_lowercase().cmp(&f2_key.to_lowercase())
    });
    for c in list {
        if flag.hidden_file(&c.0) {
            if !flag.l_flag {
                print!("{} ", flag.format_output(&c.0, &c.1, path_name));
            } else {
                println!("{}", flag.format_output(&c.0, &c.1, path_name));
            }
        }
    }
    println!();
}

// ls_helper fn
fn ls_helper(path_name: &str, flag: &Flags) -> Result<(), io::Error> {
    println!("--->Path{:?}", path_name);
    let mut content: Vec<(String, Files)> = vec![];
    for entry in fs::read_dir(path_name)? {
        match entry {
            Ok(dir_entry) => {
                let p = dir_entry.path();
                if let Some(file_name) = dir_entry.file_name().to_str() {
                    content.push((file_name.to_owned(), Files::new_file(&p)));
                }
            }
            Err(_e) => eprintln!("error in readinf '{}'", path_name),
        }
    }
    if !content.is_empty() && flag.a_flag {
        content.insert(0, (".".to_owned(), Files::Dir));
        content.insert(1, ("..".to_owned(), Files::Dir));
    }
    ls_printer(&mut content, flag, path_name);
    Ok(())
}

//ls fn
pub fn ls(args: Vec<String>) {
    let (flags, mut new_args) = match parse_args(args) {
        Ok((flags, new_args)) => (flags, new_args),
        Err(()) => {
            return;
        }
    };
    // println!("🏳️ CHECK FLAG {:?}", flags.);
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
                    if new_args.len() > 1 {
                        println!("{}:", path_str);
                    }
                    let _ = ls_helper(path_str, &flags);
                }
            }
            Err(_) => eprintln!("ls: cannot access '{}': No such file or directory", path_str),
        }

        if i != new_args.len() - 1 {
            println!();
        }
    }
}
