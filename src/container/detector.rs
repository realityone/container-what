use std::io;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

use super::ContainerEngine;
use common::{Detector, get_file_path};

pub struct ContainerDetector;

fn read_to_string(path: &Path) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

impl ContainerDetector {
    fn is_openvz() -> bool {
        // from virt-what
        let vz_path = get_file_path("proc/vz");
        let bc_path = get_file_path("proc/bc");
        if vz_path.is_dir() && !bc_path.exists() {
            return true;
        }
        false
    }

    fn is_lxc() -> bool {
        let init_proc_env_path = get_file_path("proc/1/environ");
        match read_to_string(init_proc_env_path.as_path()) {
            Ok(content) => {
                for line in content.split("\x00") {
                    if line.starts_with("container=") {
                        return true;
                    }
                }
                return false;
            }
            Err(_) => {
                return false;
            }
        }
    }
}

impl Detector for ContainerDetector {
    type D = ContainerEngine;

    fn detect() -> ContainerEngine {
        if Self::is_openvz() {
            return ContainerEngine::OpenVZ;
        }
        if Self::is_lxc() {
            return ContainerEngine::LXC;
        }

        return ContainerEngine::Unknown;
    }
}