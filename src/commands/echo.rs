use crate::Params;

/*********🌟 echo 🌟********/
pub fn echo(parameters: &mut Params) {
    let mut output: Vec<String> = Vec::new();
    for word in &parameters.args {
        let new_word = word.trim();
        let mut temp = String::new();

        if new_word == "$0" {
            output.push("0-shell".to_string());
        }

        // if new_word.starts_with("$") {
        //     if let Ok(var) = std::env::var(new_word.trim_matches('$')) {
        //         output.push(var);
        //     }
        //     continue;
        // } 
        
        let mut chars = new_word.chars().peekable();
        
        while let Some(c) = chars.next() {
            println!("==> {c}");
            if c == '$' {
                println!("### ==> {c}");
               let mut dollar_count = 1;
                while chars.peek() == Some(&'$') {
                    chars.next();
                    dollar_count += 1;
                }
                
                if dollar_count == 1 {  // bach njam3 l'env variable
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
                println!("dollar {}", dollar_count);
            } else if c == '\\' {
                if let Some(next_char) = chars.next() {
            
                    match next_char {
                        'n' => temp.push('\n'),
                        't' => temp.push('\t'),
                        'r' => temp.push('\r'),
                        '\\' => temp.push('\\'),
                        '"' => temp.push('"'),
                        '\'' => temp.push('\''),
                        '0' => temp.push('\0'),
                        _ => {
                            temp.push('\\');
                            temp.push(next_char);
                        }
                    }
                } else {
                    temp.push('\\');
                }
            } else {
                temp.push(c);
            }
        }
        
        output.push(temp);
    }
    for i in output {
        print!("{} ", i);
    }
    println!();
}
