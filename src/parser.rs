use std::io;
use std::io::Write;

/*********🌟 print_prompt 🌟********/
pub fn print_prompt() {
  let begin = "$";
  
  print!("{0} ", begin);
  io::stdout().flush().unwrap();
}

/*********🌟 read_input 🌟********/
pub fn read_input() -> String {
    let mut cmd = String::new();
    io::stdin().read_line(&mut cmd).expect("Failed to read in command");
    println!("✅ Verification: cmd: {:?}", cmd);
    cmd
}