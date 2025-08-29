use std::fs;
use std::path::Path;
use crate::Params;
use crate::colors::{red, bold_gray, yellow, green, blue, bold_red, cyan};

// 🥳 here for check args if it's valid or not 🥳
pub fn mv(params: &mut Params) {
    if params.args.len() < 2 {
        eprintln!("{}{}{}", bold_red("👀 ​mv: need at least 2 arguments\n"), green("👍 ​Usage: mv file1 file2\n"),  green("👍 ​or: mv files... folder"));        
        return;
    }

    let (sources, destination) = params.args.split_at(params.args.len() - 1);
    let dest_path = Path::new(&destination[0]);
    let is_dest_dir = dest_path.is_dir();

    if sources.len() > 1 && !is_dest_dir {
        eprintln!("{}", cyan("😸​ mv: target is not a directory"));
        return;
    }

    for source in sources {
        if let Err(e) = move_file(source, &destination[0], is_dest_dir) {
            eprintln!( "{}", red(&format!("😸​ mv: cannot move '{}' ==> {}", yellow(source), e)));
        }
    }
}

// 💁‍♀️ here to move 💁‍♀️
fn move_file(source: &str, destination: &str, dest_is_dir: bool) -> Result<(), Box<dyn std::error::Error>> {
    let source_path = Path::new(source);
    
    if !source_path.exists() {
        return Err(bold_gray("No such file or directory").into());
    }

     if dest_is_dir {
        Path::new(destination).join(source_path.file_name().ok_or("Invalid filename")?)
    } else {
        Path::new(destination).to_path_buf()
    };

    match fs::rename(source, destination) {
        Ok(_) => {
            println!("{}", green(&format!("✓ Moved '{}' to '{}'", 
                yellow(source), 
                blue(destination)
            )));
        }
        Err(e) => {
            eprintln!("{}", red(&format!("✗ mv: {}", e)));
        }
    }
    Ok(())
}