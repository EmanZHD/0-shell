mod parser;
mod consts;
mod commands;
use std::path::PathBuf;
use std::collections::HashMap;
use consts::{ TITLE, GREEN, RESET };
use parser::{ read_input, print_prompt };
use commands::ls::ls;
use commands::cd::cd;
use commands::pwd::pwd;
use commands::exit::exit;
use commands::echo::echo;
use commands::guide::guide;
use commands::clear::clear;
use commands::history::history;

#[derive(Clone, Debug)]
pub struct Params {
   args: Vec<String>,
   archieve: Vec<(i32 ,String)>,
   previous_path: Option<PathBuf>
}

impl Params {
    pub fn new() -> Self {
        Params {
            args: Vec::new(),
            archieve: Vec::new(),
            previous_path: None,
        }
    }
}
fn main() {
    println!("{GREEN}{}{RESET}", TITLE);
    let mut params = Params::new();
    let mut count = 1;
    loop {
        print_prompt();
        let (keyword, arguments) = read_input();
        if keyword.is_empty() && arguments.is_empty() {
            continue;
        }
        let valeur = format!("{} {}", keyword.clone(), arguments.join(" "));
        params.args = arguments;
        params.archieve.push((count, valeur));
        handle_cmds(&mut params, keyword);
        count+=1;
    }
}

pub fn handle_cmds(params: &mut Params, keyword: String) {
    let mut dispatcher: HashMap<&str, fn(&mut Params)> = HashMap::new();
    dispatcher.insert("ls", ls as fn(&mut Params)); 
    dispatcher.insert("cd", cd as fn(&mut Params)); 
    dispatcher.insert("pwd", pwd as fn(&mut Params));
    dispatcher.insert("exit", exit as fn(&mut Params));
    dispatcher.insert("echo", echo as fn(&mut Params));
    dispatcher.insert("guide", guide as fn(&mut Params));
    dispatcher.insert("clear", clear as fn(&mut Params));
    dispatcher.insert("history", history as fn(&mut Params));

    match dispatcher.get(&keyword.as_str()) {
        Some(func) => func(params),
        None => println!("0-shell: Command Not Found: {} ☹️", keyword),
    }
}