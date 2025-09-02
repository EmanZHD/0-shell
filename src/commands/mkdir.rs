use std::fs;
use std::path::Path;
use crate::Params;
pub fn mkdir(path: &mut Params) {
    for elemet in path.args.clone() {
        // Create a Path object from the directory path string
        let dir_path = Path::new(&elemet);
        if dir_path.exists() {
            println!("Directory '{}' already exists.", elemet);
        } else {
            match fs::create_dir(&elemet) {
                Ok(_) => {}
                Err(e) => eprintln!("Error creating directory '{}': {}", elemet, e),
          }
        }
    }
}
