use digest::DigestAuth;
use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};
use serde_xml_rs::{from_str, to_string};
use url::Url;

use std::io::ErrorKind;

use crate::utils::{
    common_types::{AuthType, RequestType},
    error::HikvisionError,
};

#[derive(Default, Debug)]
pub struct RequestHandler {
    user: Option<String>,
    pass: Option<String>,
}

impl RequestHandler {
    pub async fn send<S>(&self, url: &str, params: S) -> Result<(), HikvisionError>
    where
        S: Serialize + Send + 'static + std::fmt::Debug,
    {
        if self.user.is_none() || self.pass.is_none() {
            return Err(HikvisionError::NotSet);
        }

        let _ = Url::parse(&url.to_string())?;
        let params = format!(
            r#"{}"#,
            to_string(&params).map_err(|_| ErrorKind::InvalidInput)?
        );

        let _ = self._send(url, Some(params)).await?;
        Ok(())
    }

    pub async fn recieve<D>(&self, url: &str) -> Result<D, HikvisionError>
    where
        D: DeserializeOwned,
    {
        if self.user.is_none() || self.pass.is_none() {
            return Err(HikvisionError::NotSet);
        }

        let _ = Url::parse(&url.to_string())?;
        let response = self._retrieve(url).await?;

        Ok(from_str(&response).map_err(|_| HikvisionError::NotAvialiableApi)?)
    }

    async fn _send(&self, url: &str, data: Option<String>) -> Result<String, HikvisionError> {
        request(
            url,
            (self.user.clone(), self.pass.clone()),
            AuthType::Digest,
            RequestType::Send,
            data,
        )
        .await
    }

    async fn _retrieve(&self, url: &str) -> Result<String, HikvisionError> {
        request(
            url,
            (self.user.clone(), self.pass.clone()),
            AuthType::Digest,
            RequestType::Send,
            None,
        )
        .await
    }
}

async fn request(
    url: &str,
    auth: (Option<String>, Option<String>),
    auth_type: AuthType,
    request_type: RequestType,
    data: Option<String>,
) -> Result<String, HikvisionError> {
    use AuthType::*;
    use RequestType::*;

    let client = Client::new();
    let request = match request_type {
        Retrieve => client.get(url),
        Send => {
            if data.is_none() {
                return Err(HikvisionError::NotSet);
            } else {
                client.put(url).body(data.unwrap())
            }
        }
    };

    let (auth_type, user, pass) = match auth {
        (Some(u), Some(p)) => (auth_type, Some(u), Some(p)),

        (None, Some(p)) => {
            if auth_type == AuthType::Bearer {
                (auth_type, None, Some(p))
            } else {
                (AuthType::default(), None, None)
            }
        }

        (Some(u), None) => {
            if auth_type == AuthType::Basic {
                (auth_type, Some(u), None)
            } else {
                (AuthType::default(), None, None)
            }
        }

        (None, None) => (AuthType::default(), None, None),
    };

    let request = match auth_type {
        Basic => request.basic_auth(user.unwrap(), pass),
        Digest => request.digest_auth(&user.unwrap(), &pass.unwrap()).await?,
        Bearer => request.bearer_auth(pass.unwrap()),
        _ => request,
    };

    let response = request.send().await?.text().await?;

    Ok(response)
}
