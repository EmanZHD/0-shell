use crate::commands::ls::ls_tools::{
    find_symlink,
    col_width,
    find_major_minor,
    find_group_owner,
    format_time,
    padding,
};
use colored::{ ColoredString, Colorize };
use is_executable::is_executable;
use std::path::Path;
use std::{ fs, os::unix::fs::FileTypeExt };
use xattr::list;
use std::fs::{ Metadata };
use std::os::unix::fs::MetadataExt;
use terminal_size::{ terminal_size, Width };
use console::measure_text_width;

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

    pub fn line_data(&self, file_name: &str, path_name: &str) -> Vec<String> {
        let mut line = Vec::new();
        let file_data = FileData::extarct_data(file_name, path_name);

        if self.l_flag {
            line.extend(
                vec![
                    file_data.f_permission,
                    file_data.num_links.to_string(),
                    file_data.owner_id,
                    file_data.group_id,
                    file_data.f_major,
                    file_data.f_minor,
                    file_data.format_time
                ]
            );
            line.push(file_name.to_string());
        } else {
            line.push(file_name.to_string());
        }
        line
    }
}

#[derive(Debug)]
pub struct FileData {
    pub f_permission: String,
    pub num_links: u64,
    pub owner_id: String,
    pub group_id: String,
    pub f_major: String,
    pub f_minor: String,
    pub format_time: String,
}

