use std::io;
use std::io::Write;

use crate::Params;

fn prepare_clear() -> Result<(), String> {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().map_err(|e| e.to_string())?;
    Ok(())
}

pub fn clear(parameters: &mut Params) {
    if parameters.args.len() > 0 {
       eprintln!("clear: too many arguments ‼️");
       return;
    }
    if let Err(e) = prepare_clear() {
        eprintln!("Erreur: {}", e);
    }
}

/* note :
\x1B[2J :  "Erase Display" efface tout le contenu visible de l'ecran
\x1B[1;1H :  "Cursor Position" deplace le curseur à la ligne 1, colonne 1 
        (le coin superieur gauche du terminal).
*/