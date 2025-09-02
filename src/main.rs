mod parser;
mod consts;
mod commands;
use std::env;
use parser::*;
use std::path::PathBuf;
use std::collections::HashMap;
use consts::{ TITLE, GREEN, RESET };

use commands::ls::ls;
use commands::cd::cd;
use commands::pwd::pwd;
use commands::exit::exit;
use commands::echo::echo;
use commands::help::help;
use commands::clear::clear;
use commands::history::history;

#[derive(Clone, Debug)]
pub struct Params {
    args: Vec<String>,
    archieve: PathBuf,
    previous_path: Option<PathBuf>,
    home: PathBuf,

}

impl Params {
    pub fn new() -> Self {
        Params {
            args: Vec::new(),
            archieve: PathBuf::new(),
            previous_path: None,
            home: PathBuf::new(),
        }
    }
}
fn main() {
    println!("{GREEN}{}{RESET}", TITLE);
    let mut params = Params::new();
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
    
    let home = match env::home_dir() {
        Some(home_dir) => {
            let path = home_dir; 
            path
        }
        None => {
          PathBuf::new()
        }
    };
    params.archieve = history_path.clone();
    params.home = home.clone();

    loop {
        print_prompt();
        let (keyword, arguments) = read_input(history_path.clone());
        if keyword.is_empty() && arguments.is_empty() {
            continue;
        }
        params.args = arguments;
        handle_cmds(&mut params, keyword);
    }
}

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