impl FileData {
    pub fn extarct_data(file_name: &str, path_name: &str) -> Self {
        let path = format!("{}/{}", path_name, file_name);
        let mut f_major = String::new();
        let mut f_minor = String::new();
        let meta: Metadata = fs
            ::symlink_metadata(Path::new(&path))
            .or_else(|_| fs::symlink_metadata(Path::new(file_name)))
            .expect("can't read data");
        let num_links = meta.nlink();
        if meta.file_type().is_char_device() || meta.file_type().is_block_device() {
            if let Some((major, minor)) = find_major_minor(&path) {
                f_major.push_str(&format!("{},", major));
                f_minor.push_str(&minor.to_string());
            }
        } else {
            f_minor.push_str(&meta.len().to_string());
        }

        let owner_id = find_group_owner((meta.uid(), true));
        let group_id = find_group_owner((meta.gid(), false));
        let format_time = format_time(&meta);
        let mut f_permission = format!("{:?}", meta.permissions())
            .split_whitespace()
            .collect::<Vec<&str>>()[4]
            .trim_matches(|e| (e == '(' || e == ')'))
            .to_string();

        if Files::has_extra_attrs(&path) {
            f_permission.push('+');
        }
        Self {
            f_permission,
            num_links,
            owner_id,
            group_id,
            f_major,
            f_minor,
            format_time,
        }
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

    // pub fn file_format(file_name: &str, path: &str, flag: &Flags) -> String {
    //     let mut dir = "./".to_string();
    //     if path == "./".to_string() || Files::new_file(&Path::new(path)) == Files::Symlink {
    //         if let Ok(curr_dir) = env::current_dir() {
    //             if let Some(s) = curr_dir.to_str() {
    //                 dir = s.to_string();
    //             }
    //         }
    //     } else {
    //         dir = path.to_string();
    //     }
    //     // println!("DIR -> {} file {} path {}", dir, file_name, path);
    //     let s_link = &find_symlink(&Path::new(&format!("{}/{}", dir, file_name)));
    //     if Files::new_file(&Path::new(path)) == Files::Symlink && (flag.l_flag || flag.f_flag) {
    //         // println!(
    //         //     "SYMLINK -> {} dir {} file {}",
    //         //     dir,
    //         //     file_name,
    //         //     &find_symlink(&Path::new(&format!("/{}/{}", ".", file_name)))
    //         // );
    //         return format!("-> {}", Files::new_file(&Path::new(&s_link)).file_color(&s_link));
    //     }
    //     let f_type = Files::new_file(Path::new(&format!("{}/{}", dir, file_name)));
    //     // println!(
    //     //     "FILE --> {} TYPE--> {:?} COLOR {}",
    //     //     &format!("{}/{}", path, file_name),
    //     //     f_type,
    //     //     Files::new_file(Path::new(&format!("{}/{}", path, file_name))).file_color(file_name)
    //     // );
    //     let sym_type = Files::new_file(&Path::new(&s_link));

    //     if flag.l_flag {
    //         if !find_symlink(&Path::new(&format!("{}/{}", dir, file_name))).is_empty() {
    //             return format!("{} -> {}", f_type.file_color(file_name), if flag.f_flag {
    //                 sym_type.file_symbol(&sym_type.file_color(s_link))
    //             } else {
    //                 sym_type.file_color(s_link).to_string()
    //             });
    //         }
    //     }

    //     if flag.f_flag {
    //         return format!("{}", f_type.file_symbol(&f_type.file_color(file_name)));
    //     }
    //     format!(
    //         "{}",
    //         Files::new_file(Path::new(&format!("{}/{}", path, file_name))).file_color(file_name)
    //     )
    // }
    pub fn file_format(file_name: &str, path: &str, flag: &Flags) -> String {
        let new_path = Path::new(path).join(file_name);

        if let Ok(meta) = fs::symlink_metadata(&new_path) {
            if meta.file_type().is_symlink() {
                let sym_file = find_symlink(&new_path);
                if !sym_file.is_empty() {
                    let sym_type = Files::new_file(Path::new(&sym_file));
                    if flag.l_flag {
                        return format!("{} -> {}", Files::Symlink.file_color(file_name), if
                            flag.f_flag
                        {
                            println!(
                                "SYM -> {}",
                                sym_type.file_symbol(&sym_type.file_color(&sym_file))
                            );
                            sym_type.file_symbol(&sym_type.file_color(&sym_file))
                        } else {
                            sym_type.file_color(&sym_file).to_string()
                        });
                    }

                    if flag.f_flag {
                        return Files::Symlink.file_symbol(&Files::Symlink.file_color(file_name));
                    }

                    return Files::Symlink.file_color(file_name).to_string();
                }
            }
        }

        let f_type = Files::new_file(&new_path);
        if flag.f_flag {
            return f_type.file_symbol(&f_type.file_color(file_name));
        }
        f_type.file_color(file_name).to_string()
    }

    pub fn display_line(lines: Vec<Vec<String>>, path: &str, flag: &Flags) {
        let col_width = col_width(&lines);
        for line in &lines {
            for (i, elem) in line.iter().enumerate() {
                let file_name = i == line.len() - 1;
                let is_num: bool = elem.chars().all(|e| (e.is_ascii_digit() || e == ','));
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

    pub fn display_file(lines: Vec<Vec<String>>, path: &str, flag: &Flags) {
        // println!("TEST -> {:?}", path);
        let files_name: Vec<String> = lines
            .iter()
            .flatten()
            .map(|s| {
                // println!("COLORE IT -> {}", Files::file_format(s, path, flag).to_string());
                return Files::file_format(s, path, flag).to_string();
            })
            .collect();

        let max_len = files_name
            .iter()
            .map(|s| measure_text_width(s))
            .max()
            .unwrap_or(0);
        let col_width = max_len + 2;
        let term_width = terminal_size()
            .map(|(Width(w), _)| w as usize)
            .unwrap_or(80);
        let cols = (term_width / col_width).max(1);

        if files_name.len() <= cols {
            println!("{}", files_name.join("  "));
            return;
        }

        let rows = (files_name.len() + cols - 1) / cols;
        for row in 0..rows {
            for col in 0..cols {
                let i = col * rows + row;
                if i < files_name.len() {
                    let is_last = col == cols - 1 || i + rows >= files_name.len();
                    print!("{}", padding(files_name[i].clone(), col_width, is_last));
                }
            }
            println!();
        }
    }
}
