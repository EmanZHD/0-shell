use std::fs;
use std::path::Path;
use crate::Params;
use std::io::ErrorKind;
use std::io;
use std::env;
use std::fs::{ File, OpenOptions };

pub fn cp(input: &mut Params) {
    match input.args.len() {
        0 => eprintln!("cp: missing file operand"),
        1 => eprintln!("cp: missing destination file operand after '{}'", input.args[0]),
        _ => cp_algo(input),
    }
}

// to copy multiple sources
pub fn cp_algo(input: &mut Params) {
    let args = &input.args;

    if args.len() > 2 {
        multiple_source(args.clone());
        return;
    }

    let src = Path::new(&args[0]);
    let dst = Path::new(&args[1]);

    let exists_source = src.exists();
    let exists_dist = dst.exists();
    let source_is_file = src.is_file();
    let dst_is_file = dst.is_file();

    let _dir = env::current_dir();

    if args[0].chars().nth(0) == Some('*') {
        star_source(&args[0], dst, dst_is_file);
        return;
    }

    if is_smylink(&dst) || is_smylink(&src) {
        copy_file(&args[0], &args[1]);
        return;
    }

    if !exists_source {
        eprintln!("cp: cannot stat '{}': No such file or directory", args[0]);
        return;
    }

    if !source_is_file {
        eprintln!("cp: '{}' is a directory — sorry recursive copy isn’t supported!", args[0]);
        return;
    }

    match (exists_dist, dst_is_file) {
        (false, _) => {
            if args[1].contains("/") {
                if let Some(parent) = dst.parent() {
                    if !parent.exists() {
                        eprintln!(
                            "cp: cannot create regular file '{}': No such file or directory",
                            dst.display()
                        );
                        return;
                    }
                }
            }

            copy_file(&args[0], &args[1]);
        }

        (true, false) => {
            let finle_dis = dst.join(&args[0]);
            copy_file(&args[0], finle_dis.to_str().expect("Err in convert"));
        }

        (true, true) => {
            copy_file(&args[0], &args[1]);
        }
    }
}

pub fn is_smylink(element: &Path) -> bool {
    if let Ok(metadata) = std::fs::symlink_metadata(&element) {
        if metadata.file_type().is_symlink() {
            return true;
        }
    }
    return false;
}

// For copying multiple sources to a destination

pub fn multiple_source(files: Vec<String>) {
    let des = &files[files.len() - 1];
    let destination_path = Path::new(&files[files.len() - 1]);
    if !destination_path.exists() {
        eprintln!("cp: target '{}': No such file or directory", destination_path.display());
        return;
    }
    if destination_path.exists() && destination_path.is_file() {
        eprintln!("cp: target '{}': Not a directory", destination_path.display());
        return;
    }

    for element in &files[..files.len() - 1] {
        let tomp = Path::new(element);

        if is_smylink(&destination_path) || is_smylink(&tomp) {
            copy_file(element, des);
            return;
        }

        if !tomp.exists() && element.chars().nth(0) != Some('*') {
            eprintln!("cp: cannot stat '{}': No such file or directory", element);
            continue;
        }
        if tomp.is_dir() {
            eprintln!("cp: '{}' is a directory — sorry recursive copy isn’t supported!", element);
            continue;
        }

        if element.chars().nth(0) == Some('*') {
            star_source(element, destination_path, false);
            continue;
        }

        let source = Path::new(element);
        let dis_path = destination_path.join(source);
        copy_file(
            source.to_str().expect("Err in convert"),
            dis_path.to_str().expect("Err in convert")
        );
    }
}

// For selecting multiple files by their suffix

pub fn star_source(element: &str, destination: &Path, if_file: bool) {
    let suffix = &element[1..];
    let mut found_file = false;

    if let Ok(element_curr) = fs::read_dir(".") {
        for item in element_curr {
            if item.is_ok() {
                let path = item.expect("expected at least one file entry").path();
                if path.is_file() {
                    if let Some(_file_name) = path.file_name() {
                        if _file_name.to_string_lossy().ends_with(suffix) {
                            found_file = true;

                            let source = Path::new(_file_name);
                            if if_file {
                                copy_file(
                                    source.to_str().expect("Err in convert"),
                                    destination.to_str().expect("Err in convert")
                                );
                            } else {
                                let dis_path = destination.join(_file_name);
                                copy_file(
                                    source.to_str().expect("Err in convert"),
                                    dis_path.to_str().expect("Err in convert")
                                );
                            }
                        }
                    }
                } else {
                    if let Some(_file_name) = path.file_name() {
                        let file_name = path.file_name().unwrap();
                        if file_name.to_string_lossy().ends_with(suffix) {
                            eprintln!(
                                "cp: '{}' is a directory — sorry recursive copy isn’t supported!",
                                file_name.display()
                            );
                        }
                    }
                }
            }
        }
        if !found_file {
            eprintln!("cp: cannot stat '*.txt': No such file or directory");
        }
    }
}

// For copying a file to the destination

fn copy_file(source: &str, destination: &str) {
    let mut src_file = match File::open(source) {
        Ok(f) => f,
        Err(e) => {
            if e.kind() == ErrorKind::PermissionDenied {
                eprintln!("cp: cannot open '{}' for reading: Permission denied 🔒", source);
            } else {
                eprintln!("cp: cannot open '{}' for reading", source);
            }
            return;
        }
    };
    let mut dest_file = match
        OpenOptions::new().write(true).create(true).truncate(true).open(destination)
    {
        Ok(f) => f,
        Err(e) => {
            if e.kind() == ErrorKind::PermissionDenied {
                eprintln!("cp: cannot create regular file '{}': Permission denied 🔒", destination);
            }
            return;
        }
    };

    let path = env::current_dir();

    if
        destination == format!("{}/{}", path.expect("REASON").to_string_lossy(), source) ||
        destination == format!("./{}", source) ||
        source == destination
    {
        eprintln!("cp: '{}' and '{}' are the same file.", source, &destination);
        return;
    }

    match io::copy(&mut src_file, &mut dest_file) {
        Ok(_) => {}
        Err(_) => {}
    }
}
