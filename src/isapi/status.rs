use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum StatusCode {
    OK,
    DeviceBusy,
    DeviceError,
    InvalidOperation,
    InvalidXMLFormat,
    InvalidXMLContent,
    RebootRequired,
    AdditionalError,
    Unknown,
}

impl From<u8> for StatusCode {
    fn from(sc: u8) -> Self {
        use StatusCode::*;

        match sc {
            0 | 1 => OK,
            2 => DeviceBusy,
            3 => DeviceError,
            4 => InvalidOperation,
            5 => InvalidXMLFormat,
            6 => InvalidXMLContent,
            7 => RebootRequired,
            8 => AdditionalError,
            9 => AdditionalError,
            _ => Unknown,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SubStatusCode {
    OK,
    Error,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ErrorCode {
    OK,
    Error,
}
