use std::{ io };
use std::io::Write;
use std::env;
use colored::*;

/*********🌟 Current Dir 🌟********/
pub fn current() -> String {
    let result: String = match env::current_dir() {
        Ok(path) => {
            match path.file_name() {
                Some(file_name) => file_name.to_string_lossy().into_owned(),
                _none => String::from("/"),
            }
        }
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
  print!("> ");
  io::stdout().flush().unwrap();
}

/***********🌟 parsing 🌟**********/
fn parsing(input: &str) -> Result<Vec<String>, String> {
  let mut in_quotes = false;
  let mut new = Vec::new();
  let mut new_input = String::new();
  let mut quote = ' '; // pour memoriser le quote
    for c in input.chars() {
       match c {
          '\'' | '"' if !in_quotes => {
                in_quotes = true;
                quote = c; // pour memoriser le type de quote
          }
          '\'' | '"' if in_quotes && c == quote => {
              in_quotes = false; // fermeture de la quote du m type
          }
          ' ' | '\t' if !in_quotes => {
             if !new_input.is_empty() {
                new.push(new_input);
                new_input = String::new();
              }
          }
          _ => {
              new_input.push(c);
          }
        }
      }
      if in_quotes {
         return Err("unclosed quotes 😓".to_string());
      }

      if !new_input.is_empty() {
          new.push(new_input);
      }
      Ok(new)
  } 

/*********🌟 read_input 🌟********/
pub fn read_input() -> (String, Vec<String>) {
    let mut cmd = String::new();
    
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        let input = input.trim_end();
        //println!("✅ Input: {:?}", input);
        
        if cmd.is_empty() {
            cmd = input.to_string();
        } else {
            cmd = format!("{}\n{}", cmd, input);
            // println!("👽 else 88: {:?}", cmd);
        }
        
        //println!("✅ Command line: {:?}", cmd);
        
        match parsing(&cmd) {
            Ok(elements) => {
                if elements.is_empty() {
                    return (String::new(), Vec::new());
                }
                
                let command = elements[0].clone();
                let args = if elements.len() > 1 {
                    elements[1..].to_vec()
                } else {
                    Vec::new()
                };
                
                // println!("✅ Command: {:?}", command);
                // println!("✅ Arguments: {:?}", args);
                
                return (command, args);
            }
            Err(_) => {
                print_quote_prompt();
            }
        }
    }
}
