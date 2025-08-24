use std::process;

pub fn exit(_args: Vec<String>) {
    println!("Leaving 0-shell. Goodbye 😁");
    process::exit(1);
}
