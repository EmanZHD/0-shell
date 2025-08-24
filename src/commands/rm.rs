use std::fs; 
use crate::errors::CrateResult;  
use anyhow::anyhow;

pub fn rm(path: &str) -> CrateResult<()> {

      let split_value: Vec<&str> = path.split_whitespace().collect();
      
      if split_value.len() == 1 {
             match  fs::remove_file(path) {
                        Ok(_) => println!("File '{}' remove successfully.", split_value[0]),
                        Err(_) => eprintln!("rm: cannot remove {}: No such file", split_value[0]),    
                       }  
                          Ok(())

      }else if  split_value.len() == 2 {
            match split_value[0] {
                  "-r" => {
                       match  fs::remove_dir(split_value[1]) {
                        Ok(_) => println!("Directory '{}' remove successfully.", split_value[1]),
                        Err(_) => eprintln!("rm: cannot remove {}: No such directory", split_value[1]),    
                       }  
                  }
                  _ => println!("Unknown command"),
            };
            Ok(())
      }else {
            Err(anyhow!("Unknown command"))
       
      }
}
 