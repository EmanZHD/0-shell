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
  let begin = format!("{}{}{} ", "~".bold().yellow(), current().bold().truecolor(199, 21, 133), "$".bold().yellow());
  print!("{}", begin);
  io::stdout().flush().unwrap();
}

/*********🌟 print_quote_prompt 🌟********/
pub fn print_quote_prompt() {
  print!("quote> ");
  io::stdout().flush().unwrap();
}

/*********🌟 print_quote_prompt 🌟********/
// fn parsing(input: String) {
//   let mut in_quotes = false;
//   let mut new_input = String::new();
//   let mut quote = ' '; // bach nchad lquote
//     for c in input.chars() {
//        match c {
//           '\'' | '"' if !in_quotes => {
//                 in_quotes = true;
//                 quote = ch; // pour memoriser le type de quote
//           }
//           '\'' | '"' if in_quotes && ch == quote => {
//               in_quotes = false; // fermeture de la quote du m type
//           }
//           ' ' | '\t' if !in_quotes => {
//              if !new_input.is_empty() {
//                 tokens.push(current_token);
//                 new_input = String::new();
//               }
//           }
//           _ => {
//               new_input.push(ch);
//           }
//        }
//     }

// }

/*********🌟 read_input 🌟********/
pub fn read_input() -> (String, Vec<String>) {
    let mut cmd = String::new();
    
    io::stdin().read_line(&mut cmd).expect("Failed to read in command");
    println!("✅ Verification: cmd: {:?}", cmd);
    let cms: Vec<String> = cmd.split_whitespace().map(|s| s.to_string()).collect();
    if !cms.is_empty() {
      let keyword = cms[0].to_string();
      let arguments = cms[1..].to_vec();
      println!("✅ Verification: keyword: {:?}", keyword);
      println!("✅ Verification: arguments: {:?}", arguments);
      (keyword, arguments)
    } else {
      println!("✅ Verification: Input is empty");
      ("".to_string(), Vec::new())
    }
    
}