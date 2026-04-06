pub enum Quality {
    HD1080,
    HD720,
    SD480,
}

impl std::fmt::Display for Quality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Quality::HD1080 => write!(f, "1080p"),
            Quality::HD720 => write!(f, "720p"),
            Quality::SD480 => write!(f, "480p"),
        }
    }
}
