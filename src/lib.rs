#![allow(dead_code)]

pub mod common;
pub mod container;


#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::env;

    use common::{Detector, DetectorContext};
    use container::detector::ContainerDetector;
    use container::ContainerEngine;

    fn test_case_path_str(case: &str) -> String {
        let mut base_dir = env::var("TESTS_PATH").unwrap_or("/root".to_string());
        if !base_dir.ends_with("/") {
            base_dir.push('/');
        }
        base_dir.push_str(case);
        base_dir
    }

    #[test]
    fn test_lxc() {
        let ref lxc_path = test_case_path_str("lxc");
        let ref ctx = DetectorContext::new(Path::new(lxc_path));
        assert_eq!(ContainerDetector::detect(ctx), ContainerEngine::LXC);
    }

    #[test]
    fn test_docker() {
        let ref docker_path = test_case_path_str("docker");
        let ref ctx = DetectorContext::new(Path::new(docker_path));
        assert_eq!(ContainerDetector::detect(ctx), ContainerEngine::Docker);
    }
}