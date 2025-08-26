use std::env::{self};
use std::io::{ self };

pub fn initial_pwd() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    let path = current_dir.to_string_lossy();
    println!("{}", path);
    Ok(())
}

pub fn pwd(args: Vec<String>) {
    if args.len() > 0 {
       println!("pwd: too many arguments ‼️");
       return;
    }
    if let Err(e) = initial_pwd() {
        eprintln!("⛔ Error running pwd command: {}", e);
    }
}
