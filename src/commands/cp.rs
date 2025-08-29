use std::fs;
use std::path::Path;
pub fn cp(arg: &str) {
    let files: Vec<&str> = arg.split_whitespace().collect();

    if files.len() > 2 {
        multiple_source(files);
    } else {
        let exists_source = Path::new(files[0]).exists();
        let exists_dist = Path::new(files[1]).exists();
        let source_is_file = Path::new(files[0]).is_file();
        let dis_is_file = Path::new(files[1]).is_file();

        match (exists_source, exists_dist, source_is_file, dis_is_file) {
            (false, _, _, _) =>
                println!("cp: cannot stat '{}': No such file or directory", files[0]),
            (_, _, false, _) => println!("cp: omitting directory '{}' ", files[0]),

            (_, false, _, _) => {
                //hna dis hya file mkynch donc 5as n creah o ncopy fih source
                let parent = Path::new(files[1]).parent();
                match (parent.expect("REASON").exists(), parent.expect("REASON").is_dir()) {
                    (false, true) =>
                        println!("cp: cannot access '{}': No such file or directory", files[1]),
                    //    (_ , false) => println!("cp: cannot access '{}': No such file or directory" , files[1]) ,
                    (false, false) => {
                        fs::copy(files[0], files[1]);
                    }
                    (true, true) => {
                        fs::copy(files[0], files[1]);
                    }
                    (true, false) => println!("444444"),
                }
            }

            (_, true, _, false) => {
                let finle_dis = Path::new(files[1]).join(files[0]);
                fs::copy(files[0], finle_dis);
            }

            (true, true, true, true) => {
                fs::copy(files[0], files[1]);
            }
            (true, true, true, false) => {
                // mn file l dir
                let destination_file = Path::new(files[1]).join(
                    Path::new(files[0]).file_name().unwrap()
                );

                fs::copy(files[0], destination_file).expect("Failed to copy file");
            }
        }
    }
}

pub fn multiple_source(files: Vec<&str>) {
    // let mut is_err = false;
    let distination = Path::new(files[files.len() - 1]);
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
    // if !is_err {
    //     for (i, element) in files.iter().enumerate() {
    //         if i == files.len() - 1 {
    //         } else {

    //         }
    //     }
    // }
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
