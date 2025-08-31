use chrono::{ DateTime, Duration, Utc };
use std::fs::{ Metadata, Permissions };
use std::path::{ Path };
use std::{ fs, os::unix::fs::FileTypeExt };
use users::{ get_group_by_gid, get_user_by_uid };
use std::os::unix::fs::MetadataExt;
use crate::commands::ls::ls_models::{ Flags, Files };
use libc::{ self, uid_t };
// use exacl::{ Acl, Perm };
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
    println!("PQTH {:?} {:?}", dir_path, flag_a);
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

pub fn file_data(
    file_name: &str,
    path_name: &str
) -> (String, u64, String, String, String, String, String) {
    // println!("OUTPUT--> {} {}", path_name.cyan(), file_name.cyan());
    let mut file_permission = String::new();
    let mut num_links: u64 = 0;
    let mut owner_id = String::new();
    let mut group_id = String::new();
    let mut f_major = String::new();
    let mut f_minor = String::new();
    let mut format_time = String::new();
    let path = format!("{}/{}", path_name, file_name);
    let meta: Metadata;
    let permissions: Permissions;
    if let Ok(meta_d) = fs::symlink_metadata(Path::new(&path)) {
        permissions = meta_d.permissions();
        meta = meta_d;
    } else if let Ok(meta_d) = fs::symlink_metadata(Path::new(&file_name)) {
        // println!("Permissions: {:?}", meta_d.permissions());
        permissions = meta_d.permissions();
        meta = meta_d;
    } else {
        panic!("can't read data");
    }
    num_links = meta.nlink();

    match find_major_minor(&path) {
        Some((major, minor)) => {
            // file_size = format!("{},   {}", major, minor);
            f_major.push_str(&format!("{},", &major.to_string()));
            f_minor.push_str(&minor.to_string());
        }
        _ => {
            f_minor.push_str(&meta.len().to_string());
        }
    }
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
    if Files::has_extra_attrs(&path) {
        file_permission.push('+');
    }
    (file_permission, num_links, owner_id, group_id, f_major, f_minor, format_time)
}
