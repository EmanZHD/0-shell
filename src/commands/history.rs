use crate::Params;
use std::fs::File;
use colored::Colorize;
use std::io::{self, BufReader, BufRead};

/***********🌟 history 🌟**********/
pub fn history(_parameters: &mut Params) {
   //println!("📜 This is your history 🤗");
   // let lenght = parameters.archieve.len().to_string();
   // for col in &parameters.archieve {
   //    let espace = lenght.len() - col.0.to_string().len();
   //    let result= format!("{}{} {}", " ".repeat(espace), col.0, col.1);
   //    println!("{}", result);
   // } 
   
   if let Err(_e) = read_history_file() {
      println!("⛔ 0-shell: Error Loading History, Try Later 🙃");
   }
}

/*********🌟 read_history_file 🌟********/
fn read_history_file() -> io::Result<()> {
    let content = File::open("../0-shell/history/0-shell_history")?;
    let reader = BufReader::new(content);
    let lenght = reader.lines().count().to_string();

    let content = File::open("../0-shell/history/0-shell_history")?;
    let reader = BufReader::new(content);
   //  println!("{}" ,"📜 This is your history 🤗\n".bold());
    for (n, line) in reader.lines().enumerate() {
         if n == 0 {
            continue;
         }
         let espace = lenght.len() - n.to_string().len();
         let result = format!("\t\x1b[1m\x1b[38;5;218m{}{}.\x1b[0m  {}", " ".repeat(espace), n, line.unwrap().bold());
         println!("{}", result);
    }
    Ok(())
}