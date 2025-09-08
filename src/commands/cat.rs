use std::fs;
use crate::Params;
use crate::colors::{ bold_red, cyan };

//  🥳
pub fn cat(params: &mut Params) {
    if params.args.is_empty() {
        if let Err(e) = only_cat() {
            eprintln!("☹️ cat: stdin: {} ", e);
        }
    } else {
        for filename in &params.args {
            // println!("heere == > {:?}", filename.len());
            if params.args.len() == 1 && filename == "-s" {
                let _ = only_cat();
            } else if filename == "-s" {
                continue;
            } else {
                if let Err(e) = cat_file(filename) {
                    eprintln!("{}", bold_red(&format!("☹️ cat: '{}': {} ", filename, e)));
                }
            }
        }
    }
}

// 💁‍♀️​ handle only cat 💁‍♀️​
fn only_cat() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", cyan("☺️​ Reading from stdin (Ctrl+D to end) :"));
    let mut rl = match rustyline::DefaultEditor::new() {
        Ok(editor) => editor,
        Err(e) => {
            return Err(Box::new(e));
        }
    };
    loop {
        let input = rl.readline(&cyan("🌸 "));
        match input {
            Ok(ref content) => {
                println!("🌸 {}", content);
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                break;
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                break;
            }
            Err(e) => {
                return Err(Box::new(e));
            }
        }
    }
    Ok(())
}

// 💁‍♀️​ handle cat + plusieurs arg(files) 💁‍♀️​
fn cat_file(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    if filename == "-" || filename == "--" {
        return only_cat();
    }
    match fs::read(filename) {
        Ok(contents) => {
            println!("{}", String::from_utf8_lossy(&contents));
            Ok(())
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                return Err("Permission denied".into());
            } else if e.kind() == std::io::ErrorKind::NotFound {
                return Err("No such file or directory".into());
            } else {
                eprintln!("cat: {}: {}", filename, e);
            }
            Err(e.into())
        }
    }
}
