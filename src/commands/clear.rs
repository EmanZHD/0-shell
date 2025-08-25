use std::io;
use std::io::Write;

fn prepare_clear() -> Result<(), String> {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().map_err(|e| e.to_string())?;
    Ok(())
}

pub fn clear(args: Vec<String>) {
    if args.len() > 0 {
       println!("clear: too many arguments ‼️");
       return;
    }
    if let Err(e) = prepare_clear() {
        eprintln!("Erreur: {}", e);
    }
}

/* note :
\x1B[2J : Efface tout l'ecran
\x1B[1;1H : Place le curseur en position 1,1
*/