use crate::Params;

/*********🌟 echo 🌟********/
pub fn echo(parameters: &mut Params) {
    let mut output: Vec<String> = Vec::new();
    for word in &parameters.args {
        let new_word = word.trim();
        let mut temp = String::new();
        let mut count = 0;
        if new_word.starts_with("$") {
            if let Err(_e) = std::env::var(new_word.trim_matches('$')) {
                continue;
            } else {
                output.push(std::env::var(new_word.trim_matches('$')).unwrap());
                // print!("{}", std::env::var(new_word.trim_matches('$')).unwrap());
            }
        } else {
            // new_word = new_word.trim_matches(&['\'', '\"']);
            // output.push(new_word.to_string());
            for c in word.chars() {
                let mut charta= count % 4; // hadchy ghaaalt
                println!("🌸 {}", charta);
                if c == '\\' {
                   count+=1;
                }
                if c != '\\' {
                   temp.push(c);
                   count = 0;
                   charta = 0;
                }
                if charta > 0 {
                   temp.push('\\');
                }
            }
            output.push(temp);
            temp = "".to_string();
        }
    }
    for i in output {
        print!("{} ", i);
    }
    println!();
}
