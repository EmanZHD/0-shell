use std::fs;
use std::io::{self, Read};
use crate::colors::{red, bold_gray, yellow, green, blue, bold_red, cyan};

// 🥳​ here check input if have argument ymchi --> cat_file, makanch ya3na dar gha cat aymci --> only_cat 🥳​
pub fn cat(args: Vec<String>) {
    if args.is_empty() {
        if let Err(e) = only_cat() {
            eprintln!("cat: stdin: {} ☹️", e);
        }
    } else {
        for filename in &args {
            if let Err(e) = cat_file(filename) {
                eprintln!("{}", bold_red(&format!("cat: {}: {} ☹️", filename, e)));
            }
        }
    }
}

// 💁‍♀️​ handle only cat 💁‍♀️​
fn only_cat() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", cyan("Reading from stdin (Ctrl+D to end) ☺️​:"));
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    print!("{}", (buffer));
    Ok(())
}

// 💁‍♀️​ handle cat + plusieurs arg(files) 💁‍♀️​
fn cat_file(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    if filename == "-" {
        return only_cat() ;
    }
    let contents = fs::read_to_string(filename)?;
    println!("{}", contents);
    Ok(())
}