use std::fs;
use std::path::Path;
pub fn cp(arg : &str) {
    let files : Vec<&str> = arg.split_whitespace().collect();
    
    let exists_source = Path::new(files[0]).exists();
    let exists_dist = Path::new(files[1]).exists();
    
    let source_is_file = Path::new(files[0]).is_file();
    let dis_is_file = Path::new(files[1]).is_file();

    match (exists_source , exists_dist , source_is_file , dis_is_file){
        ( false , _ , _, _) => println!("cp: cannot stat '{}': No such file or directory" , files[0]),
        ( _ , _ , false , _) => println!("cp: omitting directory '{}' " , files[0]),


 












        ( _ , false , _, _) => {
            //hna dis hya file mkynch donc 5as n creah o ncopy fih source
            let parent =  Path::new(files[1]).parent();
            match (parent.expect("REASON").exists() , parent.expect("REASON").is_dir() ) {
                (false , true) => println!("cp: cannot access '{}': No such file or directory" , files[1]),
            //    (_ , false) => println!("cp: cannot access '{}': No such file or directory" , files[1]) ,
               (false,false) => {
                 fs::copy(files[0], files[1]);
               },
               (true , true) =>  {
                 fs::copy(files[0], files[1]);
               },
               (true, false)=> println!("444444")
            }
        },

        









        ( _ , true , _, false) => {
            let finle_dis = Path::new(files[1]).join(files[0]);
            fs::copy(files[0] , finle_dis);
                println!("File copied successfully from {} to {}", files[0], files[1]);
        },
















        
        ( true , true , true, true) => {
            fs::copy(files[0], files[1]);
            println!("File copied successfully from {} to {}", files[0], files[1]);
        },
        ( true , true , true, false) => {
            // mn file l dir
            let destination_file = Path::new(files[1]).join(Path::new(files[0]).file_name().unwrap());

            fs::copy(files[0], destination_file ) .expect("Failed to copy file");
            println!("File copied successfully from {} to {}", files[0], files[1]);
        },
    }
}