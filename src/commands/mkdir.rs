use std::fs;
use std::path::Path;
use crate::Params;

pub fn mkdir(path: &mut Params) {
    for arg in path.args.clone() {
        // Create a Path object from the directory path string
        let dir_path = Path::new(&arg);
        if dir_path.exists() {
            eprintln!("Directory '{}' already exists. 🐧", arg);
        } else if arg.starts_with("-") {
            eprintln!("Invalid name '{}': please avoid using '-' 🐧", arg);
        } else {
            match fs::create_dir(&arg) {
                Ok(_) => {}
                Err(e) => eprintln!("Error creating directory '{}': {}", arg, e),
            }
        }
    }
}
