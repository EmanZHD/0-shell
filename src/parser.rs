use colored::*;
use std::io;
use std::env;
use std::process;
use crate::Params;
use std::io::Write;
use std::path::PathBuf;

/*********🌟 Current Dir 🌟********/
pub fn current(params: &Params) -> String {
    let result: String = match env::current_dir() {
        Ok(path) => match path.file_name() {
            Some(file_name) => file_name.to_string_lossy().into_owned(),
            _none => String::from("/"),
        },
        Err(_e) => match &params.previous_path {
            Some(p)=> match p.file_name() {
               Some(file_name) => file_name.to_string_lossy().into_owned(),
               _none => String::from(""),
            },
            None=> '/'.to_string(),
        },
    };
    result
}

/*********🌟 print_prompt 🌟********/
pub fn print_prompt(params: &Params) {
    let begin = format!(
        "{}{}{} ",
        "~".bold().yellow(),
        current(params).bold().truecolor(199, 21, 133),
        "$".bold().yellow()
    );
    print!("{}", begin);
    match io::stdout().flush() {
        Ok(()) => {
            return;
        }
        _ => println!("⛔ broken pipe"),
    }
}

/*********🌟 print_quote_prompt 🌟********/
pub fn print_quote_prompt() {
    print!("> ");
    io::stdout().flush().unwrap();
}

/***********🌟 parsing 🌟**********/
fn parsing(input: &str) -> Result<Vec<String>, String> {
    let mut in_quotes = false;
    let mut new = Vec::new();
    let mut new_input = String::new();
    let mut quote = ' '; // pour memoriser les guillemets
    let mut peek_input = input.chars().peekable();
    let mut found_quotes = false; // Pour savoir si on a trouvé des guillemets
    while let Some(c) = peek_input.next() {
        match c {
            '\\' if peek_input.peek() == Some(&'\"') || peek_input.peek() == Some(&'\'') => {
                new_input.push(peek_input.next().unwrap());
                continue;
            }

            '\'' | '"' if !in_quotes => {
                in_quotes = true;
                quote = c;
                found_quotes = true;
            }

            '\'' | '\"' if in_quotes && c == quote => {
                in_quotes = false; // fermeture des guillemets du m type
                // Ajouter new_input même s'il est vide (cas des "")
                new.push(new_input.clone());
                new_input = String::new();
                found_quotes = false;
            }
            ' ' | '\t' if !in_quotes => {
                if !new_input.is_empty() || found_quotes {
                    new.push(new_input);
                    new_input = String::new();
                    found_quotes = false;
                }
            }
            _ => {
                new_input.push(c);
            }
        }
    }

    if in_quotes {
        return Err("unclosed quotes 😓".to_string());
    }

    if !new_input.is_empty() || found_quotes {
        new.push(new_input);
    }
    Ok(new)
}

/**********🌟 get_prompt 🌟**********/
pub fn get_prompt(params: &Params) -> String {
    format!(
        "{}{}{} ",
        "~".bold().yellow(),
        current(params).bold().truecolor(199, 21, 133),
        "$".bold().yellow()
    )
}

/**********🌟 read_input 🌟**********/
pub fn read_input(history: PathBuf, params: &Params) -> (String, Vec<String>) {
    let mut rl = match rustyline::DefaultEditor::new() {
        Ok(editor) => editor,
        Err(_) => {
           return (String::new(), Vec::new());
        }
    };
    rl.load_history(&history).unwrap_or_default();

    let mut cmd = String::new();

    loop {
        let prompt = if cmd.is_empty() {
            get_prompt(params)
        } else {
            "> ".to_string()
        };

        let input = rl.readline(&prompt);

        match input {
            Ok(line) => {
                let line = line.trim_end();

                if cmd.is_empty() {
                    cmd = line.to_string();
                } else {
                    cmd = format!("{}\n{}", cmd, line);
                }

                match parsing(&cmd) {
                    Ok(elements) => {
                        if elements.is_empty() {
                            return (String::new(), Vec::new());
                        }

                        let command = elements[0].clone();
                        let args = if elements.len() > 1 {
                            elements[1..].to_vec()
                        } else {
                            Vec::new()
                        };

                        let new_args = env_variable(args);

                        rl.add_history_entry(&cmd).expect("Failed to add history");
                        if let Ok(_save) = rl.save_history(&history) {
                            rl.save_history(&history).unwrap();
                        }

                        return (command, new_args);
                    }
                    Err(_) => {
                        print_quote_prompt();
                    }
                }
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                return (String::new(), Vec::new());
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                process::exit(1);
            }
            Err(err) => {
                println!("Error: {:?}", err);
                return (String::new(), Vec::new());
            }
        }
    }
}

/*********🌟 env_variable 🌟********/
fn env_variable(args: Vec<String>) -> Vec<String> {
    let mut new_args: Vec<String> = Vec::new();
    for word in &args {
        let new_word = word.trim();
        let mut temp = String::new();

        if new_word == "$0" {
            new_args.push("0-shell".to_string());
        }

        if new_word == "$0" {
            new_args.push("0-shell".to_string());
        }

        if new_word == "~" {
            let home = match env::home_dir() {
               Some(home_dir) => home_dir,
               None => PathBuf::from("/"),
            };
            new_args.push(home.display().to_string());
            break;
        }

        let mut chars = new_word.chars().peekable();
        
        while let Some(c) = chars.next() {
            if c == '$' {
               let mut dollar_count = 1;
                while chars.peek() == Some(&'$') {
                    chars.next();
                    dollar_count += 1;
                }
                
                if dollar_count == 1 { 
                    let mut var_env = String::new();
                    while let Some(&next_char) = chars.peek() {
                        if next_char.is_alphanumeric() || next_char == '_' {
                            var_env.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    
                    if var_env.is_empty() {
                        temp.push('$');
                    } else if let Ok(env_var) = std::env::var(&var_env) {
                        temp.push_str(&env_var);
                    }
                } else {
                    if dollar_count % 2 == 1 {
                        for _ in 0..(dollar_count / 2) {
                            temp.push_str(&std::process::id().to_string());
                        }
                        temp.push('$');
                    } else {
                        for _ in 0..(dollar_count / 2) {
                            temp.push_str(&std::process::id().to_string());
                        }
                    }
                }
            } else {
               temp.push(c);
            }
        }
        new_args.push(temp);
    }
    new_args
    
}
