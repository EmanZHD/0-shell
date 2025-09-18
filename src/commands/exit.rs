use std::process;
use crate::Params;
use is_number::is_number;

pub fn exit(parameters: &mut Params) {
    let goodbye = "Leaving 0-shell... Goodbye 😁";
    if parameters.args.is_empty() {
        println!("{}", goodbye);
        process::exit(0);
    }
    if parameters.args.len() > 1 {
        let mut number = false;
        for i in parameters.args.clone() {
           if is_number(&i) {
               number = true;
           }
        }
        if number {
            eprintln!("⛔ 0-shell: exit: too many arguments"); 
        }else {
            println!("{}", goodbye);
            eprintln!("⛔ 0-shell: exit: {}: numeric argument required", parameters.args[0]);
            process::exit(2);
        }
        
    }
    if parameters.args.len() == 1 {
        if is_number(&parameters.args[0]) {
            match parameters.args[0].parse::<i32>() {
                Ok(nbr) => {
                    println!("{}", goodbye);
                    process::exit(nbr)},
                Err(_) => {
                    println!("{}", goodbye);
                    process::exit(2)},
            }
        }else {
            println!("{}", goodbye);
            eprintln!("⛔ 0-shell: exit: {}: numeric argument required", parameters.args[0]);
            process::exit(2);
        }
    }
}
