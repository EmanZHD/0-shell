pub fn history(his: &mut Vec<(i32, String)>) {
   if his.is_empty() {
     println!("This is your history 🤗");
     return;
   }   // toutours il n'est jamais empty car il y a une commande qui est history
   let lenght = his.len().to_string();
   for col in his {
      let espace = lenght.len() - col.0.to_string().len();
      let result= format!("{} {}", espace, col.1);  // n'est pas termine
      println!("{}", result);
   } 
}