#[macro_use]
extern crate lazy_static;

mod common;
mod container;

use common::Detector;
use container::detector::ContainerDetector;

fn main() {
    println!("{}", ContainerDetector::detect());
}
