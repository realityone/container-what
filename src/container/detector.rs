use std::io;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

use super::ContainerEngine;
use common::{Detector, DetectorContext};

pub struct ContainerDetector;

fn read_to_string(path: &Path) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

impl ContainerDetector {
    fn is_openvz(ctx: &DetectorContext) -> bool {
        // from virt-what
        let vz_path = ctx.get_file_path("proc/vz");
        let bc_path = ctx.get_file_path("proc/bc");
        if vz_path.is_dir() && !bc_path.exists() {
            return true;
        }
        false
    }

    fn is_lxc(ctx: &DetectorContext) -> bool {
        let init_proc_env_path = ctx.get_file_path("proc/1/environ");
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

    fn detect(ctx: &DetectorContext) -> ContainerEngine {
        if Self::is_openvz(ctx) {
            return ContainerEngine::OpenVZ;
        }
        if Self::is_lxc(ctx) {
            return ContainerEngine::LXC;
        }

        return ContainerEngine::Unknown;
    }
}