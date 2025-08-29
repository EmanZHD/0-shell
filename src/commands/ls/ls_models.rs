use crate::commands::ls::ls_tools::{ file_permission, find_symlink };
use colored::{ ColoredString, Colorize };
use is_executable::is_executable;
use std::path::Path;
use std::{ fs, os::unix::fs::FileTypeExt };

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
    pub fn format_output(&self, file_name: &str, file: &Files, path_name: &str) -> Vec<String> {
        // find_symlink(&Path::new(&format!("/{}/{}", path_name, file_name)));
        // println!(
        //     "here--> {} ||| {:?}",
        //     Files::new_file(
        //         &Path::new(&find_symlink(&Path::new(&format!("/{}/{}", path_name, file_name))))
        //     ).file_color(&find_symlink(&Path::new(&format!("/{}/{}", path_name, file_name)))),
        //     find_symlink(&Path::new(&format!("/{}/{}", path_name, file_name)))
        // );
        // println!("OUTPUT--> {} {}", path_name.cyan(), file_name.cyan());
        let mut Line = Vec::new();
        let (file_perm, links, owner, group, major, minor, date) = file_permission(
            file_name,
            path_name
        );

        let s_link = &find_symlink(&Path::new(&format!("/{}/{}", path_name, file_name)));
        let link_type = Files::new_file(&Path::new(&s_link));
        if self.f_flag {
            Line.push(file.file_symbol(&file.file_color(file_name)));
        } else if self.l_flag {
            Line.extend(vec![file_perm, links.to_string(), owner, group, major, minor, date]);
            // Line[0] = &file_perm;
            // Line[1] = &links.to_string();
            // Line[2] = &owner;
            // Line[3] = &group;
            // Line[4] = &major;
            // Line[5] = &minor;
            // Line[6] = &date;
            if find_symlink(&Path::new(&format!("/{}/{}", path_name, file_name))).is_empty() {
                // Line[7] = &file.file_color(file_name).to_string();
                // Line.push(file.file_color(file_name).to_string());
                Line.push(file_name.to_string());
            } else {
                Line.push(
                    // format!(
                    //     "{} -> {}",
                    //     file.file_color(file_name).to_string(),
                    //     &link_type.file_color(&s_link).to_string()
                    // )
                    format!("{} -> {}", file_name.to_string(), &s_link.to_string())
                );
                // Line[7] = &format!(
                //     "{} -> {}",
                //     file.file_color(file_name).to_string(),
                //     &link_type.file_color(&s_link).to_string()
                // );
            }
            // format!(
            //     "{} {} {} {} {} {} {} {}",
            //     file_perm,
            //     links,
            //     owner,
            //     group,
            //     major,
            //     minor,
            //     date,
            //     if find_symlink(&Path::new(&format!("/{}/{}", path_name, file_name))).is_empty() {
            //         file.file_color(file_name).to_string()
            //     } else {
            //         format!(
            //             "{} -> {}",
            //             file.file_color(file_name).to_string(),
            //             &link_type.file_color(&s_link).to_string()
            //         )
            //     }
            // );
        } else {
            // Line.push(file.file_color(file_name).to_string());
            Line.push(file_name.to_string());
        }
        // else {
        //     Line[0] = file.file_color(file_name).to_string();
        // };
        Line
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
}
