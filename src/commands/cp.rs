use std::fs;
use std::path::Path;
pub fn cp(arg : &str) {
    let files : Vec<&str> = arg.split_whitespace().collect();
    
    let exists_source = Path::new(files[0]).exists();
    let exists_dist = Path::new(files[1]).exists();
    
    let source_is_file = Path::new(files[0]).is_file();
    let dis_is_file = Path::new(files[1]).is_file();
    
    println!("====hg{}" , dis_is_file);

    match (exists_source , exists_dist , source_is_file , dis_is_file){
        ( _ , _ , false , _) => println!("source not file"),
        ( _ , _ , _, false) => println!("dis not file"),
        ( true , true , true, true) => {
            fs::copy(files[0], files[1]);
        },
        ( true , false , true, true) =>{
            let newfile = fs::File::create(files[1]);
              fs::copy(files[0], newfile);
        },
        ( false , _ , true, true) => println!("ERROR - cannot copy nothing"),
    }
   
      println!("File copied successfully from {} to {}", files[0], files[1]);
  
}