use thiserror::Error;

#[derive(Error, Debug)]
pub enum HikvisionError {
    #[error(transparent)]
    Std {
        #[from]
        source: std::io::Error,
    },
    #[error(transparent)]
    Digest {
        #[from]
        source: digest::DigestError,
    },
    #[error(transparent)]
    Reqwest {
        #[from]
        source: reqwest::Error,
    },
    #[error("params to connection is not set")]
    NotSet,
    #[error("api is not supported")]
    NotAvialiableApi,
    #[error("error with setting|getting spotlight to camera")]
    Spotlight,
}

impl From<std::io::ErrorKind> for HikvisionError {
    fn from(error: std::io::ErrorKind) -> Self {
        HikvisionError::Std {
            source: error.into(),
        }
    }
}
