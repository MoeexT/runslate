use log::{debug, error, info, warn};

use std::{
    fs::{self, DirEntry, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use crate::errors::Error;

const APP_DIR: &str = ".cache/runslate";
const UNITS: &'static [&str; 4] = &["B", "KB", "MB", "GB"];

pub fn write_string<P: AsRef<Path>>(file_name: P, content: String) -> bool {
    let app_dir = home::home_dir().unwrap().join(APP_DIR);

    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).unwrap();
    }

    let file_path = app_dir.join(file_name);
    match File::create(&file_path) {
        Ok(mut file) => match file.write_all(content.as_bytes()) {
            Ok(()) => {
                info!("Serialize to file {:?} successfully.", file_path.display());
                return true;
            }
            Err(e) => {
                error!("Write file ({:?}) failed: {:?}", file_path.display(), e);
                return false;
            }
        },
        Err(e) => {
            error!("Open file ({:?}) failed: {:?}", file_path.display(), e);
            return false;
        }
    }
}

pub fn read_string<P: AsRef<Path>>(file_name: P) -> Result<String, Error> {
    let file_path = abs_path(file_name);

    if !file_path.exists() {
        let msg = format!("Cache file doesn't exit: {}.", file_path.display());
        warn!("{}", msg);
        return Err(Error::CacheNotFound(msg));
    }

    if let Ok(mut file) = File::open(&file_path) {
        info!("Reading content from cache file {}", &file_path.display());
        let mut content = String::new();

        if let Ok(_) = file.read_to_string(&mut content) {
            return Ok(content);
        }

        let msg = format!("Read file ({:?}) failed", file_path.display());
        error!("{}", msg);

        return Err(Error::ReadFileError(msg));
    }

    let msg = format!("Open file ({:?}) failed", file_path.display());
    error!("{}", &msg);

    Err(Error::OpenFileError(msg))
}

pub fn remove_file<P: AsRef<Path>>(file_name: P) {
    let file_path = abs_path(file_name);
    if let Err(e) = fs::remove_file(file_path) {
        warn!("Failed to remove cache file: {:#?}", e);
    }
}

/// Iterate files in `APP_DIR` and apply enclosure on every [`DirEntry`]
pub fn iter_dir<F>(mut f: F) -> (usize, usize)
where
    F: FnMut(DirEntry) -> bool,
{
    let app_dir = home::home_dir().unwrap().join(APP_DIR);
    let paths = fs::read_dir(app_dir).unwrap();
    let mut sum = 0;
    let mut success = 0;

    for pth in paths {
        if let Ok(de) = pth {
            if de.path().is_dir() {
                debug!("{:?} is a directory.", de.path());
                continue
            }
            if f(de) {
                success += 1;
            }
            sum += 1;
        }
    }

    (success, sum)
}

/// Convert file size to readable format.
///
/// # Examples
///
/// ```
/// use runslate::utils::file::fmt_size;
///
/// let (size, unit) = fmt_size(1024);
/// assert_eq!(format!("{}{}", size, unit), "1.0KB");
///
/// let (size, unit) = fmt_size(1 << 32);
/// println!("{}{}", size, unit);
/// assert_eq!(format!("{}{}", size, unit), "4.0GB");
/// ```
pub fn fmt_size(size: u64) -> (String, String) {
    let mut uniti = 0usize;
    let mut size = size as f64;

    while size >= 1000.0 && uniti < UNITS.len() - 1 {
        size /= 1024.0;
        uniti += 1;
    }

    // 233B instead of 233.0B
    let size = if uniti == 0 {
        format!("{:.0}", size)
    } else {
        format!("{:.1}", size)
    };

    (size, String::from(UNITS[uniti]))
}

fn abs_path<P: AsRef<Path>>(file_name: P) -> PathBuf {
    if file_name.as_ref().is_absolute() {
        return file_name.as_ref().to_path_buf();
    }
    let app_dir = home::home_dir().unwrap().join(APP_DIR);
    app_dir.join(file_name)
}

#[test]
fn test_save() {
    let app_dir = home::home_dir().unwrap().join(APP_DIR);

    if !app_dir.exists() {
        fs::create_dir_all(app_dir).unwrap();
    }
}
