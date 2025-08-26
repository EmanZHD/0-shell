use std::path::Path;
use std::env;
use std::env::set_current_dir;

pub fn cd(arguments: Vec<String>) {
    if arguments.len() == 0 {
        go_to_home()
    }else {
        if &arguments[0] == "~" || &arguments[0] == "--" {
            go_to_home();
            return;
        }
        let path = Path::new(&arguments[0]);
        if let Err(_e) = set_current_dir(&path) {
           eprintln!("⛔ 0-shell: No such file or directory {:?}", path);
        }
    }
}

fn go_to_home() {
    match env::home_dir() {
        Some(path) => {
            if let Err(_e) = set_current_dir(path) {
                println!("⛔ 0-shell: No such file or directory");
            }
        },
        None => println!("⛔ 0-shell: Impossible to get your home dir!"),
    }
}