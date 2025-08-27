use std::path::Path;
use std::env;
use std::env::set_current_dir;
use crate::Params;

pub fn cd(parameters: &mut Params) {
    if parameters.args.len() == 0 {
        go_to_home()
    } else {
        if &parameters.args[0] == "~" || &parameters.args[0] == "--" {
            go_to_home();
            return;
        }
        if parameters.args.len() == 1 && &parameters.args[0] == "-" {
            if let Some(prev_dir) = parameters.previous_path.take() {
                let current_dir = env::current_dir().ok();
                if let Err(e) = env::set_current_dir(&prev_dir) {
                    eprintln!("0-shell: cd: {}: {}", prev_dir.display(), e);
                    parameters.previous_path = current_dir.into();
                } else {
                    println!("{}", prev_dir.display());
                    parameters.previous_path = current_dir;
                    println!("🚨 => {:?}", parameters.previous_path);
                }
            } else {
                parameters.previous_path = env::current_dir().ok();
            }
            return;
        }
            let path = Path::new(&parameters.args[0]);
            if let Err(_e) = set_current_dir(&path) {
                eprintln!("⛔ 0-shell: No such file or directory {:?}", path);
            }
    }
}

/*********🌟 go_to_home 🌟********/
fn go_to_home() {
    match env::home_dir() {
        Some(path) => {
            if let Err(_e) = set_current_dir(path) {
                println!("⛔ 0-shell: No such file or directory");
            }
        }
        None => println!("⛔ 0-shell: Impossible to get your home dir! 🫤"),
    }
}
