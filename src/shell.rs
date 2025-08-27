use crate::errors::CrateResult;
use crate::commands::rm;  // This imports the re-exported function
use crate::commands::mkdir;  // This imports the re-exported function
use crate::commands::cp;  // This imports the re-exported function
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt},
    task::JoinHandle,
};
use anyhow::anyhow;


#[derive(Clone, Debug)]
pub enum Command {
    Mkdir(String),
    Rm(String),
    Cp(String),
}

// Standard traits (TryFrom) = Fixed method names (try_from)
impl TryFrom<&str> for Command {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split_value: Vec<&str> = value.split_whitespace().collect();
        
        match split_value[0] {
            "mkdir" => {
                if split_value.len() != 2 {
                    Err(anyhow!("mkdir command requires an argument"))
                } else {
                    Ok(Command::Mkdir(split_value[1..].join(" ")))
                }
            },
            "rm" => {
                if split_value.len() < 2 {
                    Err(anyhow!("rm command requires an argument"))
                } else  {
                    Ok(Command::Rm(split_value[1..].join(" ")))
                }
            },
            "cp" => {
                if split_value.len() < 2 {
                    Err(anyhow!("cp: missing file operand"))
                }else if split_value.len() < 3 {
                    Err(anyhow!(" cp: missing destination file operand after {}" , split_value[1]))
                } else {
                    Ok(Command::Cp(split_value[1..].join(" ")))
                }
            }
            _ => Err(anyhow!("Unknown command")),
        }
    }
}



async fn handle_new_line(line: &str) -> CrateResult<Command> {
    let command : Command = line.try_into()?;
    match command.clone() {
        Command::Rm(arg) => {  rm(&arg)?; },
        Command::Mkdir(arg) => {  mkdir(&arg); },
        Command::Cp(arg) => {  cp(&arg); },

    _ => {}
    }
    Ok(command)
}


pub fn spawn_user_input_handler() -> JoinHandle<CrateResult<()>> {
    tokio::spawn(async {
        let stdin = tokio::io::stdin();
        let mut reader = tokio::io::BufReader::new(stdin).lines();
        while let Ok(Some(line)) = reader.next_line().await {
            let command = handle_new_line(&line).await;
            if let Ok(command) = &command {
                match command {
                    _ => {}
                }
            } else {
                println!("Error parsing command: {}", command.err().unwrap()); 
            }
        }
        Ok(())
    })
}
