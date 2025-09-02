use std::fs;
use std::path::Path;
use crate::Params;
pub fn cp(input: &mut Params) {
    // println!("{:?}" , input.args);
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
        let mut same = false;
        if input.args[0] == input.args[1] {
            same = true;
        }

        match (same, exists_source, exists_dist, source_is_file, dis_is_file) {
            (true, _, _, _, _) =>
                println!("cp: '{}' and '{}' are the same file  ", input.args[0], input.args[1]),
            (false, false, _, _, _) =>
                println!("cp: cannot stat '{}': No such file or directory", input.args[0]),
            (false, true, _, false, _) =>
                println!("cp: -r not specified; omitting directory '{}' ", input.args[0]),

            (false, true, false, true, _) => {
                let source = Path::new(&input.args[0]);
                let destination = Path::new(&input.args[1]);
                if let Some(parent) = destination.parent() {
                    if !parent.exists() {
                        println!(
                            "cp: cannot create regular file '{}': No such file or directory   ",
                            destination.display()
                        );
                    } else {
                        fs::copy(source, destination);
                    }
                }
                fs::copy(source, destination);
            }

            (false, true, true, true, false) => {
                let finle_dis = Path::new(&input.args[1]).join(&input.args[0]);
                fs::copy(&input.args[0], finle_dis);
            }

            (false, true, true, true, true) => {
                let source = Path::new(&input.args[0]);
                let destination = Path::new(&input.args[1]);
                fs::copy(source, &destination);
            }
        }
    }
}

pub fn multiple_source(files: Vec<String>) {
    // let mut is_err = false;
    let distination = Path::new(&files[files.len() - 1]);
    if !distination.exists() {
        // is_err = true;
        println!("cp: target '{}': No such file or directory", distination.display());
    }
    if distination.exists() && distination.is_file() {
        // is_err = true;
        println!("cp: target '{}': Not a directory", distination.display());
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
                //         star_source(&res_elent , distination)
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
                    let mut dis_path = distination.join(source);
                    match fs::copy(source, dis_path) {
                        Ok(_) => {}
                        Err(e) => {}
                    }
                }
                // }
            } else if (!tomp.exists() && element.chars().nth(0) != Some('*')) || tomp.is_dir() {
                // is_err = true;
                println!("cp: -r not specified; omitting directory '{}'", element);
            } else if element.chars().nth(0) == Some('*') {
                star_source(element, distination);
            } else {
                let mut source = Path::new(element);
                let mut dis_path = distination.join(source);
                match fs::copy(source, dis_path) {
                    Ok(_) => {}
                    Err(e) => {}
                }
            }
        }
    }
}

pub fn star_source(element: &str, distination: &Path) {
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
                            // println!("kkkkkk{:?}" , found_file);
                            let mut dis_path = distination.join(file_name);
                            let mut source = Path::new(file_name);
                            match fs::copy(source, dis_path) {
                                Ok(_) => {}
                                Err(e) => {}
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
