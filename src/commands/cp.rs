use std::fs;
use std::path::Path;
use crate::Params;
use std::io::ErrorKind;
use std::io;
use std::fs::{ File, OpenOptions };

pub fn cp(input: &mut Params) {
    if input.args.len() == 0 {
        println!("cp: missing file operand");
    } else if input.args.len() == 1 {
        println!("cp: missing destination file operand after {}", input.args[0])
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
        match (star, same, exists_source, exists_dist, source_is_file, dis_is_file) {
            (false, true, _, _, _, _) =>
                println!("cp: '{}' and '{}' are the same file", input.args[0], input.args[1]),
            (false, false, false, _, _, _) =>
                println!("cp: cannot stat '{}': No such file or directory", input.args[0]),
            (false, false, true, _, false, _) =>
                println!("cp: -r not specified; omitting directory '{}' ", input.args[0]),

            (false, false, true, false, true, _) => {
                let source = Path::new(&input.args[0]);
                let destination = Path::new(&input.args[1]);
                if let Some(parent) = destination.parent() {
                    if !parent.exists() {
                        println!(
                            "cp: cannot create regular file '{}': No such file or directory",
                            destination.display()
                        );
                    } else {
                        // fs::copy(source, destination);
                        copy_file(&input.args[0], &input.args[1]);
                    }
                }
                copy_file(&input.args[0], &input.args[1]);
                // fs::copy(source, destination);
            }

            (false, false, true, true, true, false) => {
                println!("hhhhh");
                let finle_dis = Path::new(&input.args[1]).join(&input.args[0]);
                // fs::copy(&input.args[0], finle_dis);
                copy_file(&input.args[0], finle_dis.to_str().expect("err in convert"));
            }

            (false, false, true, true, true, true) => {
                println!("kkkkk");
                let source = Path::new(&input.args[0]);
                let destination = Path::new(&input.args[1]);
                copy_file(&input.args[0], &input.args[1]);
            }

            (true, false, false, true, false, false) => {
                println!("ddddddd");

                star_source(&input.args[0], Path::new(&input.args[1]), false);
            }

            (true, false, false, true, false, true) => {
                println!("zaaaa");

                star_source(&input.args[0], Path::new(&input.args[1]), true);
            }

            (_, _, _, _, _, _) => {}
        }
    }
}

pub fn multiple_source(files: Vec<String>) {
    // let mut is_err = false;
    let destination = Path::new(&files[files.len() - 1]);
    if !destination.exists() {
        // is_err = true;
        println!("cp: target '{}': No such file or directory", destination.display());
    }
    if destination.exists() && destination.is_file() {
        // is_err = true;
        println!("cp: target '{}': Not a directory", destination.display());
    }
    for (i, element) in files.iter().enumerate() {
        if i == files.len() - 1 {
        } else {
            let mut tomp = Path::new(element);
            if
                element.chars().nth(0) == Some('"') &&
                element.chars().nth(element.len() - 1) == Some('"')
            {
                let mut get_element = element.chars();
                get_element.next();
                get_element.next_back();
                get_element.as_str();
                let res_elent: String = get_element.collect();
                // if res_elent.chars().nth(0) == Some('*') {
                //         star_source(&res_elent , destination)
                //     } else {
                let mut source = Path::new(&res_elent);
                if !source.exists() {
                    // is_err = true;
                    println!("cp: target '{}': No such file or directory", source.display());
                }
                if source.exists() && source.is_dir() {
                    // is_err = true;
                    println!("cp: -r not specified; omitting directory '{}'", source.display());
                }
                if source.exists() && source.is_file() {
                    let mut dis_path = destination.join(source);
                    // match fs::copy(source, dis_path) {
                    //     Ok(_) => {}
                    //     Err(e) => {}
                    // }
                    copy_file(
                        source.to_str().expect("err in convert"),
                        dis_path.to_str().expect("err in convert")
                    );
                }
                // }
            } else if (!tomp.exists() && element.chars().nth(0) != Some('*')) || tomp.is_dir() {
                // is_err = true;
                println!("cp: -r not specified; omitting directory '{}'", element);
            } else if element.chars().nth(0) == Some('*') {
                star_source(element, destination, false);
            } else {
                let mut source = Path::new(element);
                let mut dis_path = destination.join(source);
                // match fs::copy(source, dis_path) {
                //     Ok(_) => {}
                //     Err(e) => {}
                // }
                copy_file(
                    source.to_str().expect("err in convert"),
                    dis_path.to_str().expect("err in convert")
                );
            }
        }
    }
}

pub fn star_source(element: &str, destination: &Path, if_file: bool) {
    let suffix = &element[1..];
    let mut found_file = false;
    //read_dir That gives you a list of files and folders
    //read_dir gives u Iterator

    if let Ok(element_curr) = fs::read_dir(".") {
        for item in element_curr {
            if item.is_ok() {
                let path = item.expect("REASON").path();
                if path.is_file() {
                    if let Some(file_name) = path.file_name() {
                        let file_name = path.file_name().unwrap();
                        if file_name.to_string_lossy().ends_with(suffix) {
                            found_file = true;

                            let mut source = Path::new(file_name);
                            // println!("kkkkkk{:?}" , found_file);
                            if if_file {
                                // match fs::copy(source, destination) {
                                //     Ok(_) => {}
                                //     Err(e) => {}
                                // }
                                copy_file(
                                    source.to_str().expect("err in convert"),
                                    destination.to_str().expect("err in convert")
                                );
                            } else {
                                let mut dis_path = destination.join(file_name);
                                // match fs::copy(source, dis_path) {
                                //     Ok(_) => {}
                                //     Err(e) => println!("{:?}", e),
                                // }
                                copy_file(
                                    source.to_str().expect("err in convert"),
                                    dis_path.to_str().expect("err in convert")
                                );
                            }
                        }
                    }
                } else {
                    if let Some(file_name) = path.file_name() {
                        let file_name = path.file_name().unwrap();
                        if file_name.to_string_lossy().ends_with(suffix) {
                            // println!("======{:?}", path);
                            println!(
                                "cp: -r not specified; omitting directory '{}'",
                                file_name.display()
                            );
                        }
                    }
                }
            }
        }
        if !found_file {
            println!("cp: cannot stat '*.txt': No such file or directory");
        }
    }
}

fn copy_file(source: &str, destination: &str) {
    let mut src_file = match File::open(source) {
        Ok(f) => f,
        Err(e) => {
            if e.kind() == ErrorKind::PermissionDenied {
                println!("cp: cannot open '{}' for reading: Permission denied", source);
            } else {
                println!("cp: cannot open '{}' for reading", source);
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
    io::copy(&mut src_file, &mut dest_file);
}
