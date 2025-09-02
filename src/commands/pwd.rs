use std::env::{self};
use std::io::{ self };
use crate::Params;

pub fn initial_pwd() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    let path = current_dir.to_string_lossy();
    println!("{}", path);
    Ok(())
}

pub fn pwd(parameters: &mut Params) {
    if parameters.args.len() > 0 {
       println!("pwd: too many arguments ‼️");
       return;
    }
    if let Err(_e) = initial_pwd() {
        println!("⛔ Error running pwd command");
    }
}
