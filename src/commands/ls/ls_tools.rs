use std::{ fs, os::unix::fs::FileTypeExt };
use std::os::unix::fs::PermissionsExt;
use std::path::{ Path };
use std::os::unix::fs::{ MetadataExt };
use terminal_size::{ terminal_size, Width };
use users::{ get_group_by_gid, get_user_by_uid };
use crate::commands::ls::ls_models::{ Flags, FileData, FileInfo, Files };
use libc::{ self, uid_t };
use console::measure_text_width;
use std::fs::Metadata;
use chrono::{ DateTime, Duration, Local, Datelike };

//-------------------------col_width FUNC
// pub fn col_width(lines: &Vec<String>) -> Vec<usize> {
//     let mut col_width = Vec::new();
//     // for line in lines {
//     for (i, elem) in lines.iter().enumerate() {
//         if col_width.len() <= i {
//             col_width.push(elem.len());
//         } else if elem.len() > col_width[i] {
//             col_width[i] = elem.len();
//         }
//     }
//     // }
//     col_width
// }

//-------------------------col_width FUNC
pub fn total_blocks(dir_path: &Path, flag_a: bool) -> u64 {
    let mut total: u64 = 0;
    if flag_a {
        if let Ok(meta) = fs::symlink_metadata(Path::new(dir_path)) {
            total += meta.blocks();
        }
    }
    if let Ok(files) = fs::read_dir(dir_path) {
        for entry in files {
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

//-------------------------col_width FUNC
pub fn find_major_minor(path: &Path) -> Option<(u64, u64)> {
    let meta = fs::metadata(path).ok()?;
    if !(meta.file_type().is_block_device() || meta.file_type().is_char_device()) {
        return None;
    }
    let rdev = meta.rdev();
    let major = ({ libc::major(rdev) }) as u64;
    let minor = ({ libc::minor(rdev) }) as u64;
    Some((major, minor))
}

//-------------------------col_width FUNC
pub fn find_group_owner(info: (uid_t, bool)) -> String {
    let name = if info.1 {
        get_user_by_uid(info.0).map(|u| u.name().to_owned())
    } else {
        get_group_by_gid(info.0).map(|g| g.name().to_owned())
    };
    let s = info.0.to_string();
    match name {
        Some(v) =>
            v
                .to_str()
                .unwrap_or_else(|| &s.as_str())
                .to_string(),
        _ => info.0.to_string(),
    }
}

//-------------------------col_width FUNC
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

//-------------------------col_width FUNC
pub fn padding(file: FileData, width: usize, is_last: bool, flags: &Flags) -> String {
    // lines[i].ftype.file_color(files_name[i].clone())
    if is_last {
        Files::format_file(&file, flags)
    } else {
        let space = measure_text_width(&file.name);
        let pad = width.saturating_sub(space);
        format!("{}{} ", Files::format_file(&file, flags), " ".repeat(pad))
    }
}

pub fn sort_files(mut files: Vec<FileData>, flag: &Flags) -> Vec<FileData> {
    files.sort_by(|f1, f2| {
        let s1 = f1.name.as_str();
        let s2 = f2.name.as_str();

        let key1 = if s1.starts_with('.') { &s1[1..] } else { s1 };
        let key2 = if s2.starts_with('.') { &s2[1..] } else { s2 };

        let key1 = key1.trim().replace('-', "");
        let key2 = key2.trim().replace('-', "");

        key1.to_lowercase().cmp(&key2.to_lowercase())
    });

    let new_files: Vec<FileData> = files
        .into_iter()
        .filter(|f| flag.hidden_file(&f.name))
        .collect();

    new_files
}

//-------------------------col_width FUNC
pub fn is_executable(metadata: &Metadata) -> bool {
    (metadata.mode() & 0o111) != 0
}

//-------------------------col_width FUNC
pub fn parse_args(args: Vec<String>) -> Result<(Flags, Vec<String>), ()> {
    let mut flags = Flags::new();
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

//-------------------------process_dirs FUNC
pub fn process_dirs(path: &str, flags: &Flags) {
    let dir_data = match fs::read_dir(path) {
        Ok(e) => e,
        Err(_) => {
            println!("process dir");
            eprintln!("ls: cannot access '{}': Permission denied", path);
            return;
        }
    };
    let mut files: Vec<FileData> = dir_data
        .filter_map(Result::ok)
        .filter_map(|entry| FileData::from_path(&entry.path()))
        .collect();
    if flags.a_flag {
        if let Some(dot) = FileData::from_path(&Path::new(path).join(".")) {
            let mut dot = dot;
            dot.name = ".".to_string();
            files.push(dot);
        }

        if let Some(parent) = FileData::from_path(&Path::new(path).join("..")) {
            let mut parent = parent;
            parent.name = "..".to_string();
            files.push(parent);
        }
    }
    // println!("")
    list_directory(&sort_files(files, flags), flags, Path::new(path));
    // println!("FILES--> {:?}", files)
}

//-------------------------process_files FUNC
pub fn process_files(files: &[String], flags: &Flags) {
    let mut files_arr = Vec::new();

    for path in files {
        if let Some(file) = FileData::from_path(Path::new(path)) {
            files_arr.push(file);
        } else {
            eprintln!("ls: cannot access '{}'", path);
        }
    }
    if !files_arr.is_empty() {
        if flags.l_flag {
            display_line(build_line(&files_arr, flags), flags);
        } else {
            for file in &files_arr {
                print!("{} ", Files::format_file(file, flags));
            }
            println!()
        }
    }
}

//-------------------------print_now FUNC
pub fn print_now(lines: &Vec<FileData>, flags: &Flags) {
    let files_name: Vec<String> = lines
        .iter()
        .map(|s| {
            return s.name.clone();
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
        // println!("{}", files_name.join("  "));
        for i in 0..lines.len() {
            print!("{} ", Files::format_file(&lines[i], flags));
        }
        println!();
        return;
    }

    let rows = (files_name.len() + cols - 1) / cols;
    for row in 0..rows {
        for col in 0..cols {
            let i = col * rows + row;
            if i < files_name.len() {
                let is_last = col == cols - 1 || i + rows >= files_name.len();
                print!("{}", padding(lines[i].clone(), col_width, is_last, flags));
            }
        }
        println!();
    }
}

//-------------------------build_line FUNC
pub fn build_line(list_path: &Vec<FileData>, _flags: &Flags) -> Vec<(Vec<String>, FileData)> {
    let mut line = Vec::new();

    for f in list_path {
        let info = FileInfo::extract_data(f);

        let row = vec![
            info.f_permission,
            info.num_links.to_string(),
            info.owner_id,
            info.group_id,
            info.f_major,
            info.f_minor,
            info.format_time
        ];
        line.push((row, f.clone()));
    }
    line
}

//-------------------------display_line FUNC
pub fn display_line(lines: Vec<(Vec<String>, FileData)>, flag: &Flags) {
    let col_width: Vec<usize> = {
        let mut widths = vec![0; lines[0].0.len()];
        for (row, _) in &lines {
            for (i, elem) in row.iter().enumerate() {
                widths[i] = widths[i].max(elem.len());
            }
        }
        widths
    };

    for (row, file_data) in &lines {
        for (i, elem) in row.iter().enumerate() {
            let is_num = elem.chars().all(|c| (c.is_ascii_digit() || c == ','));
            if is_num {
                print!("{:>w$} ", elem, w = col_width[i]);
            } else {
                print!("{:<w$} ", elem, w = col_width[i]);
            }
        }
        if file_data.ftype == Files::Symlink {
            let target_name: &str = match &file_data.sym_path {
                Some(path) => path.to_str().unwrap_or(""),
                None => "",
            };

            if let Some(target_type) = &file_data.sym_type {
                print!(" {} -> {}", file_data.ftype.file_color(&file_data.name).to_string(), if
                    flag.f_flag
                {
                    target_type.file_symbol(&target_type.file_color(target_name))
                } else {
                    target_type.file_color(target_name).to_string()
                });
            }
        } else {
            print!(" {} ", Files::format_file(file_data, flag));
        }

        println!();
    }
}

//-------------------------list_directory FUNC
pub fn list_directory(list_path: &Vec<FileData>, flags: &Flags, path: &Path) {
    if !flags.l_flag {
        print_now(list_path, flags)
    } else {
        if flags.l_flag {
            println!("total {}", total_blocks(path, flags.a_flag));
        }
        display_line(build_line(list_path, flags), flags);
    }
}

//-------------------------build_permissions FUNC
pub fn build_permissions(meta: &Metadata, has_extra_attrs: bool) -> String {
    let mode = meta.permissions().mode();
    let file_type = Files::file_type(meta).file_char();
    let perm_chars = |bits: u32, r: u32, w: u32, x: u32| {
        let r_char = if (bits & r) != 0 { 'r' } else { '-' };
        let w_char = if (bits & w) != 0 { 'w' } else { '-' };
        let mut x_char = if (bits & x) != 0 { 'x' } else { '-' };
        if x == 0o100 && (bits & 0o4000) != 0 {
            x_char = if x_char == 'x' { 's' } else { 'S' };
        }
        if x == 0o010 && (bits & 0o2000) != 0 {
            x_char = if x_char == 'x' { 's' } else { 'S' };
        }
        if x == 0o001 && (bits & 0o1000) != 0 {
            x_char = if x_char == 'x' { 't' } else { 'T' };
        }
        format!("{}{}{}", r_char, w_char, x_char)
    };

    let mut perm_string = format!(
        "{}{}{}{}",
        file_type,
        perm_chars(mode, 0o400, 0o200, 0o100),
        perm_chars(mode, 0o040, 0o020, 0o010),
        perm_chars(mode, 0o004, 0o002, 0o001)
    );
    if has_extra_attrs {
        perm_string.push('+');
    }

    perm_string
}

pub fn handle_glob(paths: &mut Vec<String>) {
    let mut i = 0;
    while i < paths.len() {
        if paths[i] == "*" {
            paths.remove(i);
            match fs::read_dir(".") {
                Ok(path_data) => {
                    for dirs in path_data {
                        if let Ok(file) = dirs {
                            if let Ok(name) = file.file_name().into_string() {
                                paths.push(name);
                            }
                        }
                    }
                }
                Err(_) => {
                    eprintln!("ls: cannot read this dir");
                }
            }
        } else {
            i += 1;
        }
    }
}
