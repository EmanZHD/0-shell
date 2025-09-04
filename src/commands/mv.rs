use std::fs;
use std::path::Path;
use crate::Params;
use crate::colors::{red, bold_gray, yellow, green, blue, bold_red};

// 🥳 here for check args if it's valid or not 🥳
pub fn mv(params: &mut Params) {
    if params.args.len() == 0 {
        eprintln!("{}", bold_red("👀 mv: missing file operand"));        
        return;
    }
    let (sources, des) = params.args.split_at(params.args.len() - 1);
    let dest_path = Path::new(&des[0]);
    let is_dest_dir = dest_path.is_dir();
    if params.args.len() < 2 {
        eprintln!("{} '{}'", bold_red("👀 mv: missing destination file operand after"), green(&des[0]));        
        return;
    }

    if sources.len() > 1 && !is_dest_dir {
        eprintln!("{} '{}' {}", bold_red("😸​ mv: target"),yellow(&des[0]) , bold_red("is not a directory"));
        return;
    }

    for source in sources {
        if source == &des[0] && is_dest_dir {
            eprintln!("{}", red(&format!("😸​ mv: cannot move '{}' to a subdirectory of itself, '{}/{}'", yellow(source), yellow(&des[0]), yellow(&des[0]))));
        } else if source == &des[0] {
            eprintln!("{}", red(&format!("😸​ mv: '{}' and '{}' are the same file", yellow(source), yellow(&des[0]))));
        } else if &des[0] == "."  {
            eprintln!("{}", red(&format!("😸​ mv: '{}' and '{}/{}' are the same file", yellow(source), yellow(&des[0]), yellow(source))));
        } else {
            let _ = move_file(source, &des[0], is_dest_dir);
        }
    }
}

// 💁‍♀️ here to move 💁‍♀️
fn move_file(source: &str, des: &str, dest_is_dir: bool) -> Result<(), Box<dyn std::error::Error>> {
    let source_path = Path::new(source);
    
    if !source_path.exists() {
        eprintln!("{}", red(&format!("😸​ mv: cannot stat '{}': {}", yellow(source), bold_gray("No such file or directory"))));
    }

    let dest_path = if dest_is_dir {
        Path::new(des).join(source_path.file_name().ok_or("Invalid filename")?)
    } else {
        Path::new(des).to_path_buf()
    };

    match fs::rename(source, &dest_path) {
        Ok(_) => {
            println!("{}", green(&format!("✓ Moved '{}' to '{}'", 
                yellow(source), 
                blue(&dest_path.display().to_string()) 
            )));
            Ok(())
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                eprintln!("{}", red(&format!("😸​ mv: cannot move '{}' to '{}' : Permission denied", source, des)));
            }
            Err(e.into())
        }
    }
}