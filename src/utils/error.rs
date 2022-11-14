use crate::isapi::Response;
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
    #[error(transparent)]
    Url {
        #[from]
        source: url::ParseError,
    },
    #[error(transparent)]
    Response {
        #[from]
        source: crate::isapi::ErrorCode,
    },
}

impl From<std::io::ErrorKind> for HikvisionError {
    fn from(error: std::io::ErrorKind) -> Self {
        HikvisionError::Std {
            source: error.into(),
        }
    }
}

impl From<Response> for HikvisionError {
    fn from(r: Response) -> Self {
        match r.error_code {
            Some(ec) => HikvisionError::Response { source: ec.into() },
            None => HikvisionError::Std {
                source: std::io::Error::new(std::io::ErrorKind::InvalidData, "Response error"),
            },
        }
    }
}
