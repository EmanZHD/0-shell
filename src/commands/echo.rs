use crate::Params;

/*********🌟 echo 🌟********/
pub fn echo(parameters: &mut Params) {
    let mut output: Vec<String> = Vec::new();
    // let mut compt_double = 0;
    // let mut compt_single = 0;
    for word in &parameters.args {
        let new_word = word.trim();
        if new_word.starts_with("$") {
            if let Err(_e) = std::env::var(new_word.trim_matches('$')) {
               continue;
            }else {
                println!(" ===> {}", std::env::var(new_word.trim_matches('$')).unwrap());
                output.push(std::env::var(new_word.trim_matches('$')).unwrap());
                println!("### output {:?}", output);
            }
            
        }else {
        //    new_word = new_word.trim_matches(&['\'', '\"']);
           output.push(new_word.to_string());
        }
    //     if new_word.starts_with("\"") {
    //         compt_double+=1;
    //     }
    //     if new_word.starts_with("\'") {
    //         compt_single+=1;
    //     }
    //     if new_word.ends_with("\"") {
    //         compt_double+=1;
    //     }
    //     if new_word.ends_with("\'") {
    //         compt_single+=1;
    //     }
        
        
    }
    //if compt_double%2 == 0 && compt_single%2 == 0 {
        for i in output {
           print!("{} ", i); 
        }
    // }else {
    //     print!("⛔ Error!"); 
    // }
    println!();
}