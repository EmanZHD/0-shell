use std::env;
use std::fs;
use crate::Params;
use crate::commands;
use std::path::PathBuf;
use std::collections::HashMap;
use crate::commands::ls::ls::ls;
use commands::cd::cd;
use commands::rm::rm;
use commands::cp::cp;
use commands::mv::mv;
use commands::pwd::pwd;
use commands::cat::cat;
use commands::exit::exit;
use commands::echo::echo;
use commands::help::help;
use commands::clear::clear;
use commands::mkdir::mkdir;
use commands::history::history;

/*********🌟 handle_cmds 🌟********/
pub fn handle_cmds(params: &mut Params, keyword: String) {
    // cas de "" keyword
    if keyword.is_empty() {
        println!("0-shell: permission denied:");
        return;
    }

    let mut dispatcher: HashMap<&str, fn(&mut Params)> = HashMap::new();
    dispatcher.insert("ls", ls as fn(&mut Params));
    dispatcher.insert("cd", cd as fn(&mut Params));
    dispatcher.insert("rm", rm as fn(&mut Params));
    dispatcher.insert("mv", mv as fn(&mut Params));
    dispatcher.insert("cp", cp as fn(&mut Params));
    dispatcher.insert("cat", cat as fn(&mut Params));
    dispatcher.insert("pwd", pwd as fn(&mut Params));
    dispatcher.insert("exit", exit as fn(&mut Params));
    dispatcher.insert("echo", echo as fn(&mut Params));
    dispatcher.insert("help", help as fn(&mut Params));
    dispatcher.insert("clear", clear as fn(&mut Params));
    dispatcher.insert("mkdir" , mkdir as fn(&mut Params));
    dispatcher.insert("history", history as fn(&mut Params));

    match dispatcher.get(&keyword.as_str()) {
        Some(func) => func(params),
        None => println!("0-shell: Command Not Found: {} ☹️", keyword),
    }
}


/*********🌟 get_paths 🌟********/
pub fn get_paths() -> PathBuf {
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

    if let Some(parent) = history_path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    return history_path
}