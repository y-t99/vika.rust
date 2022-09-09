#[derive(Debug)]
pub struct Config {
    pub protocol: String,

    pub host: String,

    pub version: String,

    pub token: String,
}

impl Config {
    pub fn clone(&self) -> Config {
        Config {
            protocol: self.protocol.clone(),
            host: self.host.clone(),
            version: self.version.clone(),
            token: self.token.clone(),
        }
    }
}
