use crate::commands::ls::ls_tools::{ parse_args, col_width };
use crate::commands::ls::ls_models::{ Files, Flags };
use std::path::Path;
use std::{ fs, io };

// ls_printer fn
fn ls_printer(list: &mut Vec<String>, flag: &Flags, path_name: &str) -> Vec<Vec<String>> {
    list.sort_by(|f1, f2| {
        let f1_key = f1.strip_prefix('.').unwrap_or(&f1);
        let f2_key = f2.strip_prefix('.').unwrap_or(&f2);
        f1_key.to_lowercase().cmp(&f2_key.to_lowercase())
    });
    let mut line = Vec::new();
    for c in list {
        if flag.hidden_file(&c) {
            line.push(flag.format_output(&c, path_name));
        }
    }
    line
}

// ls_helper fn
fn ls_helper(path_name: &str, flag: &Flags) -> Result<Vec<Vec<String>>, io::Error> {
    let mut lines: Vec<Vec<String>> = vec![];
    // println!("--->Path{:?}", path_name);
    let mut content: Vec<String> = vec![];
    match fs::read_dir(path_name) {
        Ok(dir_entries) => {
            for entry in dir_entries {
                match entry {
                    Ok(dir_entry) => {
                        if let Some(file_name) = dir_entry.file_name().to_str() {
                            content.push(file_name.to_owned());
                        }
                    }
                    Err(_e) => eprintln!("error in readinf '{}'", path_name),
                }
            }
            if !content.is_empty() && flag.a_flag {
                content.insert(0, ".".to_owned());
                content.insert(1, "..".to_owned());
            }
        }
        Err(_) => content.push(path_name.to_owned()),
    }

    // lines.push(ls_printer(&mut content, flag, path_name));
    // println!("content--> {:?} of PATH {:?}", content, path_name);
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
                if new_args.len() > 1 && path_data.is_dir() {
                    println!("{}:", path_str);
                }
                if let Ok(lines) = ls_helper(path_str, &flags) {
                    println!("HEREEE");
                    display_line(lines, path_str, &flags);
                }
            }
            Err(_) => eprintln!("ls: cannot access '{}': No such file or directory", path_str),
        }

        if i != new_args.len() - 1 {
            println!();
        }
    }
}

pub fn display_line(lines: Vec<Vec<String>>, path: &str, flag: &Flags) {
    // println!("{:?}", lines);
    let col_width = col_width(&lines);
    for line in &lines {
        for (i, elem) in line.iter().enumerate() {
            let file_name = i == line.len() - 1;
            let is_num: bool = elem.chars().all(|e| (e.is_ascii_digit() || e == ','));
            // println!("CHECKK-->{:?}", is_num);
            if file_name {
                print!("{:<w$} ", Files::file_format(elem, path, flag), w = col_width[i]);
            } else if is_num {
                print!("{:>w$} ", elem, w = col_width[i]);
            } else {
                print!("{:<w$} ", elem, w = col_width[i]);
            }
        }
        println!();
    }
}
