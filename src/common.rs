use std::sync::Mutex;
use std::path::{Path, PathBuf};

lazy_static! {
    static ref ROOT: Mutex<PathBuf> = Mutex::new(PathBuf::new());
}

#[inline]
pub fn get_file_path(path_str: &str) -> PathBuf {
    let root;
    {
        root = ROOT.lock().unwrap();
    }
    let mut result = root.clone();
    result.push(path_str);
    result
}

#[inline]
pub fn setup_root(path_str: &str) {
    {
        *ROOT.lock().unwrap() = PathBuf::from(path_str);
    }
}

pub trait Detector {
    type D: ::std::fmt::Display;
    fn detect() -> Self::D;
}