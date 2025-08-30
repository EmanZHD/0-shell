use crate::Params;

/*********🌟 echo 🌟********/
pub fn echo(parameters: &mut Params) {
    let mut output: Vec<String> = Vec::new();
    for word in &parameters.args {
        let new_word = word.trim();
        let mut temp = String::new();
        if new_word.starts_with("$") {
            if let Ok(var) = std::env::var(new_word.trim_matches('$')) {
                output.push(var);
            }
            continue;

        } 
        
        let mut chars = new_word.chars().peekable();
        
        while let Some(c) = chars.next() {
            if c == '\\' {
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
