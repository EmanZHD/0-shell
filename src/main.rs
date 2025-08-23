use std::collections::HashMap;
mod parser;
mod commands;
mod consts;
use parser::{ read_input, print_prompt };
use consts::{ TITLE, GREEN, RESET };
// use commands::echo::echo;
use commands::pwd::pwd;
use commands::exit::exit;
use commands::guide::guide;
use commands::cd::cd;

fn main() {
    println!("{GREEN}{}{RESET}", TITLE);

    loop {
        print_prompt();
        let (keyword, arguments) = read_input();
        handle_cmds(keyword, arguments);
    }
}

pub fn handle_cmds(keyword: String, arguments: Vec<String>) {
    let mut dispatcher: HashMap<&str, fn(Vec<String>)> = HashMap::new();
    dispatcher.insert("pwd", pwd);
    dispatcher.insert("exit", exit);
    dispatcher.insert("guide", guide);
    dispatcher.insert("cd", cd); 

    match dispatcher.get(&keyword.as_str()) {
        Some(func) => func(arguments),
        None => println!("0-shell: Command Not Found: {} ☹️", keyword),
    }
}