use crate::errors::CrateResult;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt},
    task::JoinHandle,
};
use anyhow::anyhow;
#[derive(Clone, Debug)]
pub enum Command {
    Mkdir(String),
}


impl TryFrom<&str> for Command {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split_value: Vec<&str> = value.split_whitespace().collect();
        match split_value[0] {
            "mkdir" => {
                if split_value.len() < 2 {
                    Err(anyhow!("mkdir command requires an argument"))
                } else {
                    println!("ssss" );
                    Ok(Command::Mkdir(split_value[1..].join(" ")))
                }
            }
            _ => Err(anyhow!("Unknown command")),
        }
    }
}




pub fn spawn_user_input_handler() -> JoinHandle<CrateResult<()>> {
    tokio::spawn(async {
        let stdin = tokio::io::stdin();
        let mut reader = tokio::io::BufReader::new(stdin).lines();
        while let Ok(Some(line)) = reader.next_line().await {
             Command::try_from(line);
            println!("User entered: {}", line);
        }
        Ok(())
    })
}
