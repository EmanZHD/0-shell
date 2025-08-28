use chrono::{ DateTime, Duration, Utc };
use colored::Colorize;
use std::path::{ Path };
use std::{ fs, os::unix::fs::FileTypeExt };
use users::{ get_group_by_gid, get_user_by_uid };
use std::os::unix::fs::MetadataExt;
use crate::commands::ls::ls_models::{ Flags };
use libc::{ self, file_clone_range, uid_t };
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

pub fn file_permission(
    file_name: &str,
    path_name: &str
) -> (String, u64, String, String, String, String) {
    // println!("OUTPUT--> {} {}", path_name.cyan(), file_name.cyan());
    let mut file_permission = String::new();
    let mut num_links: u64 = 0;
    let mut owner_id = String::new();
    let mut group_id = String::new();
    let mut file_size = String::new();
    let mut format_time = String::new();
    let path = format!("{}/{}", path_name, file_name);

    if let Ok(meta) = fs::symlink_metadata(Path::new(&path)) {
        // if file_name == "libmtp-1-4" {
        //     println!("META -> {:?}", meta);
        // }
        let permissions = meta.permissions();
        num_links = meta.nlink();

        match find_major_minor(&path) {
            Some((major, minor)) => {
                file_size = format!("{},   {}", major, minor);
            }
            _ => {
                file_size = format!("    {}", meta.len().to_string().on_bright_cyan());
            }
        }
        // file_size = meta.len();
        owner_id.push_str(&find_group_owner((meta.uid(), true)));
        group_id.push_str(&find_group_owner((meta.gid(), false)));

        // let mode = permissions.mode();
        if let Ok(time) = meta.modified() {
            let datetime: DateTime<Utc> = DateTime::from(time) + Duration::hours(1);
            format_time = datetime.format("%b %e %H:%M").to_string();
        }

        let str_prm: String = format!("{:?}", permissions.to_owned());
        // println!("PATH -> {} PERMISSIONS-----> {}", path.red(), str_prm.yellow().bold());
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
    } else {
        // println!("CAtch it--> {}", path.on_bright_green());
        if let Ok(meta) = fs::symlink_metadata(Path::new(&path)) {
            println!("Permissions: {:?}", meta.permissions());
        }
    }

    (file_permission, num_links, owner_id, group_id, file_size, format_time)
}
