# container-what
Detect if we are running in a container.

Looks like [virt-what](https://people.redhat.com/~rjones/virt-what/), but aim to container engines.

More container engines will be added.

## Installation

Put this crate in your `Cargo.toml`.

```Toml
[dependencies]
container_what = "*"
```

## Usage

```Rust
extern crate container_what;

use std::path::Path;

use container_what::container::ContainerEngine;
use container_what::common::{Detector, DetectorContext};
use container_what::container::detector::ContainerDetector;


fn main() {
    // Specify the detect root
    let ref ctx = DetectorContext::new(Path::new("/"));
    assert_eq!(ContainerDetector::detect(ctx), ContainerEngine::Docker);
}
```

## License
container-what is licensed under the MIT License - see the 
[LICENSE](https://github.com/realityone/container-what/blob/master/LICENSE) file for details