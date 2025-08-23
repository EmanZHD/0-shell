mod shell; // 👈 This tells Rust to look for `shell.rs`
mod errors; // 👈 This tells Rust to look for `errors.rs`
use shell::spawn_user_input_handler; // 👈 Import the function
#[tokio::main]
async fn main() {
    let user_input_handler = spawn_user_input_handler().await;
    if let Ok(Err(e)) = user_input_handler{
        eprintln!("Error: {}" , e);
    }
}
