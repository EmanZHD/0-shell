use std::path::{ Path, PathBuf };
use std::fs::{ Metadata };
use std::{ fs };
use std::os::unix::fs::{ MetadataExt, FileTypeExt };
use crate::commands::ls::ls_tools::{
    is_executable,
    find_group_owner,
    find_major_minor,
    format_time,
    build_permissions,
};
use colored::{ ColoredString, Colorize };
use xattr::list;

//-------------------------FileInfo STRUCT
#[derive(Debug)]
pub struct FileInfo {
    pub f_permission: String,
    pub num_links: u64,
    pub owner_id: String,
    pub group_id: String,
    pub f_major: String,
    pub f_minor: String,
    pub format_time: String,
}

impl FileInfo {
    pub fn extract_data(file: &FileData) -> Self {
        let mut f_major = String::new();
        let mut f_minor = String::new();
        let meta: &Metadata = &file.metadata;
        let num_links = meta.nlink();

        if meta.file_type().is_char_device() || meta.file_type().is_block_device() {
            if let Some((major, minor)) = find_major_minor(&file.full_path) {
                f_major.push_str(&format!("{},", major));
                f_minor.push_str(&minor.to_string());
            }
        } else {
            f_minor.push_str(&meta.len().to_string());
        }

        let owner_id = find_group_owner((meta.uid(), true));
        let group_id = find_group_owner((meta.gid(), false));
        let format_time = format_time(meta);
        let f_permission = build_permissions(meta, Files::has_extra_attrs(&file.full_path));
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

//-------------------------Files STRUCT
#[derive(Debug, PartialEq, Clone)]
pub enum Files {
    Symlink,
    Dir,
    Socket,
    Exec,
    Fifo,
    Reg,
    BlockDev,
    CharDev,
}

impl Files {
    pub fn file_type(metadata: &Metadata) -> Files {
        let ft = metadata.file_type();
        if ft.is_symlink() {
            Files::Symlink
        } else if ft.is_dir() {
            Files::Dir
        } else if ft.is_socket() {
            Files::Socket
        } else if ft.is_fifo() {
            Files::Fifo
        } else if ft.is_block_device() {
            Files::BlockDev
        } else if ft.is_char_device() {
            Files::CharDev
        } else if is_executable(metadata) {
            Files::Exec
        } else {
            Files::Reg
        }
    }

    pub fn file_char(&self) -> char {
        match self {
            Files::Dir => 'd',
            Files::Symlink => 'l',
            Files::Socket => 's',
            Files::Fifo => 'p',
            Files::BlockDev => 'b',
            Files::CharDev => 'c',
            Files::Exec => '-',
            Files::Reg => '-',
        }
    }
    pub fn file_color(&self, path_str: &str) -> ColoredString {
        match self {
            Files::Dir => path_str.bold().blue(),
            Files::Exec => path_str.bold().green(),
            Files::Socket => path_str.bold().magenta(),
            Files::Fifo | Files::BlockDev | Files::CharDev => path_str.bold().yellow().on_black(),
            Files::Symlink => path_str.bold().cyan(),
            _ => path_str.white(),
        }
    }

    pub fn file_symbol(&self, path_str: &ColoredString) -> String {
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

    pub fn has_extra_attrs(path: &Path) -> bool {
        match list(path) {
            Ok(mut attrs) => attrs.next().is_some(),
            Err(_) => false,
        }
    }

    pub fn format_file(file: &FileData, flags: &Flags) -> String {
        if flags.f_flag {
            return file.ftype.file_symbol(&file.ftype.file_color(&file.name));
        } else {
            return file.ftype.file_color(&file.name).to_string();
        }
    }
}

//-------------------------FLAGS STRUCT
#[derive(Debug, Default)]
pub struct Flags {
    pub f_flag: bool,
    pub a_flag: bool,
    pub l_flag: bool,
}

impl Flags {
    pub fn new() -> Self {
        Flags { l_flag: false, a_flag: false, f_flag: false }
    }
    pub fn hidden_file(&self, name: &str) -> bool {
        self.a_flag || !name.starts_with('.')
    }
}

//-------------------------FileData STRUCT
#[derive(Debug, Clone)]
pub struct FileData {
    pub name: String,
    pub full_path: PathBuf,
    pub metadata: Metadata,
    pub ftype: Files,
    pub sym_path: Option<PathBuf>,
    pub sym_type: Option<Files>,
}

impl FileData {
    pub fn from_path(path: &Path) -> Option<Self> {
        let metadata = fs::symlink_metadata(path).ok()?;
        let name = path
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.to_string_lossy().into_owned());
        let ftype = Files::file_type(&metadata);

        let (sym_path, sym_type) = if ftype == Files::Symlink {
            if let Ok(target) = fs::read_link(path) {
                let abs_target = if target.is_absolute() {
                    target.clone()
                } else {
                    path.parent()
                        .unwrap_or_else(|| Path::new("/"))
                        .join(&target)
                };

                let target_type = fs
                    ::metadata(&abs_target)
                    .ok()
                    .map(|m| Files::file_type(&m));

                (Some(abs_target), target_type)
            } else {
                (None, None)
            }
        } else {
            (None, None)
        };

        Some(FileData {
            name,
            full_path: path.to_path_buf(),
            metadata,
            ftype,
            sym_path,
            sym_type,
        })
    }
}
