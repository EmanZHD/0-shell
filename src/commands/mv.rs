// commands/mv.rs
use std::fs;
use std::path::Path;

pub fn mv(args: Vec<String>) {
    if args.len() < 2 {
 eprintln!("👀 ​mv: need at least 2 arguments\n👍 ​Usage: mv file1 file2\n👍 ​or: mv files... folder");        return;
    }

    let (sources, destination) = args.split_at(args.len() - 1);
    let dest_path = Path::new(&destination[0]);
    let is_dest_dir = dest_path.is_dir();

    if sources.len() > 1 && !is_dest_dir {
        eprintln!("😸​ mv: target '{}' is not a directory", destination[0]);
        return;
    }

    for source in sources {
        if let Err(e) = move_file(source, &destination[0], is_dest_dir) {
            eprintln!("😸​ mv: cannot move '{}': {}", source, e);
        }
    }
}

fn move_file(source: &str, destination: &str, dest_is_dir: bool) -> Result<(), Box<dyn std::error::Error>> {
    let source_path = Path::new(source);
    
    if !source_path.exists() {
        return Err("😸​ No such file or directory".into());
    }

    let dest_path = if dest_is_dir {
        Path::new(destination).join(source_path.file_name().ok_or("Invalid filename")?)
    } else {
        Path::new(destination).to_path_buf()
    };

    fs::rename(source_path, dest_path)?;
    Ok(())
}