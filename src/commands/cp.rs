use std::fs;
use std::path::Path;
use crate::Params;
use std::io::ErrorKind;
use std::io;
use std::env;
use std::fs::{ File, OpenOptions };

pub fn cp(input: &mut Params) {
    if input.args.len() == 0 {
        eprintln!("cp: missing file operand");
    } else if input.args.len() == 1 {
        eprintln!("cp: missing destination file operand after {}", input.args[0])
    } else if input.args.len() > 2 {
        multiple_source(input.args.clone());
    } else {
        let exists_source = Path::new(&input.args[0]).exists();
        let exists_dist = Path::new(&input.args[1]).exists();
        let source_is_file = Path::new(&input.args[0]).is_file();
        let dis_is_file = Path::new(&input.args[1]).is_file();
        let mut star = false;
        let mut same = false;

        if input.args[0] == input.args[1] {
            same = true;
        }
        if input.args[0].chars().nth(0) == Some('*') {
            star = true;
        }

        let metadata = match std::fs::symlink_metadata(&Path::new(&input.args[1])) {
            Ok(r) => r,
            Err(_err) => {
                eprintln!("Failed to read metadata for '{}'", Path::new(&input.args[1]).display());
                return;
            }
        };
        if metadata.file_type().is_symlink() {
            copy_file(&input.args[0], &input.args[1]);
            return;
        }

        match (star, same, exists_source, exists_dist, source_is_file, dis_is_file) {
            (false, true, _, _, _, _) =>
                eprintln!("cp: '{}' and '{}' are the same file", input.args[0], input.args[1]),
            (false, false, false, _, _, _) =>
                eprintln!("cp: cannot stat '{}': No such file or directory", input.args[0]),
            (false, false, true, _, false, _) =>
                eprintln!("cp: -r not specified; omitting directory '{}'", input.args[0]),

            (false, false, true, false, true, _) => {
                let _source = Path::new(&input.args[0]);
                let destination = Path::new(&input.args[1]);
                if let Some(parent) = destination.parent() {
                    if !parent.exists() {
                        eprintln!(
                            "cp: cannot create regular file '{}': No such file or directory",
                            destination.display()
                        );
                    } else {
                        copy_file(&input.args[0], &input.args[1]);
                    }
                }
                copy_file(&input.args[0], &input.args[1]);
            }

            (false, false, true, true, true, false) => {
                let finle_dis = Path::new(&input.args[1]).join(&input.args[0]);
                copy_file(&input.args[0], finle_dis.to_str().expect("Err in convert"));
            }

            (false, false, true, true, true, true) => {
                let _source = Path::new(&input.args[0]);
                let _destination = Path::new(&input.args[1]);
                copy_file(&input.args[0], &input.args[1]);
            }

            (true, false, false, true, false, false) => {
                star_source(&input.args[0], Path::new(&input.args[1]), false);
            }

            (true, false, false, true, false, true) => {
                star_source(&input.args[0], Path::new(&input.args[1]), true);
            }

            (_, _, _, _, _, _) => {}
        }
    }
}

// to copy multiple sources

pub fn multiple_source(files: Vec<String>) {
    let destination = Path::new(&files[files.len() - 1]);
    if !destination.exists() {
        eprintln!("cp: target '{}': No such file or directory", destination.display());
    }
    if destination.exists() && destination.is_file() {
        eprintln!("cp: target '{}': Not a directory", destination.display());
        return;
    }
    for (i, element) in files.iter().enumerate() {
        if i == files.len() - 1 {
        } else {
            let tomp = Path::new(element);
            let metadata = match std::fs::symlink_metadata(&tomp) {
                Ok(r) => r,
                Err(_err) => {
                    eprintln!("Failed to read metadata for '{}'", tomp.display());
                    continue;
                }
            };

            if metadata.file_type().is_symlink() {
                match std::fs::remove_file(&tomp) {
                    Ok(_) => {
                        continue;
                    }
                    Err(_err) => {
                        eprintln!(
                            "rm: can't remove '{}': No such file or directory ",
                            tomp.display()
                        );
                    }
                }
            }
            if !tomp.exists() && element.chars().nth(0) != Some('*') {
                eprintln!("cp: cannot stat '{}': No such file or directory", element);
            } else if tomp.is_dir() {
                eprintln!("cp: -r not specified; omitting directory '{}'", element);
            } else if element.chars().nth(0) == Some('*') {
                star_source(element, destination, false);
            } else {
                let source = Path::new(element);
                let dis_path = destination.join(source);
                copy_file(
                    source.to_str().expect("Err in convert"),
                    dis_path.to_str().expect("Err in convert")
                );
            }
        }
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
                        let file_name = path.file_name().unwrap();
                        if file_name.to_string_lossy().ends_with(suffix) {
                            found_file = true;

                            let source = Path::new(file_name);
                            if if_file {
                                copy_file(
                                    source.to_str().expect("Err in convert"),
                                    destination.to_str().expect("Err in convert")
                                );
                            } else {
                                let dis_path = destination.join(file_name);
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
                                "cp: -r not specified; omitting directory '{}'",
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
                eprintln!("cp: cannot open '{}' for reading: Permission denied", source);
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
                eprintln!("cp: cannot create regular file '{}': Permission denied", destination);
            } else {
                eprintln!("cp: cannot create regular file '{}': {}", destination, e);
            }
            return;
        }
    };

    let path = env::current_dir();

    if
        destination == format!("{}/{}", path.expect("REASON").to_string_lossy(), source) ||
        destination == format!("./{}", source)
    {
        eprintln!("cp: '{}' and '{}' are the same file", source, &destination);
        return;
    }

    match io::copy(&mut src_file, &mut dest_file) {
        Ok(_) => {}
        Err(_) => {}
    }
}
