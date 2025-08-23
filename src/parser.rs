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
pub fn read_input() -> String {
    let mut cmd = String::new();
    io::stdin().read_line(&mut cmd).expect("Failed to read in command");
    println!("✅ Verification: cmd: {:?}", cmd);
    cmd
}