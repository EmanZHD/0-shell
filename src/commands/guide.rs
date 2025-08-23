pub fn guide(_args: Vec<&str>) {
    println!("Commands:");
    println!("\t\x1b[1m\x1b[34mcd:\x1b[0m Changes the current directory.");
    println!("\t\x1b[1m\x1b[35mman:\x1b[0m Prints built in commands descriptions");
    println!("\t\x1b[1m\x1b[31mexit:\x1b[0m Closes the 0-shell and all of it's processes.\x1b[0m");
    println!("\t\x1b[1m\x1b[33mcat:\x1b[0m Displays the content of a file.");
    println!("\t\x1b[1m\x1b[36mecho:\x1b[0m Displays defined text.");
    println!("\t\x1b[1m\x1b[32mls:\x1b[0m Lists directory contents.");
    println!("\t\x1b[1m\x1b[93mmkdir:\x1b[0m Creates a new directory.");
    println!("\t\x1b[1m\x1b[30mmv:\x1b[0m Moves or renames files or directories.");
    println!("\t\x1b[1m\x1b[94mpwd:\x1b[0m Prints the current working directory.");
    println!("\t\x1b[1m\x1b[31mrm:\x1b[0m Removes files or directories.");
    println!("\t\x1b[1m\x1b[96mcp:\x1b[0m Copies files or directories.");
}
