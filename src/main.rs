mod parser;
mod shell;
mod consts;
mod commands;
use shell::*;
use parser::*;
mod colors;
use std::path::PathBuf;
use consts::{ TITLE, GREEN, RESET };
use commands::mv::mv;
use commands::cat::cat;
use commands::rm::rm;
use commands::mkdir::mkdir;
use commands::cp::cp;

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
    if !atty::is(atty::Stream::Stdout) || !atty::is(atty::Stream::Stderr) {
        eprintln!("Error: Avoid broken pipe");
        std::process::exit(1);
    }

    println!("{GREEN}{}{RESET}", TITLE);
    let mut params = Params::new();
    let (history_path, home) = get_paths();
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
    dispatcher.insert("guide", guide as fn(&mut Params));
    dispatcher.insert("clear", clear as fn(&mut Params));
    dispatcher.insert("history", history as fn(&mut Params));
    dispatcher.insert("mv", mv as fn(&mut Params));
    dispatcher.insert("cat", cat as fn(&mut Params));
    dispatcher.insert("rm", rm as fn(&mut Params));
    dispatcher.insert("mkdir" , mkdir as fn(&mut Params));
    dispatcher.insert("cp" , cp as fn (&mut Params));

    match dispatcher.get(&keyword.as_str()) {
        Some(func) => func(params),
        None => println!("0-shell: Command Not Found: {} ☹️", keyword),
    }
}