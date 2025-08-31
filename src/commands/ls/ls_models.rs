use crate::commands::ls::ls_tools::{ file_data, find_symlink, col_width };
use colored::{ ColoredString, Colorize };
use is_executable::is_executable;
use std::path::Path;
use std::{ fs, os::unix::fs::FileTypeExt };
use xattr::list;

#[derive(Debug, Default)]
pub struct Flags {
    pub f_flag: bool,
    pub a_flag: bool,
    pub l_flag: bool,
}

impl Flags {
    pub fn hidden_file(&self, name: &str) -> bool {
        self.a_flag || !name.starts_with('.')
    }

    pub fn format_output(&self, file_name: &str, path_name: &str) -> Vec<String> {
        let mut line = Vec::new();
        let (file_perm, links, owner, group, major, minor, date) = file_data(file_name, path_name);

        // let s_link = &find_symlink(&Path::new(&format!("/{}/{}", path_name, file_name)));
        // let link_type: Files = Files::new_file(&Path::new(&s_link));
        if self.l_flag {
            line.extend(vec![file_perm, links.to_string(), owner, group, major, minor, date]);
            // if find_symlink(&Path::new(&format!("/{}/{}", path_name, file_name))).is_empty() {
            line.push(file_name.to_string());
            // } else {
            //     line.push(format!("{} -> {}", file_name.to_string(), &s_link.to_string()));
            // }
        } else {
            line.push(file_name.to_string());
        }
        line
    }
}

#[derive(Debug, PartialEq)]
pub enum Files {
    Symlink,
    Dir,
    Socket,
    Exec,
    Fifo,
    Reg,
    Dev,
}

impl Files {
    pub fn new_file(p_: &Path) -> Self {
        if let Ok(path) = fs::symlink_metadata(p_) {
            if path.file_type().is_socket() {
                return Files::Socket;
            }
            if path.file_type().is_fifo() {
                return Files::Fifo;
            }
            if path.file_type().is_block_device() || path.file_type().is_char_device() {
                return Files::Dev;
            }
            if p_.is_file() && is_executable(p_) {
                return Files::Exec;
            }
        }
        if p_.is_symlink() {
            return Files::Symlink;
        }
        if p_.is_dir() {
            return Files::Dir;
        }
        Files::Reg
    }

    pub fn file_color(&self, path_str: &str) -> ColoredString {
        match self {
            Files::Dir => path_str.bold().blue(),
            Files::Exec => path_str.bold().green(),
            Files::Socket => path_str.bold().magenta(),
            Files::Fifo | Files::Dev => path_str.bold().yellow().on_black(),
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

    pub fn file_format(file_name: &str, path: &str, flag: &Flags) -> String {
        let s_link = &find_symlink(&Path::new(&format!("/{}/{}", path, file_name)));
        let f_type = Files::new_file(Path::new(&format!("{}/{}", path, file_name)));
        let sym_type = Files::new_file(&Path::new(&s_link));

        if flag.l_flag {
            if !find_symlink(&Path::new(&format!("/{}/{}", path, file_name))).is_empty() {
                return format!("{} -> {}", f_type.file_color(file_name), if flag.f_flag {
                    sym_type.file_symbol(&sym_type.file_color(s_link))
                } else {
                    sym_type.file_color(s_link).to_string()
                });
            }
        }

        if flag.f_flag {
            return format!("{}", f_type.file_symbol(&f_type.file_color(file_name)));
        }
        format!("{}", f_type.file_color(file_name))
    }

    pub fn display_line(lines: Vec<Vec<String>>, path: &str, flag: &Flags) {
        // println!("{:?}", lines);
        let col_width = col_width(&lines);
        for line in &lines {
            for (i, elem) in line.iter().enumerate() {
                let file_name = i == line.len() - 1;
                let is_num: bool = elem.chars().all(|e| (e.is_ascii_digit() || e == ','));
                // println!("CHECKK-->{:?}", is_num);
                if file_name {
                    print!("{:<w$} ", Files::file_format(elem, path, flag), w = col_width[i]);
                } else if is_num {
                    print!("{:>w$} ", elem, w = col_width[i]);
                } else {
                    print!("{:<w$} ", elem, w = col_width[i]);
                }
            }
            println!();
        }
    }

    pub fn has_extra_attrs(path: &str) -> bool {
        match list(path) {
            Ok(mut attrs) => attrs.next().is_some(),
            Err(_) => false,
        }
    }
}
