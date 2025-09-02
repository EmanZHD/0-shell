use std::fs;
// use crate::errors::CrateResult;
use anyhow::anyhow;
use std::path::Path;
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
                                match fs::remove_file(path) {
                                    Ok(_) => {}
                                    Err(_) => println!("err {}", path.display()),
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
                                match fs::remove_file(path) {
                                    Ok(_) => {}
                                    Err(_) => println!("err {}", path.display()),
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
