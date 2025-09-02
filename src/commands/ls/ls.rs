use crate::commands::ls::ls_tools::{ parse_args, total_blocks };
use crate::commands::ls::ls_models::{ Files, Flags };
use std::path::Path;
use std::{ fs, io };
use std::io::ErrorKind;

// ls_printer fn
fn ls_printer(list: &mut Vec<String>, flag: &Flags, path_name: &str) -> Vec<Vec<String>> {
    list.sort_by(|f1, f2| {
        let f1_key = f1.strip_prefix('.').unwrap_or(f1).trim();
        let f2_key = f2.strip_prefix('.').unwrap_or(f2).trim();
        let f1_key = f1_key.replace('-', "");
        let f2_key = f2_key.replace('-', "");

        f1_key.to_lowercase().cmp(&&f2_key.to_lowercase())
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
            if flag.a_flag {
                content.insert(0, ".".to_owned());
                content.insert(1, "..".to_owned());
            }
        }
        Err(e) => {
            if e.kind() == ErrorKind::PermissionDenied {
                println!("ls: cannot open directory '{}': Permission denied", path_name);
            } else {
                content.push(path_name.to_owned());
            }
        }
    }
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
    new_args.sort();
    for (i, path_str) in new_args.iter().enumerate() {
        let path_name = Path::new(path_str);
        match path_name.metadata() {
            Ok(path_data) => {
                if
                    (path_name.is_symlink() && flags.l_flag) ||
                    (path_name.is_symlink() && flags.f_flag)
                {
                    if flags.l_flag {
                        // println!("PATHDATA -> {:?}", path_data);
                        println!(
                            "{} {}",
                            flags.format_output(path_str, path_str).join(" "),
                            Files::file_format(path_str, path_str, &flags)
                        );
                    } else {
                        println!(
                            "{}",
                            Files::Symlink.file_symbol(&Files::Symlink.file_color(&path_str))
                        );
                    }
                } else {
                    if new_args.len() > 1 && path_data.is_dir() {
                        println!("{}:", path_str);
                    }
                    if let Ok(lines) = ls_helper(path_str, &flags) {
                        if flags.l_flag && Files::new_file(Path::new(path_str)) != Files::Reg {
                            println!("total {}", total_blocks(Path::new(path_str), flags.a_flag));
                        }
                        if flags.l_flag {
                            Files::display_line(lines, path_str, &flags);
                        } else {
                            Files::format_out(lines, &path_str, &flags);
                        }
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
