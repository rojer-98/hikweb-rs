use digest::DigestAuth;
use reqwest::Client;

use crate::utils::{
    common_types::{AuthType, RequestType},
    error::HikvisionError,
};

#[derive(Default, Debug)]
pub struct Handler {
    user: Option<String>,
    pass: Option<String>,
}

impl Handler {
    pub async fn send(&self, url: &str, data: Option<String>) -> Result<String, HikvisionError> {
        request(url, (self.user.clone(), self.pass.clone()), AuthType::Digest, RequestType::Send, data).await
    }

    pub async fn retrieve(&self, url: &str) -> Result<String, HikvisionError> {
        request(url, (self.user.clone(), self.pass.clone()), AuthType::Digest, RequestType::Send, None).await
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
