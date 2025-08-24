use std::fs;
use std::path::Path;
pub fn mkdir(path :&str)  {
     // Create a Path object from the directory path string
        let dir_path = Path::new(path);
    if dir_path.exists(){
         println!("Directory '{}' already exists.", path);
    }else {
      match   fs::create_dir(path){
           Ok(_) => println!("Directory '{}' created successfully.", path),
            Err(e) => eprintln!("Error creating directory '{}': {}", path, e),    
      }

    }
}