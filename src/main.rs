mod common;
mod container;

use std::path::Path;

use common::{Detector, DetectorContext};
use container::detector::ContainerDetector;

fn main() {
    let ref ctx = DetectorContext::new(Path::new("/root"));
    println!("{}", ContainerDetector::detect(ctx));
}
