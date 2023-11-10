use log::{debug, error, info, warn};

use std::{
    fs::{self, DirEntry, File},
    io::{Read, Write},
    path::PathBuf,
};

use crate::errors::Error;

const APP_DIR: &str = ".cache/runslate";

/// Clean all cache files in `APP_DIR`.
pub fn cache_clean() {
    let (success, sum) = cache_iter(|de: DirEntry| {
        if let Ok(()) = fs::remove_file(&de.path()) {
            info!("removed: {}", de.path().display());
            return true;
        }
        false
    });

    println!("Removed {success} file(s), {sum} in total.")
}

/// List all cache files in `APP_DIR`.
pub fn cache_show() {
    let (_, sum) = cache_iter(|de: DirEntry| {
        println!("{}", de.path().display());
        true
    });

    println!("{sum} file(s) in total.")
}

pub(crate) fn write_file(file_name: String, content: String) -> bool {
    let app_dir = home::home_dir().unwrap().join(APP_DIR);

    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).unwrap();
    }

    let file_path = app_dir.join(file_name);
    match File::create(&file_path) {
        Ok(mut file) => match file.write_all(content.as_bytes()) {
            Ok(()) => {
                debug!("Serialize to file {:?} successfully.", file_path.display());
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

pub(crate) fn read_file(file_name: &str) -> Result<String, Error> {
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

pub(crate) fn remove_file(file_name: &str) {
    let file_path = abs_path(file_name);
    if let Err(e) = fs::remove_file(file_path) {
        error!("Failed to remove cache file: {:#?}", e);
    }
}

/// Iterate files in `APP_DIR` and apply enclosure on every [`DirEntry`]
fn cache_iter<F>(f: F) -> (usize, usize)
where
    F: Fn(DirEntry) -> bool,
{
    let app_dir = home::home_dir().unwrap().join(APP_DIR);
    let paths = fs::read_dir(app_dir).unwrap();
    let mut sum = 0;
    let mut success = 0;

    for pth in paths {
        if let Ok(de) = pth {
            if f(de) {
                success += 1;
            }
            sum += 1;
        }
    }

    (success, sum)
}

fn abs_path(file_name: &str) -> PathBuf {
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
