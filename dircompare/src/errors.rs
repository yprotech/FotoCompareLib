use std::fmt;

#[derive(Debug)]
pub enum ScanError {
    IoError(std::io::Error),
    SendError(String),
}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScanError::IoError(e) => write!(f, "IO Error: {}", e),
            ScanError::SendError(e) => write!(f, "Send Error: {}", e),
        }
    }
}
