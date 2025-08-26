use std::{ fs, io, os::unix::fs::FileTypeExt };
use colored::{ ColoredString, Colorize };
use std::path::Path;
use is_executable::is_executable;

#[derive(Debug)]
pub enum Files {
    Symlink,
    Dir,
    Socket,
    Exec,
    Fifo,
    Reg,
}

impl Files {
    pub fn new_file(p_: &Path) -> Self {
        // let p_ = Path::new(p);
        // println!("INSIE NEW --> {:?}", p_.is_symlink());
        if p_.is_dir() {
            return Files::Dir;
        }
        if p_.is_symlink() {
            return Files::Symlink;
        }
        if let Ok(path) = fs::metadata(p_) {
            if path.file_type().is_socket() {
                return Files::Socket;
            }
            if path.file_type().is_fifo() {
                return Files::Fifo;
            }
        }
        if p_.is_file() && is_executable(p_) {
            return Files::Exec;
        }
        Files::Reg
    }

    pub fn file_color(&self, path_str: &str) -> ColoredString {
        match self {
            Files::Dir => path_str.bold().blue(),
            Files::Exec => path_str.bold().green(),
            Files::Socket => path_str.bold().magenta(),
            Files::Fifo => path_str.yellow(),
            Files::Symlink => path_str.bold().cyan(),
            _ => path_str.white(),
        }
    }

    pub fn file_symbol(&self, path_str: &ColoredString) -> String {
        // println!("INSIDE SYmbole---> {:?}", self);
        let mut s = path_str.to_string();
        match self {
            Files::Dir => s.push('/'),
            Files::Exec => s.push('*'),
            Files::Socket => s.push('='),
            Files::Fifo => s.push('|'),
            Files::Symlink => s.push('@'),
            _ => {}
        }
        s
    }
}
#[derive(Debug, Default)]
struct Flags {
    f_flag: bool,
    a_flag: bool,
    l_flag: bool,
}

fn ls_printer(list: &mut Vec<(String, Files)>, flag: &Flags) {
    list.sort_by(|f1, f2| {
        let f1_key = f1.0.strip_prefix('.').unwrap_or(&f1.0);
        let f2_key = f2.0.strip_prefix('.').unwrap_or(&f2.0);
        f1_key.to_lowercase().cmp(&f2_key.to_lowercase())
    });
    for c in list {
        if flag.hidden_file(&c.0) {
            print!("{} ", flag.format_output(&c.0, &c.1));
        }
    }
    println!();
}

impl Flags {
    fn hidden_file(&self, name: &str) -> bool {
        self.a_flag || !name.starts_with('.')
    }
    fn format_output(&self, file_name: &str, file: &Files) -> String {
        let format = if self.f_flag {
            file.file_symbol(&file.file_color(file_name))
        } else {
            file.file_color(file_name).to_string()
        };
        format
    }
}
fn parse_args(args: Vec<String>) -> Result<(Flags, Vec<String>), ()> {
    let mut flags = Flags::default();
    let mut paths = Vec::new();

    for arg in args {
        if arg.len() > 1 && arg.starts_with('-') {
            if !arg[1..].chars().all(|c| ['a', 'l', 'F'].contains(&c)) {
                eprintln!("ls: invalid flag - '{}'", &arg[1..]);
                // std::process::exit(1);
                return Err(());
            }
            if arg[1..].contains('a') {
                flags.a_flag = true;
            }
            if arg[1..].contains('F') {
                flags.f_flag = true;
            }
            if arg[1..].contains('l') {
                flags.l_flag = true;
            }
        } else {
            paths.push(arg);
        }
    }

    if paths.is_empty() {
        paths.push("./".to_string());
    }

    Ok((flags, paths))
}

fn ls_helper(path_name: &str, flag: &Flags) -> Result<(), io::Error> {
    let mut content: Vec<(String, Files)> = vec![];
    // println!("🪄 track entries -> {:?}  ", content);
    for entry in fs::read_dir(path_name)? {
        match entry {
            Ok(dir_entry) => {
                let p = dir_entry.path();
                if let Some(file_name) = dir_entry.file_name().to_str() {
                    // if !file_name.is_empty() && !file_name.starts_with('.') {
                    // println!("-------> {:?} WITH TYPE => {:?}", p, Files::new_file(&p));
                    content.push((file_name.to_owned(), Files::new_file(&p)));
                    // }
                }
            }
            Err(_e) => eprintln!("error in readinf '{}'", path_name),
        }
    }
    // content.sort();
    // detect_file_type(&p);
    if !content.is_empty() && flag.a_flag {
        content.insert(0, (".".to_owned(), Files::Dir));
        content.insert(1, ("..".to_owned(), Files::Dir));
    }
    ls_printer(&mut content, flag);
    Ok(())
}

pub fn ls(args: Vec<String>) {
    // let check_flags = parse_args(args);
    let (flags, mut new_args) = match parse_args(args) {
        Ok((flags, new_args)) => (flags, new_args),
        Err(()) => {
            return;
        }
    };
    // let mut detect_flag = false;

    // println!("🏳️ CHECK FLAG {:?}", flags.);
    new_args.sort();
    // println!("LS args BEFORE=> {:?}", new_args);
    for (i, path_str) in new_args.iter().enumerate() {
        let path_name = Path::new(path_str);
        // println!("{} {:?}", "🪄 detect file type -->".yellow().bold(), path_name.metadata());
        match path_name.metadata() {
            Ok(path_data) => {
                if path_data.is_file() {
                    println!("{}", path_str);
                } else if path_data.is_dir() {
                    // print!("{}  ", path_data);
                    if new_args.len() > 1 {
                        println!("{}:", path_str);
                    }
                    let _ = ls_helper(path_str, &flags);
                }
            }
            Err(_) => eprintln!("ls: cannot access '{}': No such file or directory", path_str),
        }

        if i != new_args.len() - 1 {
            println!();
        }
    }
}
