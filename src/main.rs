mod parser;
mod shell;
mod consts;
mod commands;
use shell::*;
use parser::*;
mod colors;
use std::path::PathBuf;
use consts::{ TITLE, GREEN, RESET };

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
        print_prompt(&params);
        let (keyword, arguments) = read_input(history_path.clone(), &params);
        if keyword.is_empty() && arguments.is_empty() {
            continue;
        }
        params.args = arguments;
        handle_cmds(&mut params, keyword);
    }
}
