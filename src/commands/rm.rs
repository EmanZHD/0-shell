use std::fs; 
// use crate::errors::CrateResult;  
use anyhow::anyhow;
use crate::Params;
pub fn rm(params: &mut Params)  {
println!("========{:?}", params.args);
// rm one ee rr
// ✅ Input: "rm one ee rr"
// ✅ Command line: "rm one ee rr"
// ✅ Command: "rm"
// ✅ Arguments: ["one", "ee", "rr"]
// rm, ["one", "ee", "rr"]
// ========["one", "ee", "rr"]

      // let split_value: Vec<&str> = path.split_whitespace().collect();
      match  params.args.len() {
            0=> println!("rm: missing operand"),
            _ => {
                  println!("{}", params.args[0]);
                  match params.args[0].as_str() {
                        "-r" => {
                        for i in &params.args[1..] {
                        println!("====DDDD{}", i);
                              
                        },
                        _=> {}
                  }
                  // for i in params.args.clone() {
                  //       println!("{}", i);
                  //       fs::remove 
                  // }
            }
      }
      // if params.args.len() == 1 {
      //        match  fs::remove_file(path) {
      //                   Ok(_) => println!("File '{}' remove successfully.", split_value[0]),
      //                   Err(_) => eprintln!("rm: cannot remove {}: No such file", split_value[0]),    
      //                  }  
                          
      // } else if  params.args.len() == 2 {
      //       match split_value[0] {
      //             "-r" => {
      //                  match  fs::remove_dir(split_value[1]) {
      //                   Ok(_) => println!("Directory '{}' remove successfully.", split_value[1]),
      //                   Err(_) => eprintln!("rm: cannot remove {}: No such directory", split_value[1]),    
      //                  }  
      //             }
      //             _ => println!("Unknown command"),
      //       };
      // }
}
 