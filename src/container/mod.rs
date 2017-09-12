use std::fmt;

pub mod detector;

#[derive(Debug, Clone)]
pub enum ContainerEngine {
    Docker,
    Rocket,
    OpenVZ,
    LXC,
    Unknown,
    Maybe(Vec<String>),
}

impl fmt::Display for ContainerEngine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ContainerEngine::Maybe(ref engines) => { write!(f, "Maybe: {:?}", "maybe") }
            _ => { write!(f, "{:?}", self) }
        }
    }
}