#[derive(PartialEq, Debug)]
pub enum AuthType {
    No,
    Digest,
    Basic,
    Bearer,
}

impl Default for AuthType {
    fn default() -> Self {
        AuthType::No
    }
}

#[derive(Debug)]
pub enum RequestType {
    Send,
    Recieve,
}
