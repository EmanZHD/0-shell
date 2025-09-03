use std::fs;
// use crate::errors::CrateResult;
use anyhow::anyhow;
use std::fs::OpenOptions;
use std::io;
use std::path::Path;
 use std::io::Write; 
// ma3mdhach r ok
//m3ndhach w non
use crate::Params;
pub fn rm(params: &mut Params) {
    match params.args.len() {
        0 => println!("rm: missing operand"),
        _ => {
            match params.args[0].as_str() {
                "-r" => {
                    for i in &params.args[1..] {
                        let path = Path::new(i);
                        if path.exists() {
                            if path.is_file() {
                                 if is_writable(path) {
                                    match fs::remove_file(path) {
                                        Ok(_) => {}
                                        Err(_) => println!("err {}", path.display()),
                                    }
                                } else {
                                    println!("rm: remove write-protected regular empty file '{}'? ", i);
                                    io::stdout().flush().unwrap(); // printed befor waiting for input

                                    let mut response = String::new();
                                    if let Ok(_) = io::stdin().read_line(&mut response) {
                                        let trimmed = response.trim().to_lowercase();
                                        if trimmed == "y" || trimmed == "yes" {
                                            match fs::remove_file(path) {
                                                Ok(_) => {}
                                                Err(_) => println!("err {}", path.display()),
                                            }
                                        }
                                    }
                                }
                            } else {
                                match fs::remove_dir(path) {
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
                    for i in &params.args {
                        let path = Path::new(i);
                        if path.exists() {
                            if path.is_file() {
                                if is_writable(path) {
                                    match fs::remove_file(path) {
                                        Ok(_) => {}
                                        Err(_) => println!("err {}", path.display()),
                                    }
                                } else {
                                    println!("rm: remove write-protected regular empty file '{}'? ", i);
                                    io::stdout().flush().unwrap(); // printed befor waiting for input

                                    let mut response = String::new();
                                    if let Ok(_) = io::stdin().read_line(&mut response) {
                                        let trimmed = response.trim().to_lowercase();
                                        if trimmed == "y" || trimmed == "yes" {
                                            match fs::remove_file(path) {
                                                Ok(_) => {}
                                                Err(_) => println!("err {}", path.display()),
                                            }
                                        }
                                    }
                                }
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
