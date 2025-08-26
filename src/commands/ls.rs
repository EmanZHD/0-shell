use chrono::{ DateTime, Duration, Utc };
use colored::{ ColoredString, Colorize };
use is_executable::is_executable;
use std::os::linux::fs::MetadataExt;
use std::path::Path;
use std::{ fs, io, os::unix::fs::FileTypeExt };
use users::{ get_group_by_gid, get_user_by_uid };
#[derive(Debug, Default)]
struct Flags {
    f_flag: bool,
    a_flag: bool,
    l_flag: bool,
}

fn file_permission(file_name: &str) -> (String, u64, String, String, u64, String) {
    let mut file_permission = String::new();
    let mut num_links: u64 = 0;
    let mut owner_id = String::new();
    let mut group_id = String::new();
    let mut file_size = 0;
    let mut format_time = String::new();
    if let Ok(meta) = fs::metadata(Path::new(file_name)) {
        let permissions = meta.permissions();
        num_links = meta.st_nlink();
        file_size = meta.len();
        if
            let Some(name) = get_user_by_uid(meta.st_uid()).and_then(|user|
                user
                    .name()
                    .to_str()
                    .map(|s| s.to_owned())
            )
        {
            owner_id.push_str(&name);
        } else {
            println!("file not found");
        }
        if
            let Some(name) = get_group_by_gid(meta.st_gid()).and_then(|user|
                user
                    .name()
                    .to_str()
                    .map(|s| s.to_owned())
            )
        {
            group_id.push_str(&name);
        } else {
            println!("file not found");
        }
        // let mode = permissions.mode();
        if let Ok(time) = meta.modified() {
            let datetime: DateTime<Utc> = DateTime::from(time) + Duration::hours(1);
            format_time = datetime.format("%b %e %H:%M").to_string();
        }

        let str_prm: String = format!("{:?}", permissions.to_owned());
        file_permission.push_str(
            &str_prm
                .split_whitespace()
                .collect::<Vec<&str>>()[4]
                .trim_matches(|e| (e == '(' || e == ')'))
        );
        // println!(
        //     "file-> {} |||| PERMISSIONS-> {:?}",
        //     file_name,
        //     &str_prm.split_whitespace().collect::<Vec<&str>>()
        // );
    }

    (file_permission, num_links, owner_id, group_id, file_size, format_time)
}

impl Flags {
    fn hidden_file(&self, name: &str) -> bool {
        self.a_flag || !name.starts_with('.')
    }
    fn format_output(&self, file_name: &str, file: &Files) -> String {
        let format = if self.f_flag {
            file.file_symbol(&file.file_color(file_name))
        } else if self.l_flag {
            format!(
                "{} {} {} {} {} {} {}",
                file_permission(file_name).0,
                file_permission(file_name).1,
                file_permission(file_name).2,
                file_permission(file_name).3,
                file_permission(file_name).4,
                file_permission(file_name).5,
                file.file_color(file_name).to_string()
            )
        } else {
            file.file_color(file_name).to_string()
        };
        format
    }
}

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

// ls_printer fn
fn ls_printer(list: &mut Vec<(String, Files)>, flag: &Flags) {
    list.sort_by(|f1, f2| {
        let f1_key = f1.0.strip_prefix('.').unwrap_or(&f1.0);
        let f2_key = f2.0.strip_prefix('.').unwrap_or(&f2.0);
        f1_key.to_lowercase().cmp(&f2_key.to_lowercase())
    });
    for c in list {
        if flag.hidden_file(&c.0) {
            if !flag.l_flag {
                print!("{} ", flag.format_output(&c.0, &c.1));
            } else {
                println!("{}", flag.format_output(&c.0, &c.1));
            }
        }
    }
    println!();
}

// parse_args fn
fn parse_args(args: Vec<String>) -> Result<(Flags, Vec<String>), ()> {
    let mut flags = Flags::default();
    let mut paths = Vec::new();

    for arg in args {
        if arg.len() > 1 && arg.starts_with('-') {
            if !arg[1..].chars().all(|c| ['a', 'l', 'F'].contains(&c)) {
                eprintln!("ls: invalid flag - '{}'", &arg[1..]);
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

// ls_helper fn
fn ls_helper(path_name: &str, flag: &Flags) -> Result<(), io::Error> {
    let mut content: Vec<(String, Files)> = vec![];
    for entry in fs::read_dir(path_name)? {
        match entry {
            Ok(dir_entry) => {
                let p = dir_entry.path();
                if let Some(file_name) = dir_entry.file_name().to_str() {
                    content.push((file_name.to_owned(), Files::new_file(&p)));
                }
            }
            Err(_e) => eprintln!("error in readinf '{}'", path_name),
        }
    }
    if !content.is_empty() && flag.a_flag {
        content.insert(0, (".".to_owned(), Files::Dir));
        content.insert(1, ("..".to_owned(), Files::Dir));
    }
    ls_printer(&mut content, flag);
    Ok(())
}

//ls fn
pub fn ls(args: Vec<String>) {
    let (flags, mut new_args) = match parse_args(args) {
        Ok((flags, new_args)) => (flags, new_args),
        Err(()) => {
            return;
        }
    };
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
