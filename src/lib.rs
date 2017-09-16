pub mod common;
pub mod container;


#[cfg(test)]
mod tests {
    use std::path::Path;

    use common::{Detector, DetectorContext};
    use container::detector::ContainerDetector;
    use container::ContainerEngine;

    fn test_lxc() {
        let ref ctx = DetectorContext::new(Path::new("../tests/lxc"));
        assert_eq!(ContainerDetector::detect(ctx), ContainerEngine::LXC);
    }
}