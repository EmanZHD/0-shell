use crate::Params;

pub fn history(parameters: &mut Params) {
   println!("This is your history 🤗");
   let lenght = parameters.archieve.len().to_string();
   for col in &parameters.archieve {
      let espace = lenght.len() - col.0.to_string().len();
      let result= format!("{}{} {}", " ".repeat(espace), col.0, col.1);
      println!("{}", result);
   } 
}