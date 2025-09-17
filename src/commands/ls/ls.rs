use crate::commands::ls::ls_models::{ Flags, FileInfo };
use crate::commands::ls::ls_tools::{ parse_args, process_dirs, process_files, handle_glob };
use std::{ fs };
use crate::Params;

//-------------------------handle_path FUNC
pub fn handle_path(paths: Vec<String>, flags: &Flags) -> (Vec<String>, Vec<String>) {
    let mut files = Vec::new();
    let mut dirs = Vec::new();

    for path in paths {
        let name = FileInfo::base_name(&path);
        if !flags.hidden_file(name) {
            continue;
        }
        match fs::symlink_metadata(&path) {
            Ok(metadata) => {
                let symlink_cond =
                    metadata.file_type().is_symlink() && !flags.l_flag && !flags.f_flag;
                if metadata.is_dir() || symlink_cond {
                    dirs.push(path);
                } else {
                    files.push(path);
                }
            }
            Err(_) => {
                eprintln!("ls: cannot access this dir'{}'", path);
            }
        }
    }
    files.sort();
    dirs.sort();

    (files, dirs)
}

//-------------------------ls FUNC
pub fn ls(params: &mut Params) {
    let (flags, mut paths) = match parse_args(params.args.clone()) {
        Ok((flags, paths)) => (flags, paths),
        Err(()) => {
            return;
        }
    };

    handle_glob(&mut paths);
    let (files, dirs) = handle_path(paths, &flags);

    process_files(&files, &flags);
    for (i, dir) in dirs.iter().enumerate() {
        if dirs.len() > 1 {
            println!("{}:", dir);
        }
        process_dirs(&dir, &flags);
        if i < dirs.len() - 1 {
            println!();
        }
    }
}
