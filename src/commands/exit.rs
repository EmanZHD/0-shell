use std::process;
use crate::Params;

pub fn exit(_parameters: &mut Params) {
    println!("Leaving 0-shell... Goodbye 😁");
    process::exit(1);
}
