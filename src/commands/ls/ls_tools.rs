use chrono::{ DateTime, Duration, Utc };
use std::path::Path;
use std::{ fs, os::unix::fs::FileTypeExt };
use users::{ get_group_by_gid, get_user_by_uid };
use std::os::unix::fs::MetadataExt;
use crate::commands::ls::ls_mod::{ Flags };

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
    let major = (rdev >> 8) & 0xfff;
    let minor = (rdev & 0xff) | ((rdev >> 12) & 0xfff00);

    Some((major, minor))
}

pub fn file_permission(
    file_name: &str,
    path_name: &str
) -> (String, u64, String, String, String, String) {
    // println!("FILENAME==> {:?}", file_name);
    let mut file_permission = String::new();
    let mut num_links: u64 = 0;
    let mut owner_id = String::new();
    let mut group_id = String::new();
    let mut file_size = String::new();
    let mut format_time = String::new();
    let path = format!("{}/{}", path_name, file_name);
    if let Ok(meta) = fs::metadata(Path::new(&path)) {
        let permissions = meta.permissions();
        num_links = meta.nlink();
        match find_major_minor(&path) {
            Some((major, minor)) => {
                file_size = format!("{},   {}", major, minor);
            }
            _ => {
                file_size = format!("    {}", meta.len().to_string());
            }
        }
        // file_size = meta.len();
        if
            let Some(name) = get_user_by_uid(meta.uid()).and_then(|user|
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
            let Some(name) = get_group_by_gid(meta.gid()).and_then(|user|
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
