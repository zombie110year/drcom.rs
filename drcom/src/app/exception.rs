use std::error::Error;

#[derive(Debug)]
pub enum DrcomException {
    // std::io::Error
    StdIOError(std::io::Error),
    // challenge 失败
    ChallengeRemoteDenied,
    // 帐号密码错误
    AccountError,
    // 帐号处于停机状态
    AccountStopped,
    // 帐号已欠费
    AccountOutOfCost,
    // 未知的登录错误
    LoginError,
    // keep_alive_1 出错
    KeepAlive1,
    KeepAlive2,
    KeepAlive3,
    KeepAlive4,
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
