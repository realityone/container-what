use std::path::{Path, PathBuf};

pub struct DetectorContext<'a> {
    root: &'a Path
}

impl<'a> DetectorContext<'a> {
    pub fn new(path: &'a Path) -> Self {
        DetectorContext {
            root: path
        }
    }

    pub fn get_file_path(&self, path_str: &str) -> PathBuf {
        let mut result = self.root.to_path_buf();
        result.push(path_str);
        result
    }
}

pub trait Detector {
    type D: ::std::fmt::Display;
    fn detect(ctx: &DetectorContext) -> Self::D;
}