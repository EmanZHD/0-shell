use colored::*;
use std::env;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process;

/*********🌟 Current Dir 🌟********/
pub fn current() -> String {
    let result: String = match env::current_dir() {
        Ok(path) => match path.file_name() {
            Some(file_name) => file_name.to_string_lossy().into_owned(),
            _none => String::from("/"),
        },
        Err(_e) => "/".to_string(),
    };
    result
}

/*********🌟 print_prompt 🌟********/
pub fn print_prompt() {
    let begin = format!(
        "{}{}{} ",
        "~".bold().yellow(),
        current().bold().truecolor(199, 21, 133),
        "$".bold().yellow()
    );
    print!("{}", begin);
    match io::stdout().flush() {
        Ok(()) => {
            return;
        }
        _ => eprintln!("broken pipe"),
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
    let mut quote = ' '; // pour memoriser le quote
    let mut test = input.chars().peekable();
    while let Some(c) = test.next() {
        match c {
            '\\' if test.peek() == Some(&'\"') || test.peek() == Some(&'\'') => {
                new_input.push(test.next().unwrap());
                continue;
            }

            '\'' | '"' if !in_quotes => {
                in_quotes = true;
                quote = c; // pour memoriser le type de quote
            }

            '\'' | '\"' if in_quotes && c == quote => {
                in_quotes = false; // fermeture de la quote du m type
            }
            ' ' | '\t' if !in_quotes => {
                if !new_input.is_empty() {
                    new.push(new_input);
                    new_input = String::new();
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

    if !new_input.is_empty() {
        new.push(new_input);
    }
    Ok(new)
}

/**********🌟 get_prompt 🌟**********/
pub fn get_prompt() -> String {
    format!(
        "{}{}{} ",
        "~".bold().yellow(),
        current().bold().truecolor(199, 21, 133),
        "$".bold().yellow()
    )
}

/**********🌟 read_input 🌟**********/
pub fn read_input(history: PathBuf) -> (String, Vec<String>) {
    let mut rl = rustyline::DefaultEditor::new().expect("Failed to create editor");
    rl.load_history(&history).unwrap_or_default();

    let mut cmd = String::new();

    loop {
        let prompt = if cmd.is_empty() {
            get_prompt()
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

                        rl.add_history_entry(&cmd).expect("Failed to add history");
                        if let Ok(_save) = rl.save_history(&history) {
                            rl.save_history(&history).unwrap();
                        }

                        return (command, args);
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
                println!("test");
                process::exit(1);
            }
            Err(err) => {
                println!("Error: {:?}", err);
                return (String::new(), Vec::new());
            }
        }
    }
}