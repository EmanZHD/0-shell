use std::fs;
use crate::Params;
use std::io::{self};
use std::io::{BufReader, BufRead};
use crate::colors::{bold_red, cyan};

// 🥳 Updated to work with Params instead of Vec<String> 🥳
pub fn cat(params: &mut Params) {
    if params.args.is_empty() {
        if let Err(e) = only_cat() {
            eprintln!("cat: stdin: {} ☹️", e);
        }
    } else {
        for filename in &params.args {
            if let Err(e) = cat_file(filename) {
                eprintln!("{}", bold_red(&format!("cat: {}: {} ☹️", filename, e)));
            }
        }
    }
}

// 💁‍♀️​ handle only cat 💁‍♀️​
fn only_cat() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", cyan("Reading from stdin (Ctrl+D to end) ☺️​:"));
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());
    
    for line in reader.lines() {
        match line {
             Ok(content) => {
                println!("{}", content);
            }
            Err(e) => return Err(Box::new(e)),
        }
    }
    Ok(())
}

// 💁‍♀️​ handle cat + plusieurs arg(files) 💁‍♀️​
fn cat_file(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    if filename == "-" {
        return only_cat();
    }
    let contents = fs::read_to_string(filename)?;
    println!("{}", contents);
    Ok(())
}