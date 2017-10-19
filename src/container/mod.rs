use std::fmt;

pub mod detector;

#[derive(Debug, Clone, PartialEq)]
pub enum ContainerEngine {
    Docker,
    Rocket,
    OpenVZ,
    LXC,
    RKT,
    Unknown,
    Maybe(Vec<String>),
}

impl fmt::Display for ContainerEngine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ContainerEngine::Maybe(ref engines) => write!(f, "Maybe({})", engines.join(", ")),
            _ => write!(f, "{:?}", self),
        }
    }
}
