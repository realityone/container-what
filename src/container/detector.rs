use super::ContainerEngine;
use common::{Detector, get_file_path};

pub struct ContainerDetector;

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
        false
    }
}

impl Detector for ContainerDetector {
    type D = ContainerEngine;

    fn detect() -> ContainerEngine {
        if Self::is_openvz() {
            return ContainerEngine::OpenVZ;
        }

        return ContainerEngine::Unknown;
    }
}