mod parser;
mod commands;
use parser::{ read_input, print_prompt };
// use commands::echo::echo;
use commands::pwd::pwd;
use commands::exit::exit;
use commands::man::man;
// use commands::cd::cd;


fn main() {
    println!("🥰 Welcome to our mini-shell 100% Girly 💋");
    loop {
        print_prompt();
        let cmd = read_input();
        let cms: Vec<&str>= cmd.split(' ').collect();
        let keyword = cms[0];
        println!("✅ Verification: Keyword: {0}", keyword);
        let arguments = &cms[1..];
        println!("✅ Verification: Number of arguments: {0:?}", arguments.len());
        match keyword.trim() {
            // "echo" => echo(arguments),
            "pwd" => pwd(),
            "exit" => exit(),
            "man" => man(),
            // "cd" => cd(arguments),
            _ => println!("0-shell: Command Not Found: {} ☹️", keyword.trim())
        }
    }
}