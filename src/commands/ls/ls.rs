use crate::commands::ls::ls_tools::{ parse_args, col_width };
use crate::commands::ls::ls_models::{ Files, Flags };
use std::path::Path;
use std::{ fs, io };
use colored::Colorize;

// ls_printer fn
fn ls_printer(list: &mut Vec<(String, Files)>, flag: &Flags, path_name: &str) -> Vec<Vec<String>> {
    list.sort_by(|f1, f2| {
        let f1_key = f1.0.strip_prefix('.').unwrap_or(&f1.0);
        let f2_key = f2.0.strip_prefix('.').unwrap_or(&f2.0);
        f1_key.to_lowercase().cmp(&f2_key.to_lowercase())
    });
    let mut line = Vec::new();
    for c in list {
        if flag.hidden_file(&c.0) {
            if !flag.l_flag {
                // print!("{:?}", flag.format_output(&c.0, &c.1, path_name));
                line.push(flag.format_output(&c.0, &c.1, path_name));
            } else {
                // println!("{:?}", flag.format_output(&c.0, &c.1, path_name));
                line.push(flag.format_output(&c.0, &c.1, path_name));
            }
        }
    }
    line
}

// ls_helper fn
fn ls_helper(path_name: &str, flag: &Flags) -> Result<Vec<Vec<String>>, io::Error> {
    let mut lines: Vec<Vec<String>> = vec![];
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
    // lines.push(ls_printer(&mut content, flag, path_name));
    lines.extend(ls_printer(&mut content, flag, path_name));
    Ok(lines)
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
                    if let Ok(lines) = ls_helper(path_str, &flags) {
                        println!("HEREEE");
                        // for line in lines {
                        //     println!("{:?}", line);
                        // }
                        // println!("{:?}", lines);
                        display_line(lines);
                    }
                }
            }
            Err(_) => eprintln!("ls: cannot access '{}': No such file or directory", path_str),
        }

        if i != new_args.len() - 1 {
            println!();
        }
    }
}

pub fn display_line(lines: Vec<Vec<String>>) {
    // println!("{:?}", lines);
    let col_width = col_width(&lines);
    for line in &lines {
        for (i, elem) in line.iter().enumerate() {
            let file_name = i == line.len() - 1;
            let is_num: bool = elem.chars().all(|e| (e.is_ascii_digit() || e == ','));
            // println!("CHECKK-->{:?}", is_num);
            if file_name {
                print!("{:<w$} ", elem.red(), w = col_width[i]);
            } else if is_num {
                print!("{:>w$} ", elem, w = col_width[i]);
            } else {
                print!("{:<w$} ", elem, w = col_width[i]);
            }
        }
        println!();
    }
}
