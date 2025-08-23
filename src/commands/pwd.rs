use std::env;
use std::io::{ self };

pub fn initial_pwd() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    let _path = current_dir.to_string_lossy();
    Ok(())
}

pub fn pwd(_args: Vec<&str>) {
    if let Err(e) = initial_pwd() {
        eprintln!("⛔ Error running pwd command: {}", e);
    }
}
