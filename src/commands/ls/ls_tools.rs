use std::path::{ Path };
use std::{ fs, os::unix::fs::FileTypeExt };
use users::{ get_group_by_gid, get_user_by_uid };
use std::os::unix::fs::MetadataExt;
use crate::commands::ls::ls_models::{ Flags };
use libc::{ self, uid_t };
use console::measure_text_width;
use std::fs::Metadata;
use chrono::{ DateTime, Duration, Local, Datelike };

pub fn col_width(lines: &Vec<Vec<String>>) -> Vec<usize> {
    let mut col_width = Vec::new();
    for line in lines {
        for (i, elem) in line.iter().enumerate() {
            if col_width.len() <= i {
                col_width.push(elem.len());
            } else if elem.len() > col_width[i] {
                col_width[i] = elem.len();
            }
        }
    }
    col_width
}

pub fn total_blocks(dir_path: &Path, flag_a: bool) -> u64 {
    // println!("PQTH {:?} {:?}", dir_path, flag_a);
    let mut total: u64 = 0;
    if flag_a {
        if let Ok(meta) = fs::symlink_metadata(Path::new(dir_path)) {
            total += meta.blocks();
        }
    }
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if !flag_a && entry.file_name().to_string_lossy().starts_with('.') {
                    continue;
                }
                if let Ok(metadata) = fs::symlink_metadata(entry.path()) {
                    total += metadata.blocks();
                }
            }
        }
    }
    if flag_a {
        if let Ok(meta) = fs::symlink_metadata(dir_path.join("..")) {
            total += meta.blocks();
        }
    }
    total / 2
}

// parse_args fn
pub fn parse_args(args: Vec<String>) -> Result<(Flags, Vec<String>), ()> {
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

pub fn find_major_minor(path: &str) -> Option<(u64, u64)> {
    let meta = fs::metadata(Path::new(path)).ok()?;
    if !(meta.file_type().is_block_device() || meta.file_type().is_char_device()) {
        return None;
    }
    let rdev = meta.rdev();
    let major = ({ libc::major(rdev) }) as u64;
    let minor = ({ libc::minor(rdev) }) as u64;
    Some((major, minor))
}

pub fn find_group_owner(info: (uid_t, bool)) -> String {
    let name = if info.1 {
        get_user_by_uid(info.0).map(|u| u.name().to_owned())
    } else {
        get_group_by_gid(info.0).map(|g| g.name().to_owned())
    };

    match name {
        Some(v) =>
            v
                .to_str()
                .unwrap_or_else(|| "undefined")
                .to_string(),
        _ => "undefined".to_string(),
    }
}

pub fn find_symlink(path: &Path) -> String {
    if let Ok(link) = fs::read_link(path) {
        // println!("{:?} ---> {:?}", path, link);
        return link.display().to_string();
    } else {
        return "".to_owned();
    }
}

pub fn format_time(meta: &Metadata) -> String {
    if let Ok(modified) = meta.modified() {
        let mut date: DateTime<Local> = modified.into();
        let real_date = date.clone();
        date = date + Duration::hours(1);
        let now = Local::now();
        if date.year() != now.year() || real_date > now {
            date.format("%b %e  %Y").to_string()
        } else {
            date.format("%b %e %H:%M").to_string()
        }
    } else {
        "--- --- --:--".to_string()
    }
}

pub fn padding(str: String, width: usize, is_last: bool) -> String {
    if is_last {
        str
    } else {
        let space = measure_text_width(&str);
        let pad = width.saturating_sub(space);
        format!("{}{}", str, " ".repeat(pad))
    }
}

// format_lines fn
pub fn format_lines(list: &mut Vec<String>, flag: &Flags, path_name: &str) -> Vec<Vec<String>> {
    list.sort_by(|f1, f2| {
        let f1_key = f1.strip_prefix('.').unwrap_or(f1).trim();
        let f2_key = f2.strip_prefix('.').unwrap_or(f2).trim();
        let f1_key = f1_key.replace('-', "");
        let f2_key = f2_key.replace('-', "");

        f1_key.to_lowercase().cmp(&&f2_key.to_lowercase())
    });
    let mut line = Vec::new();
    for c in list {
        if flag.hidden_file(&c) {
            line.push(flag.line_data(&c, path_name));
        }
    }
    line
}
