use serde::{Deserialize, Serialize};

use std::fmt::Display;

use crate::isapi::{ErrorCode, StatusCode, SubStatusCode};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    #[serde(rename = "requestURL")]
    pub request_url: String,
    pub status_code: u8,
    pub status_string: String,
    pub sub_status_string: Option<String>,
    pub id: Option<u32>,
    pub sub_status_code: SubStatusCode,
    pub error_code: Option<ErrorCode>,
    pub error_msg: Option<String>,
    pub additional_err: Option<AdditionalErr>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalErr {
    #[serde(rename = "AdditionalError")]
    pub additional_error: AdditionalError,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalError {
    #[serde(rename = "StatusList")]
    pub status_list: StatusList,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusList {
    #[serde(rename = "Status")]
    pub status: Status,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub id: Option<u32>,
    pub status_code: u8,
    pub status_string: String,
    pub sub_status_code: SubStatusCode,
}

#[derive(Debug)]
pub struct SimpleResponse {
    pub request_url: String,
    pub status_code: StatusCode,
    pub status_string: String,
    pub sub_status_code: SubStatusCode,
}

impl From<Response> for SimpleResponse {
    fn from(r: Response) -> Self {
        Self {
            request_url: r.request_url,
            status_code: r.status_code.into(),
            status_string: r.status_string,
            sub_status_code: r.sub_status_code,
        }
    }
}

impl Display for SimpleResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> ({:?}:{}:{:?})", self.request_url, self.status_code, self.status_string, self.sub_status_code)
    }
}
