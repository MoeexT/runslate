use std::{
    cmp,
    collections::HashMap,
    fs::{self, DirEntry},
};

use log::{error, info};
use serde_json::Value;

use crate::{
    cache::util::unpack,
    errors::Error,
    utils::file::{fmt_size, iter_dir}, translators::Translators,
};

/// List all cache files in `APP_DIR`.
pub fn list() {
    let mut s_size = 0u64;
    let mut c_valid = 0usize;
    let mut ff_size_len = 0usize;
    let mut lines = vec![];

    let (_, sum) = iter_dir(|de: DirEntry| {
        let f_size = fs::metadata(de.path()).unwrap().len();
        let ff_size = fmt_size(f_size);
        ff_size_len = cmp::max(ff_size_len, ff_size.0.len());

        let is_expired = match unpack(de.file_name()) {
            Ok(_) => {
                c_valid += 1;
                String::from("✔")
            }
            Err(_) => String::from("✘"),
        };

        lines.push((is_expired, ff_size, de.file_name()));
        s_size += f_size;
        true
    });

    for (is_expired, (ff_size, ff_size_u), f_name) in lines {
        println!(
            "{is_expired} {: <ff_size_len$}{: >2} {:}",
            ff_size,
            ff_size_u,
            f_name.to_str().unwrap_or("")
        );
    }

    let (s_size, s_size_u) = fmt_size(s_size);
    println!("{c_valid}/{sum} valid file(s), {s_size} {s_size_u} in total.")
}

/// Clean all cache files in `APP_DIR`.
pub fn clean() {
    let mut s_size = 0u64;

    let (del, sum) = iter_dir(|de: DirEntry| {
        let f_size = fs::metadata(de.path()).unwrap().len();
        match fs::remove_file(de.path()) {
            Ok(..) => {
                s_size += f_size;
                info!("removed: {:?}", de.file_name());
                true
            }
            Err(..) => false,
        }
    });

    let (s_size, s_size_u) = fmt_size(s_size);
    println!(
        "Removed {}/{} file(s), {} {}, file(s) in total.",
        del, sum, s_size, s_size_u
    )
}

/// Remove expired file
pub fn purge() {
    let mut s_size = 0u64;

    let (del, sum) = iter_dir(|de: DirEntry| match unpack(de.file_name()) {
        Ok(..) => false,
        Err(..) => {
            let f_size = fs::metadata(de.path()).unwrap().len();
            match fs::remove_file(de.path()) {
                Err(..) => false,
                Ok(..) => {
                    s_size += f_size;
                    info!("removed: {:?}", de.file_name());
                    true
                }
            }
        }
    });

    let (s_size, s_size_u) = fmt_size(s_size);
    println!(
        "Removed {}/{} file(s), {} {}, file(s) in total.",
        del, sum, s_size, s_size_u
    )
}

/// View cache file content
pub fn view(hash: String) {
    iter_dir(|de: DirEntry| {
        if de.file_name().to_string_lossy().starts_with(&hash) {
            match unpack(de.file_name()) {
                Ok(record) => match record.t_type {
                    Translators::Google => match serde_json::from_str::<HashMap<String, Value>>(&record.data) {
                    Ok(value) => {
                        Google::show()
                    },
                    Err(e) => error!("Deserialize value failed: {e}"),
                },
                    Translators::Youdao => todo!(),
                },
                Err(..) => error!("Deserialize file failed"),
            }
        }
        true
    });
}

mod test {
    #[cfg(test)]
    use crate::utils::file::fmt_size;

    #[test]
    fn test_format() {
        let indent = 10;
        println!("{: >indent$}", "str");
        println!("{: <10}", "str");
    }

    #[test]
    fn test_fmt_size() {
        let (size, unit) = fmt_size(999);
        println!("{}{}", size, unit);
        let (size, unit) = fmt_size(1000);
        println!("{}{}", size, unit);
        let (size, unit) = fmt_size(1024);
        assert_eq!(format!("{}{}", size, unit), "1.0KB");
        let (size, unit) = fmt_size(1124);
        println!("{}{}", size, unit);
        let (size, unit) = fmt_size(1 << 20);
        println!("{}{}", size, unit);
        let (size, unit) = fmt_size(1 << 24);
        println!("{}{}", size, unit);
        let (size, unit) = fmt_size(1 << 32);
        println!("{}{}", size, unit);
        assert_eq!(format!("{}{}", size, unit), "4.0GB");
        let (size, unit) = fmt_size(1 << 42);
        println!("{}{}", size, unit);
    }
}
