use std::fs;
use std::io::{self, Read};

// 🥳​ here check input if have argument ymchi --> cat_file, makanch ya3na dar gha cat aymci --> only_cat 🥳​
pub fn cat(args: Vec<String>) {
    if args.is_empty() {
        only_cat();
    } else {
        for filename in &args {
            if let Err(e) = cat_file(filename) {
                eprintln!("cat: {}: {} ☹️", filename, e);
            }
        }
    }
}

// 💁‍♀️​ handle only cat 💁‍♀️​
fn only_cat() {
    println!("Reading from stdin (Ctrl+D to end) ☺️​:");
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    print!("{}", buffer);
}

// 💁‍♀️​ handle cat + plusieurs arg(files) 💁‍♀️​
fn cat_file(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(filename)?;
    println!("{}", contents);
    Ok(())
}