mod parser;
mod consts;
mod commands;
use std::collections::HashMap;
use consts::{ TITLE, GREEN, RESET };
use parser::{ read_input, print_prompt };
// use commands::echo::echo;
use commands::cd::cd;
use commands::ls::ls;
use commands::pwd::pwd;
use commands::exit::exit;
use commands::clear::clear;
use commands::guide::guide;
use commands::history::history;
use commands::cat::cat;

fn main() {
    println!("{GREEN}{}{RESET}", TITLE);
    let mut historique: Vec<(i32 ,String)> = Vec::new();
    let mut count = 1;

    loop {
        print_prompt();
        let (keyword, arguments) = read_input();
        let valeur = format!("{} {}", keyword.clone(), arguments.join(" "));
        historique.push((count, valeur));
        handle_cmds(keyword, arguments, &mut historique);
        count+=1;
    }
}

pub fn handle_cmds(keyword: String, arguments: Vec<String>, historique: &mut Vec<(i32, String)>) {
    if keyword == "history" {
        history(historique);
        return;
    }
    let mut dispatcher: HashMap<&str, fn(Vec<String>)> = HashMap::new();
    dispatcher.insert("cd", cd); 
    dispatcher.insert("pwd", pwd);
    dispatcher.insert("exit", exit);
    dispatcher.insert("guide", guide);
    dispatcher.insert("clear", clear);
    dispatcher.insert("cd", cd); 
    dispatcher.insert("ls", ls);
    dispatcher.insert("cat", cat); 

    match dispatcher.get(&keyword.as_str()) {
        Some(func) => func(arguments),
        None => println!("0-shell: Command Not Found: {} ☹️", keyword),
    }
}