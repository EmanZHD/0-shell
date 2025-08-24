use std::fs; 
use crate::errors::CrateResult;   
pub fn rm(path: &str) -> CrateResult<()> {
      fs::remove_file(path)?;
      Ok(())
}