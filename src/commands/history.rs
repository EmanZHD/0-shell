use crate::Params;
use std::{fs::File, path::PathBuf};
use colored::Colorize;
use std::io::{self, BufReader, BufRead};

/***********🌟 history 🌟**********/
pub fn history(parameters: &mut Params) {
   if let Err(_e) = read_history_file(parameters.archieve.clone()) {
      println!("⛔ 0-shell: Error Loading History, Try Later 🙃");
   }
}

/*********🌟 read_history_file 🌟********/
fn read_history_file(arch: PathBuf) -> io::Result<()> {
    let content = File::open(arch.clone())?;
    let reader = BufReader::new(content);
    let lenght = reader.lines().count().to_string();

    let content = File::open(arch.clone())?;
    let reader = BufReader::new(content);
    println!("{}" ,"📜 This is your history 🤗\n".bold());
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