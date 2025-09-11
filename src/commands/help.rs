use crate::Params;

pub fn help(_parameters: &mut Params) {
    println!("\x1b[1m📌 Commands:\x1b[0m");
    println!("\t\x1b[1m\x1b[34mcd:\x1b[0m Changes the current directory.");
    println!("\t\x1b[1m\x1b[35mhelp:\x1b[0m Prints commands descriptions");
    println!("\t\x1b[1m\x1b[31mexit:\x1b[0m Closes the 0-shell and all of it's processes.");
    println!("\t\x1b[1m\x1b[33mcat:\x1b[0m Displays the content of a file.");
    println!("\t\x1b[1m\x1b[36mecho:\x1b[0m Displays defined text.");
    println!("\t\x1b[1m\x1b[32mls:\x1b[0m Lists directory contents.");
    println!("\t\t\x1b[1m\x1b[32mls -a:\x1b[0m List all files, including hidden ones.");
    println!("\t\t\x1b[1m\x1b[32mls -l:\x1b[0m List files in long format (with details).");
    println!("\t\t\x1b[1m\x1b[32mls -F:\x1b[0m List files and mark types with symbols.");
    println!("\t\x1b[1m\x1b[34mmkdir:\x1b[0m Creates a new directory.");
    println!("\t\x1b[1m\x1b[30mmv:\x1b[0m Moves or renames files or directories.");
    println!("\t\x1b[1m\x1b[38;5;218mpwd:\x1b[0m Prints the current working directory.");
    println!("\t\x1b[1m\x1b[31mrm:\x1b[0m Removes files or directories.");
    println!("\t\t\x1b[1m\x1b[31mrm -r:\x1b[0m remove a directory and all its contents recursively.");
    println!("\t\x1b[1m\x1b[96mcp:\x1b[0m Copies files or directories.");
    println!("\t\x1b[1m\x1b[35mhistory:\x1b[0m Displays an enumerated list with the commands you’ve used in the past");
}
