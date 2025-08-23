use std::path::Path;
use std::env::set_current_dir;

pub fn cd(arguments: Vec<String>) {
    if arguments.len() == 0 {
        println!("Please, specify the path you want to enter 🙃")
    }else {
        println!("✅ Verification (cd): {:?}", arguments[0]);
        let path = Path::new(&arguments[0]);
        if let Err(_e) = set_current_dir(&path) {
           eprintln!("⛔ 0-shell: No such file or directory {:?}", path);
        }
    }
}