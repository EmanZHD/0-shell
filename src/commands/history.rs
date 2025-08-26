pub fn history(his: &mut Vec<(i32, String)>) {
   println!("This is your history 🤗");
   let lenght = his.len().to_string();
   for col in his {
      let espace = lenght.len() - col.0.to_string().len();
      let result= format!("{}{} {}", " ".repeat(espace), col.0, col.1);
      println!("{}", result);
   } 
}