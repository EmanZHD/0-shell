use std::collections::HashMap;
mod parser;
mod commands;
mod consts;
use parser::{ read_input, print_prompt };
use consts::{ TITLE, GREEN, RESET };
// use commands::echo::echo;
use commands::pwd::pwd;
use commands::exit::exit;
use commands::man::man;
// use commands::cd::cd;

fn main() {
    println!("{GREEN}{}{RESET}", TITLE);

    let mut dispatcher: HashMap<&str, fn(Vec<&str>)> = HashMap::new();
    dispatcher.insert("pwd", pwd);
    dispatcher.insert("exit", exit);
    dispatcher.insert("man", man);
    // println!("MAP {:?}", dispatcher);
    loop {
        print_prompt();
        let cmd = read_input();
        let cms: Vec<&str> = cmd.split_whitespace().collect();

        let keyword = cms[0];
        println!("✅ Verification: Keyword: {0}", keyword);
        let arguments = cms[1..].to_vec();
        println!("✅ Verification: Number of arguments: {0:?}", arguments.len());

        match dispatcher.get(keyword) {
            Some(func) => func(arguments),
            None => println!("0-shell: Command Not Found: {} ☹️", keyword),
        }
    }
}
