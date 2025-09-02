use std::env;
use crate::Params;
use crate::commands;
use std::path::PathBuf;
use std::collections::HashMap;

use commands::ls::ls;
use commands::cd::cd;
use commands::pwd::pwd;
use commands::exit::exit;
use commands::echo::echo;
use commands::help::help;
use commands::clear::clear;
use commands::history::history;

/*********🌟 handle_cmds 🌟********/
pub fn handle_cmds(params: &mut Params, keyword: String) {
    let mut dispatcher: HashMap<&str, fn(&mut Params)> = HashMap::new();
    dispatcher.insert("ls", ls as fn(&mut Params));
    dispatcher.insert("cd", cd as fn(&mut Params));
    dispatcher.insert("pwd", pwd as fn(&mut Params));
    dispatcher.insert("exit", exit as fn(&mut Params));
    dispatcher.insert("echo", echo as fn(&mut Params));
    dispatcher.insert("help", help as fn(&mut Params));
    dispatcher.insert("clear", clear as fn(&mut Params));
    dispatcher.insert("history", history as fn(&mut Params));

    match dispatcher.get(&keyword.as_str()) {
        Some(func) => func(params),
        None => println!("0-shell: Command Not Found: {} ☹️", keyword),
    }
}


/*********🌟 get_paths 🌟********/
pub fn get_paths() -> (PathBuf, PathBuf) {
    // get history file path
    let history_path = match env::current_dir() {
        Ok(home_dir) => {
          let mut path = home_dir;
          path.push("history/0-shell_history");
          path
        }
        Err(_) => {
            PathBuf::from("0-shell_history")
        }
    };
    
    // get home_directory path
    let home = match env::home_dir() {
        Some(home_dir) => {
            let path = home_dir; 
            path
        }
        None => {
           let root = PathBuf::from("/");
           root
        }
    };
    return (history_path, home)
}