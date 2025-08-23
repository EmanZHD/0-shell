use std::{io};
use std::io::Write;
use std::env;
use colored::*;

/*********🌟 Current Dir 🌟********/
fn current() -> String {
  let result: String = match env::current_dir() {
    Ok(path) => {
      match path.file_name() {
        Some(file_name) => file_name.to_string_lossy().into_owned(),
        None => String::from("/"),
      }
    },
    Err(_e) => "/".to_string(),
  };
  result
}

/*********🌟 print_prompt 🌟********/
pub fn print_prompt() {
  print!("{}", "~".bold().yellow());
  print!("{}", current().bold().truecolor(199, 21, 133));
  print!("{} ", "$".bold().yellow());
  io::stdout().flush().unwrap();
}

/*********🌟 read_input 🌟********/
pub fn read_input() -> (String, Vec<String>) {
    let mut cmd = String::new();
    io::stdin().read_line(&mut cmd).expect("Failed to read in command");
    println!("✅ Verification: cmd: {:?}", cmd);
    let cms: Vec<String> = cmd.split_whitespace().map(|s| s.to_string()).collect();
    let keyword = cms[0].to_string();
    let arguments = cms[1..].to_vec();
    println!("✅ Verification: keyword: {:?}", keyword);
    println!("✅ Verification: arguments: {:?}", arguments);
    (keyword, arguments)
}