use serde::Deserialize;

use std::fmt::Display;

use crate::isapi::{StatusCode, SubStatusCode};

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    #[serde(rename = "requestURL")]
    pub request_url: String,
    pub status_code: u8,
    pub status_string: String,
    pub sub_status_string: Option<String>,
    pub id: Option<u32>,
    pub sub_status_code: SubStatusCode,
    pub error_code: Option<u64>,
    pub error_msg: Option<String>,
    pub additional_err: Option<AdditionalErr>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct AdditionalErr {
    #[serde(rename = "AdditionalError")]
    pub additional_error: AdditionalError,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct AdditionalError {
    #[serde(rename = "StatusList")]
    pub status_list: StatusList,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct StatusList {
    #[serde(rename = "Status")]
    pub status: Status,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub id: Option<u32>,
    pub status_code: u8,
    pub status_string: String,
    pub sub_status_code: SubStatusCode,
}

#[derive(Debug, Clone)]
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
        write!(
            f,
            "{} -> ({:?}:{}:{:?})",
            self.request_url, self.status_code, self.status_string, self.sub_status_code
        )
    }
}

mod tests {
    #[test]
    fn test_parse_response() {
        use crate::isapi::{Response, SubStatusCode};
        use serde_xml_rs::from_str;

        let data = r##"<?xml version="1.0" encoding="UTF-8"?>
        <ResponseStatus version="2.0" xmlns="http://www.isapi.org/ver20/XMLSchema">
        <requestURL>/ISAPI/Streaming/channels/1/icr</requestURL>
        <statusCode>4</statusCode>
        <statusString>Invalid Operation</statusString>
        <subStatusCode>invalidOperation</subStatusCode>
        </ResponseStatus>"##;

        let response = Response {
            request_url: "/ISAPI/Streaming/channels/1/icr".to_string(),
            status_code: 4,
            status_string: "Invalid Operation".to_string(),
            sub_status_string: None,
            id: None,
            sub_status_code: SubStatusCode::InvalidOperation,
            error_code: None,
            error_msg: None,
            additional_err: None,
        };

        let second_test: Response = from_str(data).unwrap();
        assert_eq!(second_test, response);
    }
}
