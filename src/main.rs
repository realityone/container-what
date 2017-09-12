#[macro_use]
extern crate lazy_static;

mod common;
mod container;

use common::Detector;

fn main() {
    println!("Container Engine: {}", container::detector::ContainerDetector::detect());
}
