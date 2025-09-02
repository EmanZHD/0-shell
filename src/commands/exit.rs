use std::process;
use crate::Params;
use is_number::is_number;

pub fn exit(parameters: &mut Params) {
    println!("Leaving 0-shell... Goodbye 😁");
    if parameters.args.is_empty() {
        process::exit(0);
    }
    if parameters.args.len() > 1 {
        println!("⛔ 0-shell: exit: {}: numeric argument required", parameters.args[0]);
        process::exit(2);
    }
    if parameters.args.len() == 1 {
        if is_number(&parameters.args[0]) {
            match parameters.args[0].parse::<i32>() {
                Ok(nbr) => process::exit(nbr),
                Err(_) => process::exit(2),
            }
        }else {
            println!("⛔ 0-shell: exit: {}: numeric argument required", parameters.args[0]);
            process::exit(2);
        }
    }
}
