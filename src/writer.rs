use std::fmt::{Debug, Formatter};
use std::io::Write;
use std::sync::Mutex;

pub enum Writer {
    Stdout(#[cfg(feature = "colors")] bool),

    Stderr(#[cfg(feature = "colors")] bool),

    Other(Box<Mutex<dyn Write + Sync + Send + 'static>>),
}

impl Default for Writer {
    fn default() -> Self {
        Writer::Stdout(
            #[cfg(feature = "colors")]
            false,
        )
    }
}

impl Debug for Writer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Writer::Stdout(_) => write!(f, "Stdout"),
            Writer::Stderr(_) => write!(f, "Stderr"),
            Writer::Other(_) => write!(f, "Other"),
        }
    }
}
