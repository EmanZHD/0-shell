use std::path::Path;
use std::env;
use std::env::set_current_dir;
use crate::Params;

/*********🌟 cd 🌟********/
pub fn cd(parameters: &mut Params) {
    let current_dir = env::current_dir().ok();
    if parameters.args.is_empty() {
        go_to_home();
        parameters.previous_path = current_dir;
        return;
    }
    
    match parameters.args[0].as_str() {
       "--" => {
           go_to_home();
           parameters.previous_path = current_dir;  
       }
       "-" => {
           if let Some(precedent) = parameters.previous_path.take() {
              if let Err(e) = env::set_current_dir(&precedent) {
                 eprintln!("0-shell: cd: {}: {}", precedent.display(), e);
                 parameters.previous_path = Some(precedent); // En cas de probleme, nous reviendrons au chemin precedent
              }else {
                 println!("{}", precedent.display());
                 parameters.previous_path = current_dir;
              }
           }else {
               if let Some(current) = &current_dir {
                  println!("{}", current.display());
               }
               parameters.previous_path = current_dir;
           }
       }
        path => {
          let new_path = Path::new(path);
            if let Err(_e) = set_current_dir(&new_path) {
                eprintln!("⛔ 0-shell: No such file or directory {:?} 🫤", new_path);
            }else {
                parameters.previous_path = Some(path.into());
            }
        }
    }
}

/*********🌟 go_to_home 🌟********/
fn go_to_home() {
    match env::home_dir() {
        Some(path) => {
            if let Err(_e) = set_current_dir(path) {
                eprintln!("⛔ 0-shell: No such file or directory");
            }
        }
        None => eprintln!("⛔ 0-shell: Impossible to get your home dir! 🫤"),
    }
}
