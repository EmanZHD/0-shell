use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::path::Path;
use std::io::Write;

use crate::Params;
pub fn rm(params: &mut Params) {
    match params.args.len() {
        0 => println!("rm: missing operand"),
        _ => {
            match params.args[0].as_str() {
                "-r" => {
                    for arg in &params.args[1..] {
                        let path = Path::new(arg);
                        if path.exists() {
                            if path.is_file() {
                                is_file(path);
                            } else {
                                match fs::remove_dir_all(path) {
                                    Ok(_) => {}
                                    Err(_) => println!("err {}", path.display()),
                                }
                            }
                        } else {
                            println!(
                                "rm: cannot remove '{}': No such file or directory",
                                path.display()
                            );
                        }
                    }
                }
                _ => {
                    for arg in &params.args {
                        let path = Path::new(arg);
                        if path.exists() {
                            if path.is_file() {
                                is_file(path);
                            } else {
                                println!("rm: cannot remove '{}': Is a directory", path.display());
                            }
                        } else {
                            println!(
                                "rm: cannot remove '{}': No such file or directory",
                                path.display()
                            );
                        }
                    }
                }
            }
        }
    }
}

fn is_writable(path: &Path) -> bool {
    OpenOptions::new().write(true).open(path).is_ok()
}

fn is_file(path: &Path) {
    if is_writable(path) {
        match fs::remove_file(path) {
            Ok(_) => {}
            Err(_) => println!("err {}", path.display()),
        }
    } else {
        println!("rm: remove write-protected regular empty file '{}'? ", path.display());
        io::stdout().flush().unwrap(); // printed befor waiting for input

        let mut response = String::new();
        if let Ok(_) = io::stdin().read_line(&mut response) {
            let resp = response.trim().to_lowercase();
            if resp == "y" || resp == "yes" {
                match fs::remove_file(path) {
                    Ok(_) => {}
                    Err(_) => println!("err {}", path.display()),
                }
            }
        }
    }
}
