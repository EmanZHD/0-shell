use crate::commands::ls::ls_tools::{ parse_args, total_blocks, format_lines };
use crate::commands::ls::ls_models::{ Files, Flags };
use std::path::Path;
use std::{ env, fs, io };
use std::io::ErrorKind;
use crate::Params;

// ls_helper fn
fn ls_helper(path_str: &str, path_name: &str, flag: &Flags) -> Result<Vec<Vec<String>>, io::Error> {
    let mut content: Vec<String> = vec![];
    let path = Path::new(path_name);
    if (path.is_symlink() && flag.l_flag) || (path.is_symlink() && flag.f_flag) {
        let data = flag.line_data(path_str, path_name);
        if flag.l_flag {
            println!(
                "{} {}",
                data[0..data.len() - 1].join(" "),
                Files::file_format(path_name, path_str, &flag)
            );
        } else {
            println!("{}", Files::Symlink.file_symbol(&Files::Symlink.file_color(&path_str)));
        }
        return Ok(vec![]);
    }
    match fs::read_dir(path_str) {
        Ok(dir_entries) => {
            for entry in dir_entries.flatten() {
                if let Some(file_name) = entry.file_name().to_str() {
                    content.push(file_name.to_owned());
                }
            }
            if flag.a_flag {
                content.insert(0, ".".to_owned());
                content.insert(1, "..".to_owned());
            }
        }
        Err(e) => {
            if e.kind() == ErrorKind::PermissionDenied {
                eprintln!("ls: cannot open directory '{}': Permission denied", path_str);
                return Err(e);
            } else {
                content.push(path_str.to_owned());
            }
        }
    }
    Ok(format_lines(&mut content, flag, path_str))
}

//ls fn
pub fn ls(params: &mut Params) {
    let tilde = "~".to_string();
    let (flags, mut new_args) = match parse_args(params.args.clone()) {
        Ok((flags, new_args)) => (flags, new_args),
        Err(()) => {
            return;
        }
    };
    for s in &mut new_args {
        if *s == tilde {
            *s = format!("{}", params.home.display().to_string()).clone();
            break;
        }
    }
    new_args.sort();
    for (i, path_str) in new_args.iter().enumerate() {
        let mut path: String = path_str.to_string();
        if let Ok(curr_dir) = env::current_dir() {
            if let Some(s) = curr_dir.to_str() {
                if s == "/" {
                    path = s.to_string();
                }
            }
        }
        let path_name = Path::new(&path);
        match path_name.symlink_metadata() {
            Ok(path_data) => {
                if new_args.len() > 1 && path_data.is_dir() {
                    println!("{}:", path_str);
                }
                if let Ok(lines) = ls_helper(&path, path_str, &flags) {
                    if
                        !path_name.is_symlink() &&
                        flags.l_flag &&
                        Files::new_file(path_name) != Files::Reg
                    {
                        println!("total {}", total_blocks(path_name, flags.a_flag));
                    }
                    if flags.l_flag {
                        Files::display_line(lines, &path, &flags);
                    } else {
                        Files::display_file(lines, &path, &flags);
                    }
                }
            }
            Err(_) => eprintln!("ls: cannot access '{}': No such file or directory", path),
        }

        if i != new_args.len() - 1 && !new_args.is_empty() {
            println!();
        }
    }
}
