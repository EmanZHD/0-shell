pub fn history(his: &mut Vec<String>) {
   if his.is_empty() {
     println!("This is your history 🤗");
     return;
   } 
   for col in his {
      println!("{}", col);
   } 
}