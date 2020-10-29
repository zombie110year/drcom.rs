use std::error::Error;

#[derive(Debug)]
pub enum DrcomException {
    // std::io::Error
    StdIOError(std::io::Error),
    // challenge 失败
    ChallengeRemoteDenied,
}

impl std::fmt::Display for DrcomException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DrcomException::StdIOError(err) => write!(f, "{}", err),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl Error for DrcomException {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DrcomException::StdIOError(err) => err.source(),
            _ => Some(self),
        }
    }
}

impl From<std::io::Error> for DrcomException {
    fn from(e: std::io::Error) -> Self {
        DrcomException::StdIOError(e)
    }
}
